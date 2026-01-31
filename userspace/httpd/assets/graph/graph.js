// Thing-OS Graph Viewer
// ======================
// Interactive Cytoscape.js graph visualization with ELK.js layout

// =============================================================================
// Configuration
// =============================================================================

const CONFIG = {
    DEFAULT_ROOT: '',   // Empty = auto-discover from system nodes (like Photosynthesis)
    DEFAULT_DEPTH: 3,   // Increased for auto-discovery
    MAX_NODES: 500,
    AUTOSAVE_DEBOUNCE_MS: 750,
    LAYOUT_SPACE: 'graph_ui_v1',
    // ELK layout defaults
    DEFAULT_SPACING: 40,
    DEFAULT_LAYER_SPACING: 60,
};

// =============================================================================
// DOM Helpers
// =============================================================================

const $ = (id) => document.getElementById(id);

// =============================================================================
// Label Measurement
// =============================================================================

const labelMeasurer = {
    svg: null,
    text: null,
    cache: new Map(),

    init() {
        // Create offscreen SVG for text measurement
        this.svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
        this.svg.style.position = 'absolute';
        this.svg.style.visibility = 'hidden';
        this.svg.style.pointerEvents = 'none';
        this.svg.setAttribute('width', '1');
        this.svg.setAttribute('height', '1');

        this.text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
        this.text.style.fontFamily = '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif';
        this.text.style.fontSize = '13px';
        this.text.style.fontWeight = '500';

        this.svg.appendChild(this.text);
        document.body.appendChild(this.svg);
    },

    measure(label, secondaryLabel = '') {
        const cacheKey = `${label}|${secondaryLabel}`;
        if (this.cache.has(cacheKey)) {
            return this.cache.get(cacheKey);
        }

        // Measure primary label
        this.text.textContent = label || '';
        let bbox = this.text.getBBox();
        let width = bbox.width;
        let height = bbox.height;

        // Measure secondary label if present
        if (secondaryLabel) {
            this.text.style.fontSize = '11px';
            this.text.style.fontWeight = '400';
            this.text.textContent = secondaryLabel;
            const secBbox = this.text.getBBox();
            width = Math.max(width, secBbox.width);
            height += secBbox.height + 4; // Line gap
            // Reset to primary style
            this.text.style.fontSize = '13px';
            this.text.style.fontWeight = '500';
        }

        // Add padding: 24px horizontal, 16px vertical minimum
        const result = {
            width: Math.max(80, Math.ceil(width) + 24),
            height: Math.max(50, Math.ceil(height) + 16),
        };

        this.cache.set(cacheKey, result);
        return result;
    },

    clearCache() {
        this.cache.clear();
    },
};

// =============================================================================
// ELK Worker
// =============================================================================

const elkLayout = {
    worker: null,
    pending: new Map(), // requestId -> { resolve, reject }
    requestCounter: 0,
    ready: false,

    init() {
        this.worker = new Worker('/elk-worker.js');

        this.worker.onmessage = (event) => {
            const data = event.data;

            // Handle ready signal
            if (data.type === 'ready') {
                this.ready = true;
                console.log('[ELK] Worker ready');
                return;
            }

            const { requestId, laidOut, error, ms } = data;
            const pending = this.pending.get(requestId);

            if (!pending) {
                console.warn('[ELK] Received response for unknown request:', requestId);
                return;
            }

            this.pending.delete(requestId);

            if (error) {
                pending.reject(new Error(error));
            } else {
                pending.resolve({ laidOut, ms });
            }
        };

        this.worker.onerror = (err) => {
            console.error('[ELK] Worker error:', err);
            // Reject all pending requests
            for (const [id, pending] of this.pending) {
                pending.reject(new Error('Worker crashed'));
            }
            this.pending.clear();
        };
    },

    layout(graph, options = {}) {
        return new Promise((resolve, reject) => {
            const requestId = `elk-${++this.requestCounter}`;

            this.pending.set(requestId, { resolve, reject });

            this.worker.postMessage({
                requestId,
                graph,
                options,
            });
        });
    },

    // Cancel pending request (for debouncing)
    cancel(requestId) {
        this.pending.delete(requestId);
    },
};

// =============================================================================
// API Layer
// =============================================================================

