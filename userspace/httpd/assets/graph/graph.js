// Thing-OS Graph Viewer
// ======================
// Interactive Cytoscape.js graph visualization with layout persistence

// =============================================================================
// Configuration
// =============================================================================

const CONFIG = {
    DEFAULT_ROOT: '',   // Empty = auto-discover from system nodes (like Photosynthesis)
    DEFAULT_DEPTH: 3,   // Increased for auto-discovery
    MAX_NODES: 500,
    AUTOSAVE_DEBOUNCE_MS: 750,
    LAYOUT_SPACE: 'graph_ui_v1',
};

// =============================================================================
// DOM Helpers
// =============================================================================

const $ = (id) => document.getElementById(id);

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
// State
// =============================================================================

const state = {
    cy: null,
    selectedNode: null,
    autoSave: false,
    saveTimeout: null,
    movedNodes: new Set(),
    graphData: null,
};

// =============================================================================
// Cytoscape Initialization
// =============================================================================

function initCytoscape() {
    state.cy = cytoscape({
        container: $('cy'),
        elements: [],
        style: [
            // Photosynthesis-style nodes: 120x160 rectangular cards
            {
                selector: 'node',
                style: {
                    // Card dimensions to match Photosynthesis (120x160)
                    'width': 120,
                    'height': 160,
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
                    'text-valign': 'bottom',
                    'text-halign': 'center',
                    'text-margin-y': -20,
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
            // Edges: gray lines matching Photosynthesis 0xFF888888
            {
                selector: 'edge',
                style: {
                    'width': 2,
                    'line-color': '#888888',
                    'target-arrow-color': '#888888',
                    'target-arrow-shape': 'triangle',
                    'curve-style': 'bezier',
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
// Graph Loading
// =============================================================================

async function loadGraph(root, depth) {
    setStatus('Loading...', '');

    try {
        const data = await api.getSubgraph(root, depth);
        state.graphData = data;
        state.movedNodes.clear();

        // Determine if we should use preset or auto layout
        const nodesWithPos = data.nodes.filter(n => n.x !== undefined && n.y !== undefined);
        const usePreset = nodesWithPos.length > data.nodes.length * 0.5;

        // Build Cytoscape elements
        const elements = [];

        for (const n of data.nodes) {
            // Format labels like Photosynthesis: prefer name, fallback to shortened ID
            const primaryLabel = n.label || n.id.toString().slice(-8);
            // Use kind_name from API (resolved symbol name like "ui.window")
            const kindName = n.kind_name || 'unknown';

            const elem = {
                data: {
                    id: n.id.toString(),
                    // Primary label (name or shortened ID)
                    label: primaryLabel,
                    // Kind name for secondary display
                    kindName: kindName,
                    kind: n.kind || 0,
                    // Only mark as root if a specific root was requested
                    isRoot: root && root.trim() !== '' && n.id.toString() === root.toString(),
                },
            };
            if (usePreset && n.x !== undefined && n.y !== undefined) {
                elem.position = { x: n.x, y: n.y };
            }
            elements.push(elem);
        }

        for (const e of data.edges) {
            elements.push({
                data: {
                    id: e.id,
                    source: e.from.toString(),
                    target: e.to.toString(),
                    rel: e.rel || 0,
                    // Use resolved relationship name for edge label
                    label: e.rel_name || 'link',
                },
            });
        }

        // Update graph
        state.cy.elements().remove();
        state.cy.add(elements);

        // Apply layout
        if (usePreset) {
            // Already positioned via preset
            state.cy.fit(undefined, 50);
        } else {
            // Use cose layout for automatic positioning
            state.cy.layout({
                name: 'cose',
                animate: false,
                padding: 50,
                nodeRepulsion: 8000,
                idealEdgeLength: 100,
            }).run();
        }

        // Update stats
        const truncMsg = data.truncated ? ' (truncated)' : '';
        $('graphStats').textContent =
            `${data.nodes.length} nodes, ${data.edges.length} edges, depth ${data.stats?.depth || depth}${truncMsg}`;

        setStatus('Ready', '');

    } catch (err) {
        setStatus(`Error: ${err.message}`, '');
        console.error('Load failed:', err);
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
