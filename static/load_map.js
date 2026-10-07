let map;
let mapElement;
let mapModulePromise;
let shouldLoadMap = isMapRoute(window.location.href);
let pendingPath;
let navigationVersion = 0;
let loadingElement;

function isMapRoute(href) {
    const url = new URL(href, window.location.href);
    return url.origin === window.location.origin && /^\/fahrtenbuch\/[^/]+\/?$/.test(url.pathname);
}

const getMapElement = () => document.querySelector('[id^="mapContainer"] > [id^="map"]');

const destroyMap = () => {
    if (map) {
        map.remove();
        map = undefined;
    }

    mapElement = undefined;
    loadingElement = undefined;
};

const loadMap = async (element, version) => {
    loadingElement = element;
    mapModulePromise ??= import('/map/maplibre-gl.mjs');
    const maplibregl = await mapModulePromise;

    if (version !== navigationVersion || !shouldLoadMap || !element.isConnected || element !== getMapElement()) {
        if (loadingElement === element) {
            loadingElement = undefined;
        }
        return;
    }

    mapElement = element;
    loadingElement = undefined;
    map = new maplibregl.Map({
        container: element,
        style: 'https://demotiles.maplibre.org/globe.json',
        center: [0, 0],
        zoom: 1,
        attributionControl: false
    });
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