const api = {
    async getSubgraph(root, depth = CONFIG.DEFAULT_DEPTH, maxNodes = CONFIG.MAX_NODES) {
        const params = new URLSearchParams({
            depth: depth.toString(),
            max_nodes: maxNodes.toString(),
        });
        // Only add root if specified (empty = auto-discover)
        if (root && root.trim() !== '') {
            params.set('root', root);
        }
        const r = await fetch(`/api/v1/subgraph?${params}`);
        if (!r.ok) {
            const text = await r.text();
            throw new Error(`${r.status} ${r.statusText}: ${text}`);
        }
        return r.json();
    },

    async saveLayout(nodes) {
        const body = {
            space: CONFIG.LAYOUT_SPACE,
            nodes: nodes.map(n => ({
                id: n.id,
                x: n.x,
                y: n.y,
            })),
        };
        const r = await fetch('/api/v1/layout', {
            method: 'PATCH',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(body),
        });
        if (!r.ok) {
            const text = await r.text();
            throw new Error(`${r.status} ${r.statusText}: ${text}`);
        }
        return r.json();
    },
};

// =============================================================================
// ELK Graph Builder
// =============================================================================

function buildElkGraph(graphData) {
    const children = [];
    const edges = [];

    for (const node of graphData.nodes) {
        const label = node.label || node.id.toString().slice(-8);
        const kindName = node.kind_name || 'unknown';

        // Measure label size
        const size = labelMeasurer.measure(label, kindName);

        children.push({
            id: node.id.toString(),
            width: size.width,
            height: size.height,
            labels: [{ text: label }],
            // Store original data for rendering
            _data: {
                label,
                kindName,
                kind: node.kind || 0,
            },
        });
    }

    for (const edge of graphData.edges) {
        edges.push({
            id: edge.id,
            sources: [edge.from.toString()],
            targets: [edge.to.toString()],
            // Edge labels are optional
            labels: edge.rel_name ? [{ text: edge.rel_name }] : [],
        });
    }

    return {
        id: 'root',
        children,
        edges,
    };
}

// =============================================================================
// State
// =============================================================================

const state = {
    cy: null,
    selectedNode: null,
    autoSave: false,
    saveTimeout: null,
    movedNodes: new Set(),
    graphData: null,
    lastLayoutMs: 0,
};

// =============================================================================
// Cytoscape Initialization
// =============================================================================

function initCytoscape() {
    state.cy = cytoscape({
        container: $('cy'),
        elements: [],
        style: [
            // Photosynthesis-style nodes: rectangular cards with dynamic sizing
            {
                selector: 'node',
                style: {
                    // Width/height set per-node from ELK results
                    'width': 'data(width)',
                    'height': 'data(height)',
                    'shape': 'roundrectangle',
                    // Light glassmorphism fill (translucent white like native app)
                    'background-color': 'rgba(255, 255, 255, 0.53)',
                    'background-opacity': 1,
                    // Light border matching Photosynthesis TILE_BORDER_COLOR #E0E0E8
                    'border-width': 2,
                    'border-color': '#E0E0E8',
                    'border-opacity': 1,
                    // Compound label: name + kindName (like Photosynthesis)
                    'label': function (ele) {
                        const name = ele.data('label') || '';
                        const kind = ele.data('kindName') || '';
                        return name + '\n' + kind;
                    },
                    'color': '#383838',  // TYPE_TEXT_COLOR
                    'font-size': '13px',
                    'font-family': '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
                    'font-weight': 500,
                    'text-valign': 'center',
                    'text-halign': 'center',
                    'text-wrap': 'wrap',
                    'text-max-width': '110px',
                    // Enable line height for multi-line labels
                    'line-height': 1.4,
                },
            },
            {
                selector: 'node:selected',
                style: {
                    'border-color': '#4ea8de',
                    'border-width': 3,
                    'background-color': 'rgba(255, 255, 255, 0.7)',
                },
            },
            {
                selector: 'node[?isRoot]',
                style: {
                    'background-color': 'rgba(78, 168, 222, 0.2)',
                    'border-color': '#4ea8de',
                },
            },
            // Edges: styled for orthogonal routing
            {
                selector: 'edge',
                style: {
                    'width': 2,
                    'line-color': '#888888',
                    'target-arrow-color': '#888888',
                    'target-arrow-shape': 'triangle',
                    'curve-style': 'taxi',  // Orthogonal-style edges
                    'taxi-direction': 'rightward',
                    'arrow-scale': 1.0,
                    'opacity': 0.8,
                },
            },
            {
                selector: 'edge:selected',
                style: {
                    'line-color': '#4ea8de',
                    'target-arrow-color': '#4ea8de',
                    'width': 3,
                    'opacity': 1,
                },
            },
            // Edge labels
            {
                selector: 'edge[label]',
                style: {
                    'label': 'data(label)',
                    'font-size': '9px',
                    'font-family': '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
                    'color': '#666666',
                    'text-rotation': 'autorotate',
                    'text-margin-y': -8,
                },
            },
        ],
        layout: { name: 'preset' },
        wheelSensitivity: 0.3,
        minZoom: 0.1,
        maxZoom: 5,
    });

    // Event handlers
    state.cy.on('tap', 'node', (evt) => {
        selectNode(evt.target);
    });

    state.cy.on('tap', (evt) => {
        if (evt.target === state.cy) {
            clearSelection();
        }
    });

    state.cy.on('dbltap', 'node', (evt) => {
        const id = evt.target.id();
        window.location.href = `/#thing=${encodeURIComponent(id)}`;
    });

    state.cy.on('dragfree', 'node', (evt) => {
        const node = evt.target;
        state.movedNodes.add(node.id());

        // Update inspector if this node is selected
        if (state.selectedNode && state.selectedNode.id() === node.id()) {
            updateInspectorPosition(node);
        }

        // Auto-save if enabled
        if (state.autoSave) {
            scheduleSave();
        }
    });
}

