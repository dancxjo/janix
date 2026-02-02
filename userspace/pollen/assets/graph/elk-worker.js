// ELK.js Layout Worker
// =====================
// Web Worker that runs ELK layout algorithm asynchronously
// to prevent UI freezes on large graphs.

// Load ELK.js from CDN
importScripts('https://cdn.jsdelivr.net/npm/elkjs@0.9.3/lib/elk.bundled.min.js');

// Create ELK instance
const elk = new ELK();

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
self.postMessage({ type: 'ready' });
