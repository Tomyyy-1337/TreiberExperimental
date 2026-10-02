class MediaMTXWebRTCReader {
    static fetchTimeout = 6000;

    constructor(config) {
        this.config = config;
        this.state = 'running';
        this.peerConnection = null;
        this.sessionUrl = null;
        this.offerData = null;
        this.queuedCandidates = [];
        this.controllers = new Set();
        this.start();
    }

    close() {
        this.state = 'closed';
        for (const controller of this.controllers) controller.abort();
        this.controllers.clear();
        if (this.peerConnection) this.peerConnection.close();
        this.peerConnection = null;
        if (this.sessionUrl) {
            fetch(this.sessionUrl, { method: 'DELETE' }).catch(() => {});
            this.sessionUrl = null;
        }
        this.config.video.srcObject = null;
    }

    async fetchWithTimeout(input, init) {
        const controller = new AbortController();
        const timeout = window.setTimeout(() => controller.abort(new Error('request timeout')), MediaMTXWebRTCReader.fetchTimeout);
        this.controllers.add(controller);
        try {
            return await fetch(input, { ...init, signal: controller.signal });
        } finally {
            clearTimeout(timeout);
            this.controllers.delete(controller);
        }
    }

    authHeader() {
        const { user, pass, token } = this.config;
        if (user) return { Authorization: `Basic ${btoa(`${user}:${pass || ''}`)}` };
        if (token) return { Authorization: `Bearer ${token}` };
        return {};
    }

    async start() {
        try {
            const iceServers = await this.requestIceServers();
            await this.setupPeerConnection(iceServers);
            const answer = await this.sendOffer(await this.peerConnection.localDescription.sdp);
            await this.peerConnection.setRemoteDescription({ type: 'answer', sdp: answer });
            for (const candidate of this.queuedCandidates) this.sendCandidates([candidate]);
            this.queuedCandidates = [];
            this.config.onState?.('connected');
        } catch (error) {
            this.handleError(error);
        }
    }

    async requestIceServers() {
        const response = await this.fetchWithTimeout(this.config.url, {
            method: 'OPTIONS',
            headers: this.authHeader(),
        });
        if (!response.ok) throw new Error(`ICE server request failed (${response.status})`);
        return this.parseIceServers(response.headers.get('Link'));
    }

    parseIceServers(linkHeader) {
        if (!linkHeader) return [];
        return linkHeader.split(', ').flatMap((link) => {
            const match = link.match(/^<(.+?)>; rel="ice-server"(?:; username="(.*?)"; credential="(.*?)"; credential-type="password")?/i);
            if (!match) return [];
            const server = { urls: [match[1]] };
            if (match[2] !== undefined) {
                server.username = JSON.parse(`"${match[2]}"`);
                server.credential = JSON.parse(`"${match[3]}"`);
            }
            return [server];
        });
    }

    async setupPeerConnection(iceServers) {
        this.peerConnection = new RTCPeerConnection({ iceServers });
        this.peerConnection.addTransceiver('video', { direction: 'recvonly' });
        this.peerConnection.createDataChannel('');
        this.peerConnection.ontrack = (event) => {
            if (event.track.kind === 'video') {
                this.config.video.srcObject = event.streams[0] || new MediaStream([event.track]);
                this.config.video.play().catch(() => {});
            }
            this.config.onTrack?.(event);
        };
        this.peerConnection.ondatachannel = (event) => this.config.onDataChannel?.(event);
        this.peerConnection.onicecandidate = (event) => {
            if (!event.candidate) return;
            if (this.sessionUrl) this.sendCandidates([event.candidate]);
            else this.queuedCandidates.push(event.candidate);
        };
        const offer = await this.peerConnection.createOffer();
        this.offerData = this.parseOffer(offer.sdp);
        await this.peerConnection.setLocalDescription(offer);
    }

    parseOffer(sdp) {
        const result = { iceUfrag: '', icePwd: '', medias: [] };
        for (const line of sdp.split('\r\n')) {
            if (line.startsWith('m=')) result.medias.push(line.slice(2));
            if (!result.iceUfrag && line.startsWith('a=ice-ufrag:')) result.iceUfrag = line.slice(12);
            if (!result.icePwd && line.startsWith('a=ice-pwd:')) result.icePwd = line.slice(10);
        }
        return result;
    }

    async sendOffer(offer) {
        const response = await this.fetchWithTimeout(this.config.url, {
            method: 'POST',
            headers: { ...this.authHeader(), 'Content-Type': 'application/sdp' },
            body: offer,
        });
        if (response.status === 404) throw new Error('stream not found');
        if (!response.ok) throw new Error(`offer failed (${response.status})`);
        const location = response.headers.get('Location');
        if (!location) throw new Error('MediaMTX did not return a session location');
        this.sessionUrl = new URL(location, this.config.url).toString();
        return response.text();
    }

    makeFragment(candidates) {
        const byMedia = new Map();
        for (const candidate of candidates) {
            const index = candidate.sdpMLineIndex ?? 0;
            if (!byMedia.has(index)) byMedia.set(index, []);
            byMedia.get(index).push(candidate);
        }
        let fragment = `a=ice-ufrag:${this.offerData.iceUfrag}\r\na=ice-pwd:${this.offerData.icePwd}\r\n`;
        this.offerData.medias.forEach((media, index) => {
            if (!byMedia.has(index)) return;
            fragment += `m=${media}\r\na=mid:${index}\r\n`;
            for (const candidate of byMedia.get(index)) fragment += `a=${candidate.candidate}\r\n`;
        });
        return fragment;
    }

    async sendCandidates(candidates) {
        if (!this.sessionUrl || !this.offerData) return;
        try {
            const response = await this.fetchWithTimeout(this.sessionUrl, {
                method: 'PATCH',
                headers: { 'Content-Type': 'application/trickle-ice-sdpfrag', 'If-Match': '*' },
                body: this.makeFragment(candidates),
            });
            if (!response.ok) throw new Error(`ICE candidate request failed (${response.status})`);
        } catch (error) {
            this.handleError(error);
        }
    }

    handleError(error) {
        if (this.state === 'closed' || this.state === 'failed') return;
        this.state = 'failed';
        this.config.onState?.('failed');
        this.config.onError?.(error instanceof Error ? error.message : String(error));
        this.close();
        this.state = 'failed';
    }
}

const video = document.querySelector('[data-live-video]');
const container = document.querySelector('[data-live-config]');

if (video && container) {
    const reader = new MediaMTXWebRTCReader({
        url: container.dataset.url,
        user: container.dataset.user || '',
        pass: container.dataset.pass || '',
        token: container.dataset.token || '',
        video,
    });
    window.addEventListener('pagehide', () => reader.close(), { once: true });
}