// =============================================================================
// Graph Loading with ELK Layout
// =============================================================================

async function loadGraph(root, depth) {
    setStatus('Loading...', '');

    try {
        const data = await api.getSubgraph(root, depth);
        state.graphData = data;
        state.movedNodes.clear();

        // Check for existing layout positions
        const nodesWithPos = data.nodes.filter(n => n.x !== undefined && n.y !== undefined);
        const usePreset = nodesWithPos.length > data.nodes.length * 0.5;

        if (usePreset) {
            // Use existing positions from graph
            renderWithPresetLayout(data, root);
        } else {
            // Use ELK for layout
            await runElkLayout(data, root);
        }

    } catch (err) {
        setStatus(`Error: ${err.message}`, '');
        console.error('Load failed:', err);
    }
}

async function runElkLayout(data, root) {
    setStatus('Computing layout...', '');

    try {
        // Build ELK graph with measured node sizes
        const elkGraph = buildElkGraph(data);

        // Get layout options from URL
        const params = new URLSearchParams(window.location.search);
        const spacing = parseInt(params.get('spacing')) || CONFIG.DEFAULT_SPACING;
        const layerSpacing = parseInt(params.get('layer_spacing')) || CONFIG.DEFAULT_LAYER_SPACING;

        const options = {
            'elk.spacing.nodeNode': spacing.toString(),
            'elk.layered.spacing.nodeNodeBetweenLayers': layerSpacing.toString(),
        };

        // Run ELK layout in worker
        const { laidOut, ms } = await elkLayout.layout(elkGraph, options);
        state.lastLayoutMs = ms;

        // Apply layout to Cytoscape
        applyElkLayout(laidOut, data, root);

        // Update stats
        updateStats(data, ms);
        setStatus('Ready', '');

    } catch (err) {
        console.error('ELK layout failed:', err);
        setStatus(`Layout error: ${err.message}`, '');
        // Fallback to cose layout
        renderWithCoseLayout(data, root);
    }
}

function applyElkLayout(laidOut, data, root) {
    const elements = [];

    // Build node map from ELK results
    const nodePositions = new Map();
    for (const child of laidOut.children || []) {
        nodePositions.set(child.id, {
            x: child.x + child.width / 2,  // ELK uses top-left, Cytoscape uses center
            y: child.y + child.height / 2,
            width: child.width,
            height: child.height,
        });
    }

    // Create Cytoscape elements with ELK positions
    for (const n of data.nodes) {
        const id = n.id.toString();
        const pos = nodePositions.get(id);
        const primaryLabel = n.label || n.id.toString().slice(-8);
        const kindName = n.kind_name || 'unknown';

        elements.push({
            data: {
                id,
                label: primaryLabel,
                kindName,
                kind: n.kind || 0,
                width: pos ? pos.width : 100,
                height: pos ? pos.height : 60,
                isRoot: root && root.trim() !== '' && id === root.toString(),
            },
            position: pos ? { x: pos.x, y: pos.y } : { x: 0, y: 0 },
        });
    }

    for (const e of data.edges) {
        elements.push({
            data: {
                id: e.id,
                source: e.from.toString(),
                target: e.to.toString(),
                rel: e.rel || 0,
                label: e.rel_name || 'link',
            },
        });
    }

    // Update Cytoscape
    state.cy.elements().remove();
    state.cy.add(elements);
    state.cy.fit(undefined, 50);
}

