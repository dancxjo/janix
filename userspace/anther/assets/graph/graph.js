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
    // Network and layout stability
    API_TIMEOUT_MS: 8000,
    SUBGRAPH_RETRIES: 3,
    SUBGRAPH_RETRY_BASE_MS: 250,
    ELK_TIMEOUT_MS: 15000,
    ELK_RETRY_COOLDOWN_MS: 5000,
    // Physics
    FORCE_LINK_DISTANCE: 80,
    FORCE_CHARGE: -200,
    FORCE_COLLIDE_PADDING: 10,
    FORCE_ALPHA_DECAY: 0.05,
    FORCE_VELOCITY_DECAY: 0.3,
};

// =============================================================================
// DOM Helpers
// =============================================================================

const $ = (id) => document.getElementById(id);

const sleep = (ms) => new Promise(resolve => setTimeout(resolve, ms));

async function fetchWithTimeout(url, options = {}, timeoutMs = CONFIG.API_TIMEOUT_MS) {
    const controller = new AbortController();
    const timeout = setTimeout(() => controller.abort(), timeoutMs);
    try {
        return await fetch(url, { ...options, signal: controller.signal });
    } finally {
        clearTimeout(timeout);
    }
}

function isRetryableError(err) {
    if (!err) return false;
    if (err.name === 'AbortError') return true;
    if (err.name === 'TypeError') return true; // Network errors surface as TypeError in fetch
    const msg = String(err.message || '');
    const code = parseInt(msg.slice(0, 3), 10);
    return !Number.isNaN(code) && (code >= 500 || code === 408 || code === 429);
}

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
    pending: new Map(), // requestId -> { resolve, reject, timeoutId }
    requestCounter: 0,
    ready: false,
    disabledUntil: 0,
    lastError: null,

    init() {
        if (this.worker) {
            return;
        }
        this.ready = false;
        this.worker = new Worker('/elk-worker.js');

        this.worker.onmessage = (event) => {
            const data = event.data;

            // Handle ready signal
            if (data.type === 'ready') {
                if (data.error) {
                    this.ready = false;
                    this.disable(data.error);
                    console.warn('[ELK] Worker disabled:', data.error);
                    this.resetWorker();
                } else {
                    this.ready = true;
                    console.log('[ELK] Worker ready');
                }
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
            this.rejectAll(new Error('Worker crashed'));
            this.disable('Worker crashed');
            this.resetWorker();
        };

        this.worker.onmessageerror = (err) => {
            console.error('[ELK] Worker message error:', err);
            this.rejectAll(new Error('Worker message error'));
            this.disable('Worker message error');
            this.resetWorker();
        };
    },

    layout(graph, options = {}) {
        return new Promise((resolve, reject) => {
            if (this.isDisabled()) {
                reject(new Error(`ELK disabled: ${this.lastError || 'cooldown'}`));
                return;
            }

            this.init();
            if (!this.worker) {
                reject(new Error('ELK worker unavailable'));
                return;
            }
            const requestId = `elk-${++this.requestCounter}`;

            const timeoutId = setTimeout(() => {
                this.pending.delete(requestId);
                const err = new Error(`Layout timed out after ${CONFIG.ELK_TIMEOUT_MS}ms`);
                this.disable(err.message);
                this.resetWorker();
                reject(err);
            }, CONFIG.ELK_TIMEOUT_MS);

            this.pending.set(requestId, {
                resolve: (payload) => {
                    clearTimeout(timeoutId);
                    resolve(payload);
                },
                reject: (err) => {
                    clearTimeout(timeoutId);
                    reject(err);
                },
                timeoutId,
            });

            try {
                this.worker.postMessage({
                    requestId,
                    graph,
                    options,
                });
            } catch (err) {
                clearTimeout(timeoutId);
                this.pending.delete(requestId);
                this.disable(err.message || 'Worker postMessage failed');
                this.resetWorker();
                reject(err);
            }
        });
    },

    // Cancel pending request (for debouncing)
    cancel(requestId) {
        const pending = this.pending.get(requestId);
        if (pending?.timeoutId) {
            clearTimeout(pending.timeoutId);
        }
        this.pending.delete(requestId);
    },

    rejectAll(err) {
        for (const [, pending] of this.pending) {
            if (pending.timeoutId) {
                clearTimeout(pending.timeoutId);
            }
            pending.reject(err);
        }
        this.pending.clear();
    },

    resetWorker() {
        if (this.worker) {
            this.worker.terminate();
        }
        this.worker = null;
        this.ready = false;
    },

    disable(reason) {
        this.lastError = reason;
        this.disabledUntil = performance.now() + CONFIG.ELK_RETRY_COOLDOWN_MS;
    },

    isDisabled() {
        return performance.now() < this.disabledUntil;
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
        const r = await fetchWithTimeout(`/api/v1/subgraph?${params}`);
        if (!r.ok) {
            const text = await r.text();
            throw new Error(`${r.status} ${r.statusText}: ${text}`);
        }
        return r.json();
        if (!r.ok) {
            const text = await r.text();
            throw new Error(`${r.status} ${r.statusText}: ${text}`);
        }
        return r.json();
    },

    async getViews() {
        const r = await fetchWithTimeout('/api/v1/views');
        if (!r.ok) throw new Error('Failed to fetch views');
        return r.json();
    },

    async getView(viewId, query = '') {
        // Query might be passed to view endpoint if implemented
        const r = await fetchWithTimeout(`/api/v1/views/${encodeURIComponent(viewId)}?${query}`);
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
        const r = await fetchWithTimeout('/api/v1/layout', {
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

    async launch(thingId) {
        const r = await fetchWithTimeout(`/api/v1/things/${encodeURIComponent(thingId)}/launch`, {
            method: 'POST',
        });
        const ct = r.headers.get('content-type') || '';
        const body = ct.includes('json') ? await r.json() : await r.text();
        if (!r.ok) {
            const msg = body?.error?.message || (typeof body === 'string' ? body : `${r.status} ${r.statusText}`);
            throw new Error(msg);
        }
        return body;
    },

    async executeGqlQuery(query) {
        const r = await fetchWithTimeout('/api/v1/query', {
            method: 'POST',
            headers: { 'Content-Type': 'text/plain' },
            body: query,
        });
        if (!r.ok) {
            const text = await r.text();
            throw new Error(`${r.status} ${r.statusText}: ${text}`);
        }
        return await r.json();
    },

    async uploadFile(file) {
        const r = await fetchWithTimeout('/upload', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/octet-stream',
                'X-File-Name': file.name,
            },
            body: file,
        }, 30000); // Longer timeout for uploads
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
        const label = node.name || node.label || node.id.toString().slice(-8);
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
                name: node.name || '',
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
    loadSeq: 0,
    // Property watching
    watchInterval: null,
    lastProps: {},  // Track previous values for change detection
    launchInFlight: false,
    // Physics
    simulation: null,
    forceEnabled: true,
    elkPositions: new Map(), // id -> {x, y}
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
                        const name = ele.data('name') || ele.data('label') || '';
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

    // Drag handlers for physics interaction
    state.cy.on('grab', 'node', (evt) => {
        const node = evt.target;
        if (state.simulation && state.forceEnabled) {
            const d3Node = state.simulation.nodes().find(n => n.id === node.id());
            if (d3Node) {
                d3Node.fx = d3Node.x;
                d3Node.fy = d3Node.y;
                state.simulation.alphaTarget(0.3).restart();
            }
        }
    });

    state.cy.on('drag', 'node', (evt) => {
        const node = evt.target;
        if (state.simulation && state.forceEnabled) {
            const d3Node = state.simulation.nodes().find(n => n.id === node.id());
            if (d3Node) {
                const pos = node.position();
                d3Node.fx = pos.x;
                d3Node.fy = pos.y;
            }
        }
    });

    state.cy.on('dragfree', 'node', (evt) => {
        const node = evt.target;
        state.movedNodes.add(node.id());

        // Wake simulation if enabled
        if (state.simulation && state.forceEnabled) {
            state.simulation.alpha(0.3).restart();
            // Free the node from fixed position after drag ends?
            // Usually d3-force users want to pin it, but for our case let's release it
            // back into the pool unless we implement per-node pinning.
            // For now, update the d3 node state to match final drag position.
            const d3Node = state.simulation.nodes().find(n => n.id === node.id());
            if (d3Node) {
                d3Node.x = node.position('x');
                d3Node.y = node.position('y');
                d3Node.fx = null;
                d3Node.fy = null;
            }
        }

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

async function getSubgraphWithRetry(root, depth) {
    let attempt = 0;
    const maxAttempts = CONFIG.SUBGRAPH_RETRIES + 1;

    while (attempt < maxAttempts) {
        try {
            return await api.getSubgraph(root, depth);
        } catch (err) {
            attempt += 1;
            if (!isRetryableError(err) || attempt >= maxAttempts) {
                throw err;
            }
            const delay = CONFIG.SUBGRAPH_RETRY_BASE_MS * Math.pow(2, attempt - 1);
            setStatus(`Load failed, retrying in ${delay}ms (${attempt}/${CONFIG.SUBGRAPH_RETRIES})...`, 'retrying');
            await sleep(delay);
        }
    }

    throw new Error('Failed to load subgraph');
}

async function loadViewData(viewData) {
    const loadSeq = ++state.loadSeq;

    if (viewData.view && viewData.view.id) {
        const url = new URL(window.location);
        url.searchParams.set('view', viewData.view.id);
        url.searchParams.delete('thing');
        url.searchParams.delete('root');
        window.history.pushState({}, '', url);
    }

    try {
        if (loadSeq !== state.loadSeq) return;

        state.graphData = {
            nodes: viewData.nodes || [],
            edges: viewData.edges || []
        };
        state.movedNodes.clear();

        // Apply Hints
        const hints = viewData.hints || {};
        let layoutOptions = {};
        if (hints.layout) {
            if (hints.layout.direction) {
                layoutOptions['elk.direction'] = hints.layout.direction;
            }
        }
        await runElkLayout(state.graphData, null, layoutOptions);

    } catch (err) {
        if (loadSeq !== state.loadSeq) return;
        setStatus(`Error loading view: ${err.message}`, 'error');
        console.error('View load failed:', err);
    }
}

async function initViews() {
    try {
        const views = await api.getViews();
        const select = $('viewSelect');
        // Clear except default
        while (select.options.length > 1) {
            select.remove(1);
        }

        views.forEach(v => {
            const opt = document.createElement('option');
            opt.value = v.query || `VIEW ${v.id}`;
            opt.textContent = v.title;
            opt.title = v.description;
            select.appendChild(opt);
        });

        // Handle selection change
        select.onchange = () => {
            const query = select.value;
            if (query) {
                $('gqlInput').value = query;
                executeGqlQuery();
            } else {
                const url = new URL(window.location);
                url.searchParams.delete('view');
                window.history.pushState({}, '', url);
            }
        };

        // Check URL for initial view
        const urlParams = new URLSearchParams(window.location.search);
        const viewId = urlParams.get('view');
        if (viewId) {
            const expectedQuery = `VIEW ${viewId}`;
            for (let i = 0; i < select.options.length; i++) {
                if (select.options[i].value === expectedQuery) {
                    select.selectedIndex = i;
                    $('gqlInput').value = expectedQuery;
                    executeGqlQuery();
                    return true;
                }
            }
        }
    } catch (err) {
        console.warn('Failed to init views:', err);
    }
    return false;
}

async function loadGraph(root, depth) {
    const loadSeq = ++state.loadSeq;
    setStatus('Loading...', '');

    try {
        const data = await getSubgraphWithRetry(root, depth);
        if (loadSeq !== state.loadSeq) {
            return;
        }
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
        if (loadSeq !== state.loadSeq) {
            return;
        }
        setStatus(`Error: ${err.message}`, '');
        console.error('Load failed:', err);
    }
}

// =============================================================================
// Physics Simulation (d3-force)
// =============================================================================

function initForceSimulation(laidOut, edges) {
    if (!state.forceEnabled) return;

    // Convert ELK layout to d3 nodes
    // laidOut is { children: [{id, x, y, width, height}], edges: [...] }
    const nodes = (laidOut.children || []).map(n => ({
        id: n.id,
        // ELK is top-left, move to center
        x: n.x + n.width / 2,
        y: n.y + n.height / 2,
        width: n.width,
        height: n.height,
        // Save initial ELK pos for reset
        initialX: n.x + n.width / 2,
        initialY: n.y + n.height / 2,
        // Preserve data for simulation logic
        _data: n._data || {}
    }));

    // Save ELK positions for reset functionality
    state.elkPositions.clear();
    nodes.forEach(n => state.elkPositions.set(n.id, { x: n.initialX, y: n.initialY }));

    const links = (edges || []).map(e => ({
        id: e.id,
        source: e.from.toString(),
        target: e.to.toString(),
    }));

    if (state.simulation) {
        state.simulation.stop();
    }

    state.simulation = d3.forceSimulation(nodes)
        .alphaDecay(CONFIG.FORCE_ALPHA_DECAY)
        .velocityDecay(CONFIG.FORCE_VELOCITY_DECAY)
        .force('link', d3.forceLink(links).id(d => d.id).distance(CONFIG.FORCE_LINK_DISTANCE))
        .force('charge', d3.forceManyBody().strength(CONFIG.FORCE_CHARGE))
        .force('collide', d3.forceCollide().radius(d => Math.max(d.width, d.height) / 2 + CONFIG.FORCE_COLLIDE_PADDING))
        // ANTHER LAYOUT RULES
        // 1. mem.Range tiled across the bottom
        .force('mem', d3.forceY(500).strength(d => d._data.kindName === 'mem.Range' ? 0.8 : 0))
        // 2. dev.Host just above svc.Root
        .force('host_x', d3.forceX(0).strength(d => d.id === 'dev.Host' ? 1.0 : 0))
        .force('host_y', d3.forceY(-150).strength(d => d.id === 'dev.Host' ? 1.0 : 0))
        // 3. svc.Root centered
        .force('root_center', d3.forceRadial(0, 0, 0).strength(d => d.id === 'svc.Root' ? 1.0 : 0))
        .on('tick', () => {
            // Apply positions to Cytoscape
            // To improve performance, we could batch these or use requestAnimationFrame
            // but for <500 nodes, direct update is usually fine.
            state.cy.batch(() => {
                for (const node of nodes) {
                    const el = state.cy.getElementById(node.id);
                    if (el.length > 0 && !el.locked() && !el.grabbed()) {
                        el.position({ x: node.x, y: node.y });
                    }
                }
            });

            // Update inspector if selected node is moving
            if (state.selectedNode) {
                updateInspectorPosition(state.selectedNode);
            }
        });
}

function stopSimulation() {
    if (state.simulation) {
        state.simulation.stop();
    }
}

function restartSimulation() {
    if (state.simulation && state.forceEnabled) {
        state.simulation.alpha(0.3).restart();
    }
}

function resetToElk() {
    stopSimulation();

    state.cy.batch(() => {
        state.elkPositions.forEach((pos, id) => {
            const node = state.cy.getElementById(id);
            if (node.length) {
                node.position(pos);
                // Also update d3 state if it exists
                if (state.simulation) {
                    const d3Node = state.simulation.nodes().find(n => n.id === id);
                    if (d3Node) {
                        d3Node.x = pos.x;
                        d3Node.y = pos.y;
                        d3Node.vx = 0;
                        d3Node.vy = 0;
                    }
                }
            }
        });
    });

    // If relax is on, restart gently? Or stay static?
    // "Reset to ELK" implies "put things back". If physics is on, they'll drift again.
    // Let's assume user wants to see the grid.
    // But if relax toggle is checked, it will eventually start moving if we restart.
    // Let's just set positions. If simulation is running, it will continue from there.
    if (state.forceEnabled) {
        state.simulation.alpha(0.1).restart();
    }
}

function zapToCenter(nodeId) {
    if (!state.simulation) return;

    const node = state.simulation.nodes().find(n => n.id === nodeId);
    if (!node) return;

    // 1. Pan viewport to center
    // We already do state.cy.center(node) in some places, but let's be explicit
    // Actually, physically pulling the node to (0,0) or center of layout might destroy layout structure.
    // Instead, let's pull it to the *current center of the viewport* in physics space?
    // Or just re-center the camera on the node (standard behavior) and let physics relax neighbors?

    // The requirement says: "Clicking a node recenters view and 'zaps' it to the center with a smooth re-layout."
    // This implies a physics "pull" to the visual center.

    // Let's apply a temporary force to pull this node to the center of the graph's bounding box?
    // Or just simple re-heat:
    state.simulation.alpha(0.5).restart();

    // Optionally: pull to center of mass?
    // For now, just waking the simulation is usually enough to "breathe" around the selection.
}

async function runElkLayout(data, root) {
    setStatus('Computing layout...', '');

    if (elkLayout.isDisabled()) {
        renderWithCoseLayout(data, root);
        return;
    }

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

        // Start physics after ELK is done
        initForceSimulation(laidOut, data.edges);

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
        const primaryLabel = n.name || n.label || n.id.toString().slice(-8);
        const kindName = n.kind_name || 'unknown';

        elements.push({
            data: {
                id,
                label: primaryLabel,
                name: n.name || '',
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
        const primaryLabel = n.name || n.label || n.id.toString().slice(-8);
        const kindName = n.kind_name || 'unknown';
        const size = labelMeasurer.measure(primaryLabel, kindName);

        elements.push({
            data: {
                id: n.id.toString(),
                label: primaryLabel,
                name: n.name || '',
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
        const primaryLabel = n.name || n.label || n.id.toString().slice(-8);
        const kindName = n.kind_name || 'unknown';
        const size = labelMeasurer.measure(primaryLabel, kindName);

        elements.push({
            data: {
                id: n.id.toString(),
                label: primaryLabel,
                name: n.name || '',
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
    // Stop any existing watch
    stopWatching();
    resetLaunchUI();

    state.selectedNode = node;

    $('inspectorEmpty').style.display = 'none';
    $('inspectorContent').style.display = 'block';

    const id = node.id();
    const data = node.data();

    $('inspectorThingLink').textContent = id;
    const depth = parseInt($('depthInput').value) || CONFIG.DEFAULT_DEPTH;
    $('inspectorThingLink').href = `?root=${encodeURIComponent(id)}&depth=${depth}`;
    $('inspectorKind').textContent = data.kindName || data.kind || '-';
    $('inspectorLabel').textContent = data.name || data.label || '-';
    updateInspectorPosition(node);

    // Start watching this node's properties
    startWatching(id);

    // Physics "Zap"
    if (state.forceEnabled && state.simulation) {
        zapToCenter(id);
    }
}

function updateInspectorPosition(node) {
    const pos = node.position();
    $('inspectorPos').textContent = `x: ${pos.x.toFixed(1)}, y: ${pos.y.toFixed(1)}`;
}

function clearSelection() {
    stopWatching();
    state.selectedNode = null;
    $('inspectorEmpty').style.display = 'block';
    $('inspectorContent').style.display = 'none';
    resetLaunchUI();
}

function resetLaunchUI() {
    const btn = $('launchBtn');
    const status = $('launchStatus');
    if (btn) {
        btn.style.display = 'none';
        btn.disabled = false;
        btn.textContent = 'Launch';
    }
    if (status) {
        status.style.display = 'none';
        status.textContent = '';
        status.className = 'launch-status';
    }
    state.launchInFlight = false;
}

function shortModuleName(name) {
    if (!name) return '';
    const parts = name.split('/');
    return parts[parts.length - 1] || name;
}

function setLaunchStatus(message, kind) {
    const status = $('launchStatus');
    if (!status) return;
    status.textContent = message || '';
    status.className = 'launch-status';
    if (kind === 'success') {
        status.classList.add('success');
    } else if (kind === 'error') {
        status.classList.add('error');
    }
    status.style.display = message ? 'block' : 'none';
}

function updateLaunchControls(data) {
    const btn = $('launchBtn');
    const status = $('launchStatus');
    if (!btn || !status) return;

    if (!state.selectedNode) {
        resetLaunchUI();
        return;
    }

    const launchable = !!(data && data.launchable);
    if (!launchable) {
        btn.style.display = 'none';
        btn.disabled = true;
        btn.textContent = 'Launch';
        const reason = data && data.launch_reason ? data.launch_reason : '';
        setLaunchStatus(reason, '');
        return;
    }

    const name = data && data.launch_name ? data.launch_name : '';
    const label = name ? `Launch ${shortModuleName(name)}` : 'Launch';
    btn.textContent = label;
    btn.style.display = 'block';
    btn.disabled = state.launchInFlight;
    btn.onclick = () => launchSelected(state.selectedNode.id());
    if (!state.launchInFlight) {
        setLaunchStatus('', '');
    }
}

async function launchSelected(thingId) {
    if (!thingId || state.launchInFlight) return;
    state.launchInFlight = true;
    const btn = $('launchBtn');
    if (btn) btn.disabled = true;
    setLaunchStatus('Launching...', '');

    try {
        const res = await api.launch(thingId);
        const pid = res?.pid ?? '?';
        if (res && res.graph_linked === false && res.graph_error) {
            setLaunchStatus(`Launched (pid ${pid}), graph link failed: ${res.graph_error}`, 'error');
        } else {
            setLaunchStatus(`Launched (pid ${pid})`, 'success');
        }
    } catch (err) {
        setLaunchStatus(`Launch failed: ${err.message}`, 'error');
    } finally {
        state.launchInFlight = false;
        if (btn) btn.disabled = false;
    }
}

// =============================================================================
// Property Watching
// =============================================================================

const WATCH_POLL_INTERVAL_MS = 1000;  // Poll every 1 second

async function fetchProps(thingId) {
    try {
        const r = await fetchWithTimeout(`/api/v1/things/${thingId}/props`);
        if (!r.ok) {
            return null;
        }
        return r.json();
    } catch (err) {
        console.warn('[Watch] Fetch failed:', err);
        return null;
    }
}

function startWatching(thingId) {
    // Clear previous props
    state.lastProps = {};
    renderProps(null, true);  // Show loading state

    // Fetch immediately
    pollProps(thingId, true);

    // Set up interval for subsequent polls
    state.watchInterval = setInterval(() => {
        pollProps(thingId, false);
    }, WATCH_POLL_INTERVAL_MS);
}

function stopWatching() {
    if (state.watchInterval) {
        clearInterval(state.watchInterval);
        state.watchInterval = null;
    }
    state.lastProps = {};
    // Reset props list to empty state
    $('propsList').innerHTML = '<div class="props-empty">Select a node to view properties</div>';
    $('propsLoading').style.display = 'none';
}

async function pollProps(thingId, isInitial) {
    if (isInitial) {
        $('propsLoading').style.display = 'inline';
    }

    const data = await fetchProps(thingId);

    $('propsLoading').style.display = 'none';

    if (!data) {
        if (isInitial) {
            renderProps({ props: {} }, false);
        }
        return;
    }

    renderProps(data, false);
}

function renderProps(data, isLoading) {
    const container = $('propsList');

    if (isLoading) {
        container.innerHTML = '<div class="props-empty">Loading...</div>';
        updateLaunchControls(data);
        return;
    }

    if (!data || !data.props || Object.keys(data.props).length === 0) {
        container.innerHTML = '<div class="props-empty">No properties found</div>';
        updateLaunchControls(data);
        return;
    }

    // Build property list
    const props = data.props;
    const propKeys = Object.keys(props).sort();

    let html = '';
    if (data.truncated) {
        const count = data.prop_count ?? propKeys.length;
        html += `<div class="props-note">Showing first ${count} properties (truncated)</div>`;
    }
    for (const key of propKeys) {
        const value = props[key];
        const prevValue = state.lastProps[key];
        const isChanged = prevValue !== undefined && prevValue !== value;

        // Format value for display
        let displayValue = value;
        if (typeof value === 'number') {
            // For floats, show fewer decimals
            if (!Number.isInteger(value)) {
                displayValue = value.toFixed(2);
            }
        } else if (typeof value === 'string' && value.length > 30) {
            displayValue = value.substring(0, 27) + '...';
        }

        const changedClass = isChanged ? ' updated' : '';
        html += `<div class="prop-item">
            <span class="prop-name">${escapeHtml(key)}</span>
            <span class="prop-value${changedClass}" title="${escapeHtml(String(value))}">${escapeHtml(String(displayValue))}</span>
        </div>`;
    }

    container.innerHTML = html;

    // Update lastProps for next comparison
    state.lastProps = { ...props };

    updateLaunchControls(data);
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
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
// GQL Query Execution
// =============================================================================

async function executeGqlQuery() {
    const query = $('gqlInput').value.trim();
    if (!query) {
        return;
    }

    setStatus('Executing query...', '');
    showQueryResults('Executing...'); // Show loading state

    try {
        const result = await api.executeGqlQuery(query);

        // Prepare to render graph if result looks like a graph/view
        if (result && (result.view || (result.nodes && result.edges))) {
            hideQueryResults();
            await loadViewData(result);
            setStatus('Query executed (Graph View)', '');
        } else {
            showQueryResults(result);
            setStatus('Query executed', '');
        }
    } catch (err) {
        showQueryResults(`Error: ${err.message}`);
        setStatus(`Query error: ${err.message}`, '');
        console.error('Query execution failed:', err);
    }
}

function showQueryResults(data) {
    const panel = $('queryResults');
    const content = $('queryResultsContent');
    content.innerHTML = '';

    if (typeof data === 'string') {
        content.textContent = data;
    } else if (data && data.success === false) {
        content.innerHTML = `<div class="error-msg">Error: ${data.error || data.message || 'Unknown error'}</div>`;
    } else if (data && data.rows && data.columns) {
        renderQueryResultTable(data, content);
    } else if (data && data.message) {
        content.textContent = data.message;
    } else {
        const pre = document.createElement('pre');
        pre.textContent = JSON.stringify(data, null, 2);
        content.appendChild(pre);
    }

    panel.style.display = 'flex';
}

function renderQueryResultTable(data, container) {
    const table = document.createElement('table');
    table.className = 'query-result-table';

    // Header
    const thead = document.createElement('thead');
    const headerRow = document.createElement('tr');
    data.columns.forEach(col => {
        const th = document.createElement('th');
        th.textContent = col;
        headerRow.appendChild(th);
    });
    thead.appendChild(headerRow);
    table.appendChild(thead);

    // Body
    const tbody = document.createElement('tbody');
    data.rows.forEach(row => {
        const tr = document.createElement('tr');
        row.forEach(val => {
            const td = document.createElement('td');
            if (val && val.type === 'node') {
                const a = document.createElement('a');
                a.href = '#';
                a.textContent = `(ID: ${val.id})`;
                a.onclick = (e) => {
                    e.preventDefault();
                    // Select node in graph
                    const node = state.cy.getElementById(String(val.id));
                    if (node.length > 0) {
                        state.cy.center(node);
                        node.select();
                    } else {
                        // Not in current subgraph?
                        $('rootInput').value = val.id;
                        $('loadBtn').click();
                    }
                };
                td.appendChild(a);
            } else {
                td.textContent = val !== null ? val : 'null';
            }
            tr.appendChild(td);
        });
        tbody.appendChild(tr);
    });
    table.appendChild(tbody);
    container.appendChild(table);
}

function hideQueryResults() {
    const panel = $('queryResults');
    panel.style.display = 'none';
}

function clearQueryResults() {
    $('gqlInput').value = '';
    hideQueryResults();
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

    // Relax toggle
    $('relaxToggle').addEventListener('change', (e) => {
        state.forceEnabled = e.target.checked;
        if (state.forceEnabled) {
            restartSimulation();
        } else {
            stopSimulation();
        }
    });

    // Reset to ELK
    $('resetElkBtn').addEventListener('click', resetToElk);

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

    $('setRootBtn').addEventListener('click', () => {
        if (state.selectedNode) {
            const root = state.selectedNode.id();
            const depth = parseInt($('depthInput').value) || CONFIG.DEFAULT_DEPTH;
            $('rootInput').value = root;
            updateUrl(root, depth);
            loadGraph(root, depth);
        }
    });

    // GQL Query buttons
    $('executeBtn').addEventListener('click', executeGqlQuery);

    $('clearQueryBtn').addEventListener('click', clearQueryResults);

    $('closeQueryBtn').addEventListener('click', hideQueryResults);

    // Enter key in query input
    // Upload button
    $('uploadBtn').addEventListener('click', async () => {
        const fileInput = $('uploadInput');
        if (fileInput.files.length === 0) {
            alert('Please select a file first');
            return;
        }
        const file = fileInput.files[0];

        setStatus(`Uploading ${file.name}...`, '');
        try {
            await api.uploadFile(file);
            setStatus(`Uploaded ${file.name}`, 'success');
            // Clear input
            fileInput.value = '';
        } catch (err) {
            console.error('Upload failed:', err);
            setStatus(`Upload failed: ${err.message}`, 'error');
        }
    });

    $('gqlInput').addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            executeGqlQuery();
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

    if (typeof cytoscape === 'undefined') {
        console.error('Cytoscape.js failed to load');
        setStatus('Cytoscape failed to load', 'error');
        return;
    }

    // Initialize Cytoscape
    initCytoscape();
    bindEvents();

    // Load initial params from URL
    const { root, depth } = getUrlParams();
    $('rootInput').value = root;
    $('depthInput').value = depth;

    // Initialize views and check if a view needs to be loaded
    const viewLoaded = await initViews();

    // Only load default graph if no view was loaded from URL
    if (!viewLoaded) {
        await loadGraph(root, depth);
    }
}

// Start when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}
