// ELK.js Layout Worker
// =====================
// Web Worker that runs ELK layout algorithm asynchronously
// to prevent UI freezes on large graphs.

// Load ELK.js from CDN (wrapped in try/catch for stability)
const ELK_BUNDLE_URLS = [
    'https://cdn.jsdelivr.net/npm/elkjs@0.9.3/lib/elk.bundled.min.js',
];

let elk = null;
let elkLoadError = null;

function tryLoadElk(url) {
    try {
        importScripts(url);
        if (typeof ELK === 'undefined') {
            throw new Error(`ELK not found after loading ${url}`);
        }
        elk = new ELK();
        return true;
    } catch (err) {
        elkLoadError = err instanceof Error ? err : new Error(String(err));
        return false;
    }
}

for (const url of ELK_BUNDLE_URLS) {
    if (tryLoadElk(url)) {
        elkLoadError = null;
        break;
    }
}

if (!elk && !elkLoadError) {
    elkLoadError = new Error('ELK unavailable');
}

// Default layout options tuned for readable directed graphs
const DEFAULT_OPTIONS = {
    'elk.algorithm': 'layered',
    'elk.direction': 'RIGHT',
    'elk.spacing.nodeNode': '40',
    'elk.layered.spacing.nodeNodeBetweenLayers': '60',
    'elk.edgeRouting': 'ORTHOGONAL',
    'elk.layered.nodePlacement.strategy': 'NETWORK_SIMPLEX',
    'elk.layered.crossingMinimization.strategy': 'LAYER_SWEEP',
    'elk.nodeLabels.placement': '[H_CENTER, V_CENTER]',
    // Improve label readability
    'elk.spacing.edgeNode': '20',
    'elk.spacing.edgeEdge': '15',
};

/**
 * Handle incoming layout requests
 * 
 * Message format:
 * {
 *   requestId: string,
 *   graph: ElkNode,      // ELK JSON format with id, children, edges
 *   options?: object     // Layout options overrides
 * }
 */
self.onmessage = async function (event) {
    const { requestId, graph, options } = event.data;
    const startTime = performance.now();

    try {
        if (!requestId) {
            return;
        }

        if (!elk) {
            throw new Error(elkLoadError ? elkLoadError.message : 'ELK unavailable');
        }

        // Validate input
        if (!graph || typeof graph !== 'object') {
            throw new Error('Invalid graph: must be an object');
        }
        if (!graph.id) {
            throw new Error('Invalid graph: missing root id');
        }

        // Merge options with defaults
        const layoutOptions = { ...DEFAULT_OPTIONS, ...options };

        // Apply layout options to graph
        const graphWithOptions = {
            ...graph,
            layoutOptions,
        };

        // Run ELK layout
        const laidOut = await elk.layout(graphWithOptions);

        const endTime = performance.now();
        const ms = Math.round(endTime - startTime);

        // Send back the laid out graph
        self.postMessage({
            requestId,
            laidOut,
            ms,
        });

    } catch (error) {
        // Send error response
        self.postMessage({
            requestId,
            error: error.message || 'Unknown layout error',
        });
    }
};

// Signal that worker is ready
self.postMessage({
    type: 'ready',
    error: elk ? null : (elkLoadError ? elkLoadError.message : 'ELK unavailable'),
});