function renderWithPresetLayout(data, root) {
    const elements = [];

    for (const n of data.nodes) {
        const primaryLabel = n.label || n.id.toString().slice(-8);
        const kindName = n.kind_name || 'unknown';
        const size = labelMeasurer.measure(primaryLabel, kindName);

        elements.push({
            data: {
                id: n.id.toString(),
                label: primaryLabel,
                kindName,
                kind: n.kind || 0,
                width: size.width,
                height: size.height,
                isRoot: root && root.trim() !== '' && n.id.toString() === root.toString(),
            },
            position: { x: n.x, y: n.y },
        });
    }

    for (const e of data.edges) {
        elements.push({
            data: {
                id: e.id,
                source: e.from.toString(),
                target: e.to.toString(),
                rel: e.rel || 0,
                label: e.rel_name || 'link',
            },
        });
    }

    state.cy.elements().remove();
    state.cy.add(elements);
    state.cy.fit(undefined, 50);

    updateStats(data, 0);
    setStatus('Ready (preset)', '');
}

function renderWithCoseLayout(data, root) {
    const elements = [];

    for (const n of data.nodes) {
        const primaryLabel = n.label || n.id.toString().slice(-8);
        const kindName = n.kind_name || 'unknown';
        const size = labelMeasurer.measure(primaryLabel, kindName);

        elements.push({
            data: {
                id: n.id.toString(),
                label: primaryLabel,
                kindName,
                kind: n.kind || 0,
                width: size.width,
                height: size.height,
                isRoot: root && root.trim() !== '' && n.id.toString() === root.toString(),
            },
        });
    }

    for (const e of data.edges) {
        elements.push({
            data: {
                id: e.id,
                source: e.from.toString(),
                target: e.to.toString(),
                rel: e.rel || 0,
                label: e.rel_name || 'link',
            },
        });
    }

    state.cy.elements().remove();
    state.cy.add(elements);

    state.cy.layout({
        name: 'cose',
        animate: false,
        padding: 50,
        nodeRepulsion: 8000,
        idealEdgeLength: 100,
    }).run();

    updateStats(data, 0);
    setStatus('Ready (fallback)', '');
}

function updateStats(data, layoutMs) {
    const truncMsg = data.truncated ? ' (truncated)' : '';
    const layoutInfo = layoutMs > 0 ? ` | Layout: ${layoutMs}ms` : '';
    $('graphStats').textContent =
        `${data.nodes.length} nodes, ${data.edges.length} edges, depth ${data.stats?.depth || CONFIG.DEFAULT_DEPTH}${truncMsg}${layoutInfo}`;
}

// =============================================================================
// Re-layout Action
// =============================================================================

async function relayout() {
    if (!state.graphData) return;

    const root = $('rootInput').value.trim();
    await runElkLayout(state.graphData, root);
}

function fitToScreen() {
    if (state.cy) {
        state.cy.fit(undefined, 50);
    }
}

// =============================================================================
// Layout Persistence
// =============================================================================

function scheduleSave() {
    if (state.saveTimeout) {
        clearTimeout(state.saveTimeout);
    }
    state.saveTimeout = setTimeout(saveLayout, CONFIG.AUTOSAVE_DEBOUNCE_MS);
}

