let map;
let mapElement;
let mapModulePromise;
let pmtilesModulePromise;
let availableMapsPromise;
let shouldLoadMap = isMapRoute(window.location.href);
let pendingPath;
let navigationVersion = 0;
let loadingElement;
let themeObserver;

const PMTILES_MAGIC_NUMBER = 19792;
let currentPmtilesFiles = [];
let pmtilesProtocolRegistered = false;

const loadPmtilesModule = () => pmtilesModulePromise ??= new Promise((resolve, reject) => {
    if (globalThis.pmtiles) {
        resolve(globalThis.pmtiles);
        return;
    }

    const script = document.createElement('script');
    script.src = '/static/pmtiles.js';
    script.onload = () => resolve(globalThis.pmtiles);
    script.onerror = reject;
    document.head.append(script);
});

const getMapPalette = (theme) => theme === 'dark' ? {
    background: '#121518', water: '#2f4f63', waterOutline: '#4f748a', roadMotorway: '#8b9aa6', roadTrunk: '#7f8d97', roadPrimary: '#73808b', roadSecondary: '#6a757f', roadOther: '#5f6973', textCapital: '#f0f3f6', textTown: '#dbe1e7', textVillage: '#c8d0d8', textHalo: '#0d1013',
    landuse: { residential: '#22262b', suburb: '#1f2429', commercial: '#2a2f35', industrial: '#30343a', hospital: '#3a3030', military: '#2e3238', quarry: '#3a3f45', themePark: '#294036', cemetery: '#314238', track: '#3a332b', fallback: '#242a30' }
} : {
    background: '#e0e0e0', water: '#5da8d4', waterOutline: '#3a7fa0', roadMotorway: '#888888', roadTrunk: '#999999', roadPrimary: '#a0a0a0', roadSecondary: '#ffffff', roadOther: '#ffffff', textCapital: '#000000', textTown: '#0a0a0a', textVillage: '#1a1a1a', textHalo: '#e8e8e8',
    landuse: { residential: '#d9d6cf', suburb: '#d3d0c9', commercial: '#ccc9c2', industrial: '#b8b2ab', hospital: '#e8c8b8', military: '#a8a8a8', quarry: '#959595', themePark: '#6ba86b', cemetery: '#a8c8a8', track: '#c8b89c', fallback: '#c8c8c8' }
};

const buildLanduseColorExpression = (palette) => ['match', ['get', 'class'], 'residential', palette.landuse.residential, 'suburb', palette.landuse.suburb, 'neighbourhood', palette.landuse.suburb, 'commercial', palette.landuse.commercial, 'retail', palette.landuse.commercial, 'industrial', palette.landuse.industrial, 'hospital', palette.landuse.hospital, 'military', palette.landuse.military, 'quarry', palette.landuse.quarry, 'theme_park', palette.landuse.themePark, 'cemetery', palette.landuse.cemetery, 'track', palette.landuse.track, palette.landuse.fallback];
const pmtilesSourceName = (filename) => filename.replace(/\.pmtiles$/, '');

const discoverPmtilesBaseUrl = async (element) => {
    return availableMapsPromise ??= Promise.resolve().then(() => {
        const validFiles = JSON.parse(element.getAttribute('available_maps') ?? '[]');
        return {
            baseUrl: `${window.location.origin}/maps`,
            validFiles
        };
    });
};