async function saveLayout() {
    if (!state.cy || state.cy.nodes().length === 0) return;

    setStatus('Saving...', 'saving');

    try {
        // Collect all node positions (or only moved nodes for auto-save)
        const nodesToSave = state.autoSave && state.movedNodes.size > 0
            ? state.cy.nodes().filter(n => state.movedNodes.has(n.id()))
            : state.cy.nodes();

        const positions = nodesToSave.map(n => ({
            id: n.id(),
            x: n.position('x'),
            y: n.position('y'),
        }));

        if (positions.length === 0) {
            setStatus('No changes to save', '');
            return;
        }

        const result = await api.saveLayout(positions);
        state.movedNodes.clear();
        setStatus(`Saved ${result.saved || positions.length} nodes`, 'saved');

        // Clear status after a moment
        setTimeout(() => {
            if ($('statusText').textContent.startsWith('Saved')) {
                setStatus('Ready', '');
            }
        }, 2000);

    } catch (err) {
        setStatus(`Save failed: ${err.message}`, '');
        console.error('Save failed:', err);
    }
}

// =============================================================================
// Inspector
// =============================================================================

function selectNode(node) {
    state.selectedNode = node;

    $('inspectorEmpty').style.display = 'none';
    $('inspectorContent').style.display = 'block';

    const id = node.id();
    const data = node.data();

    $('inspectorThingLink').textContent = id;
    $('inspectorThingLink').href = `/#thing=${encodeURIComponent(id)}`;
    $('inspectorKind').textContent = data.kindName || data.kind || '-';
    $('inspectorLabel').textContent = data.label || '-';
    updateInspectorPosition(node);
}

function updateInspectorPosition(node) {
    const pos = node.position();
    $('inspectorPos').textContent = `x: ${pos.x.toFixed(1)}, y: ${pos.y.toFixed(1)}`;
}

function clearSelection() {
    state.selectedNode = null;
    $('inspectorEmpty').style.display = 'block';
    $('inspectorContent').style.display = 'none';
}

// =============================================================================
// UI Helpers
// =============================================================================

function setStatus(text, className) {
    const el = $('statusText');
    el.textContent = text;
    el.className = 'status ' + className;
}

function getUrlParams() {
    const params = new URLSearchParams(window.location.search);
    return {
        root: params.get('root') ?? CONFIG.DEFAULT_ROOT,  // Allow empty string
        depth: parseInt(params.get('depth')) || CONFIG.DEFAULT_DEPTH,
    };
}

function updateUrl(root, depth) {
    const params = new URLSearchParams({ root, depth: depth.toString() });
    history.replaceState(null, '', `?${params}`);
}

// =============================================================================
// Event Bindings
// =============================================================================

function bindEvents() {
    // Load button
    $('loadBtn').addEventListener('click', () => {
        const root = $('rootInput').value.trim();  // Allow empty for auto-discover
        const depth = parseInt($('depthInput').value) || CONFIG.DEFAULT_DEPTH;
        updateUrl(root, depth);
        loadGraph(root, depth);
    });

    // Enter key in inputs
    $('rootInput').addEventListener('keydown', (e) => {
        if (e.key === 'Enter') $('loadBtn').click();
    });
    $('depthInput').addEventListener('keydown', (e) => {
        if (e.key === 'Enter') $('loadBtn').click();
    });

    // Save button
    $('saveBtn').addEventListener('click', saveLayout);

    // Re-layout button
    $('relayoutBtn').addEventListener('click', relayout);

    // Fit button
    $('fitBtn').addEventListener('click', fitToScreen);

    // Auto-save toggle
    $('autoSaveToggle').addEventListener('change', (e) => {
        state.autoSave = e.target.checked;
    });

    // Inspector buttons
    $('copyIdBtn').addEventListener('click', () => {
        if (state.selectedNode) {
            navigator.clipboard.writeText(state.selectedNode.id());
            $('copyIdBtn').textContent = '✓';
            setTimeout(() => { $('copyIdBtn').textContent = '📋'; }, 1000);
        }
    });

    $('centerBtn').addEventListener('click', () => {
        if (state.selectedNode) {
            state.cy.center(state.selectedNode);
        }
    });

    $('openExplorerBtn').addEventListener('click', () => {
        if (state.selectedNode) {
            window.location.href = `/#thing=${encodeURIComponent(state.selectedNode.id())}`;
        }
    });
}

// =============================================================================
// Initialization
// =============================================================================

async function init() {
    // Initialize label measurer
    labelMeasurer.init();

    // Initialize ELK worker
    elkLayout.init();

    // Initialize Cytoscape
    initCytoscape();
    bindEvents();

    // Load initial params from URL
    const { root, depth } = getUrlParams();
    $('rootInput').value = root;
    $('depthInput').value = depth;

    // Load initial graph
    await loadGraph(root, depth);
}

// Start when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}