const buildSourceLayers = (sourceName, palette) => [
    { id: `${sourceName}-landuse`, type: 'fill', source: sourceName, 'source-layer': 'landuse', paint: { 'fill-color': buildLanduseColorExpression(palette), 'fill-opacity': 1 } },
    { id: `${sourceName}-water`, type: 'fill', source: sourceName, 'source-layer': 'water', paint: { 'fill-color': palette.water, 'fill-opacity': 0.95 } },
    { id: `${sourceName}-water-outline`, type: 'line', source: sourceName, 'source-layer': 'water', paint: { 'line-color': palette.waterOutline, 'line-width': ['interpolate', ['linear'], ['zoom'], 4, 0.5, 8, 0.5, 14, 1.5] } },
    ...[['motorway', palette.roadMotorway, 5, 0.5, 10, 2, 14, 4, 18, 10], ['trunk', palette.roadTrunk, 5, 0.4, 10, 1.5, 14, 3, 18, 8], ['primary', palette.roadPrimary, 8, 0.5, 12, 1.5, 16, 3], ['secondary', palette.roadSecondary, 10, 0.5, 14, 1.5, 18, 4]].map(([roadClass, color, ...widthStops]) => ({ id: `${sourceName}-${roadClass}`, type: 'line', source: sourceName, 'source-layer': 'transportation', filter: ['==', ['get', 'class'], roadClass], layout: { 'line-join': 'round', 'line-cap': 'round' }, paint: { 'line-color': color, 'line-width': ['interpolate', ['linear'], ['zoom'], ...widthStops] } })),
    { id: `${sourceName}-roads-other`, type: 'line', source: sourceName, 'source-layer': 'transportation', filter: ['all', ...['motorway', 'trunk', 'primary', 'secondary'].map((roadClass) => ['!=', ['get', 'class'], roadClass])], paint: { 'line-color': palette.roadOther, 'line-width': ['interpolate', ['linear'], ['zoom'], 12, 0.3, 16, 0.8, 18, 2], 'line-opacity': ['interpolate', ['linear'], ['zoom'], 10, 0.3, 14, 0.7] } },
    ...[['city', 'capital', palette.textCapital, 'Open Sans Bold', 5, 12, 10, 16, 15, 22, 2.5], ['town', 'town', palette.textTown, 'Open Sans SemiBold', 8, 10, 12, 14, 16, 18, 2], ['village', 'village', palette.textVillage, 'Open Sans Regular', 10, 9, 14, 12, 18, 14, 1.5]].map(([placeClass, placeName, color, font, ...textStops]) => ({ id: `${sourceName}-places-${placeName}`, type: 'symbol', source: sourceName, 'source-layer': 'place', filter: ['==', ['get', 'class'], placeClass], layout: { 'text-field': ['get', 'name'], 'text-font': [font, 'Arial Unicode MS Bold'], 'text-size': ['interpolate', ['linear'], ['zoom'], ...textStops.slice(0, -1)], 'text-offset': [0, 0.3], 'text-anchor': 'center', 'text-max-width': placeName === 'capital' ? 10 : placeName === 'town' ? 8 : 7 }, paint: { 'text-color': color, 'text-halo-color': palette.textHalo, 'text-halo-width': textStops.at(-1) } }))
];

const buildMapStyle = (baseUrl, files, theme) => {
    const palette = getMapPalette(theme);
    return { version: 8, sources: Object.fromEntries([...files.map((filename) => [pmtilesSourceName(filename), { type: 'vector', url: `pmtiles://${baseUrl}/${filename}` }]), ['waypoint-path', { type: 'geojson', data: { type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} } }]]), layers: [{ id: 'background', type: 'background', paint: { 'background-color': palette.background } }, ...files.flatMap((filename) => buildSourceLayers(pmtilesSourceName(filename), palette)), { id: 'waypoint-path', type: 'line', source: 'waypoint-path', paint: { 'line-color': '#e53935', 'line-width': 5, 'line-opacity': 0.9 } }] };
};

const applyThemeToMap = (theme) => {
    if (!map) return;
    const palette = getMapPalette(theme);
    const setPaint = (layerId, property, value) => { if (map.getLayer(layerId)) map.setPaintProperty(layerId, property, value); };
    setPaint('background', 'background-color', palette.background);
    for (const filename of currentPmtilesFiles) {
        const sourceName = pmtilesSourceName(filename);
        setPaint(`${sourceName}-landuse`, 'fill-color', buildLanduseColorExpression(palette));
        setPaint(`${sourceName}-water`, 'fill-color', palette.water);
        setPaint(`${sourceName}-water-outline`, 'line-color', palette.waterOutline);
        for (const roadClass of ['motorway', 'trunk', 'primary', 'secondary']) setPaint(`${sourceName}-${roadClass}`, 'line-color', palette[`road${roadClass[0].toUpperCase()}${roadClass.slice(1)}`]);
        setPaint(`${sourceName}-roads-other`, 'line-color', palette.roadOther);
        for (const place of ['capital', 'town', 'village']) { setPaint(`${sourceName}-places-${place}`, 'text-color', palette[`text${place[0].toUpperCase()}${place.slice(1)}`]); setPaint(`${sourceName}-places-${place}`, 'text-halo-color', palette.textHalo); }
    }
};

function isMapRoute(href) {
    const url = new URL(href, window.location.href);
    return url.origin === window.location.origin && /^\/fahrtenbuch\/[^/]+\/?$/.test(url.pathname);
}

const getMapElement = () => document.querySelector('[id^="map"]');

const loadWaypointPath = async () => {
    if (!mapElement || !map) return;

    try {
        const positions = JSON.parse(mapElement.getAttribute('gps_data') ?? '[]');
        const coordinates = positions
            .filter((position) => Number.isFinite(position.longitude) && Number.isFinite(position.latitude))
            .map((position) => [position.longitude, position.latitude]);
        const source = map.getSource('waypoint-path');
        if (!source) return;

        source.setData({
            type: 'Feature',
            geometry: { type: 'LineString', coordinates },
            properties: {}
        });

        if (coordinates.length > 0) {
            const longitudes = coordinates.map(([longitude]) => longitude);
            const latitudes = coordinates.map(([, latitude]) => latitude);
            map.fitBounds([[Math.min(...longitudes), Math.min(...latitudes)], [Math.max(...longitudes), Math.max(...latitudes)]], {
                padding: 40,
                maxZoom: 16,
                duration: 0
            });
        }
    } catch {
        // The map remains usable when an entry has no GPS history.
    }
};

const destroyMap = () => {
    themeObserver?.disconnect();
    themeObserver = undefined;
    if (map) {
        map.remove();
        map = undefined;
    }

    mapElement = undefined;
    loadingElement = undefined;
    currentPmtilesFiles = [];
};

const loadMap = async (element, version) => {
    loadingElement = element;
    mapModulePromise ??= import('/static/maplibre-gl.mjs');
    const [maplibregl] = await Promise.all([mapModulePromise, loadPmtilesModule()]);
    const { baseUrl, validFiles } = await discoverPmtilesBaseUrl(element);

    if (version !== navigationVersion || !shouldLoadMap || !element.isConnected || element !== getMapElement()) {
        if (loadingElement === element) {
            loadingElement = undefined;
        }
        return;
    }

    if (!pmtilesProtocolRegistered) {
        const protocol = new globalThis.pmtiles.Protocol();
        maplibregl.addProtocol('pmtiles', protocol.tile.bind(protocol));
        pmtilesProtocolRegistered = true;
    }

    currentPmtilesFiles = validFiles;
    mapElement = element;
    loadingElement = undefined;
    map = new maplibregl.Map({
        container: element,
        style: {
            ...buildMapStyle(baseUrl, validFiles, document.documentElement.className),
        },
        center: [10, 51],
        zoom: 4,
        attributionControl: false
    });

    themeObserver = new MutationObserver(() => applyThemeToMap(document.documentElement.className));
    themeObserver.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] });
    map.once('load', loadWaypointPath);
};

const syncMap = () => {
    const element = getMapElement();
    const onMapRoute = isMapRoute(window.location.href);
    const onPendingRoute = !pendingPath || window.location.pathname === pendingPath;

    if (!shouldLoadMap || !onMapRoute || (mapElement && mapElement !== element)) {
        destroyMap();
    }

    if (shouldLoadMap && onMapRoute && onPendingRoute && !map && !loadingElement && element) {
        loadMap(element, navigationVersion);
    }
};

const syncAfterNavigation = () => {
    if (!pendingPath || window.location.pathname === pendingPath) {
        pendingPath = undefined;
        syncMap();
        return;
    }

    requestAnimationFrame(syncAfterNavigation);
};

window.addEventListener('click', (event) => {
    const link = event.target.closest?.('a');
    if (!link) {
        return;
    }

    navigationVersion += 1;
    destroyMap();

    const url = new URL(link.href, window.location.href);
    shouldLoadMap = isMapRoute(url.href);
    pendingPath = url.pathname;

    requestAnimationFrame(syncAfterNavigation);
});

const startMapObserver = () => {
    new MutationObserver(syncMap).observe(document.body, {
        childList: true,
        subtree: true,
        attributes: true,
        attributeFilter: ['id']
    });

    syncMap();
};

if (document.body) {
    startMapObserver();
} else {
    document.addEventListener('DOMContentLoaded', startMapObserver, { once: true });
}