// Thing-OS Graph Explorer
// ========================
// A lightweight graph browser that can be upgraded to 3D rendering later.

// =============================================================================
// Data Layer (api.js-like)
// =============================================================================

const api = {
    async get(path) {
        const r = await fetch(path, { headers: { "Accept": "application/json" } });
        const ct = r.headers.get("content-type") || "";
        if (!r.ok) {
            let body = "";
            try { body = ct.includes("json") ? JSON.stringify(await r.json()) : await r.text(); } catch { }
            throw new Error(`${r.status} ${r.statusText} for ${path}\n${body}`);
        }
        return ct.includes("json") ? r.json() : r.text();
    },

    fetchThing(id) {
        return this.get(`/api/v1/things/${encodeURIComponent(id)}`);
    },

    resolvePath(path) {
        return this.get(`/api/v1/path${path}`);
    },

    fetchBytespaceMeta(id, key) {
        return this.get(`/api/v1/things/${encodeURIComponent(id)}/bytespaces/${encodeURIComponent(key)}/meta`);
    },

    getBytespaceUrl(id, key) {
        return `/api/v1/things/${encodeURIComponent(id)}/bytespaces/${encodeURIComponent(key)}`;
    },

    watch(rootId, onEvent) {
        const url = `/api/v1/watch?root=${encodeURIComponent(rootId)}`;
        const es = new EventSource(url);

        es.addEventListener("thing_changed", (e) => onEvent("thing_changed", e.data));
        es.addEventListener("bytespace_changed", (e) => onEvent("bytespace_changed", e.data));
        es.onerror = () => onEvent("error", null);

        return es;
    }
};

// =============================================================================
// Scene-Ready Graph Format (for 3D future)
// =============================================================================

const graphSnapshot = {
    nodes: [], // [{ id, kind, label }]
    edges: [], // [{ from, to, rel }]

    clear() {
        this.nodes = [];
        this.edges = [];
    },

    addNode(thing) {
        this.nodes.push({
            id: thing.thing_id,
            kind: thing.kind_id || "unknown",
            label: thing.thing_id.toString()
        });
    },

    addEdgesFromLinks(thing) {
        const links = thing.links || [];
        for (const l of links) {
            this.edges.push({
                from: thing.thing_id,
                to: l.thing_id,
                rel: l.rel || "link"
            });
        }
    },

    getSnapshot() {
        return { nodes: [...this.nodes], edges: [...this.edges] };
    }
};

// =============================================================================
// 2D Renderer (render2d.js-like)
// =============================================================================

const $ = (id) => document.getElementById(id);

const state = {
    live: false,
    es: null,
    currentThing: null,
};

function setHash(obj) {
    const params = new URLSearchParams(obj);
    location.hash = params.toString();
}

function getHash() {
    const h = location.hash.startsWith("#") ? location.hash.slice(1) : "";
    return Object.fromEntries(new URLSearchParams(h));
}

function linkifyThingId(text) {
    // Heuristic: ThingIds are numeric in this system
    const re = /\b(\d{1,20})\b/g;
    return text.replace(re, (m, id) => `<a href="#thing=${encodeURIComponent(id)}">${id}</a>`);
}

function renderRoots() {
    // Known system roots - adjust as more are discovered
    const roots = [
        { label: "/sys", path: "/sys" },
        { label: "/assets", path: "/assets" },
        { label: "/apps", path: "/apps" },
        { label: "/ui", path: "/ui" },
        { label: "/pci", path: "/pci" },
    ];
    $("roots").innerHTML = roots.map(r =>
        `<div><a href="#path=${encodeURIComponent(r.path)}">${r.label}</a></div>`
    ).join("");
}

function renderThing(thing) {
    state.currentThing = thing.thing_id;

    // Update graph snapshot for 3D
    graphSnapshot.clear();
    graphSnapshot.addNode(thing);
    graphSnapshot.addEdgesFromLinks(thing);

    $("thingTitle").innerHTML = `Thing ${thing.thing_id}` +
        (state.live ? '<span class="badge live">LIVE</span>' : '');
    $("thingMeta").textContent = `kind: ${thing.kind_id || "?"}`;

    $("rawJson").textContent = JSON.stringify(thing, null, 2);

    // Links panel
    const links = thing.links || [];
    if (links.length === 0) {
        $("links").innerHTML = `<div class="subtle">No links.</div>`;
    } else {
        const grouped = new Map();
        for (const l of links) {
            const rel = l.rel || "link";
            if (!grouped.has(rel)) grouped.set(rel, []);
            grouped.get(rel).push(l);
        }
        $("links").innerHTML = [...grouped.entries()].map(([rel, arr]) => {
            const items = arr.map(l => `<div><a href="#thing=${encodeURIComponent(l.thing_id)}">${l.thing_id}</a></div>`).join("");
            return `<div><div class="subtle">${rel}</div>${items}</div>`;
        }).join("");
    }

    // Props table
    const props = thing.props || {};
    const keys = Object.keys(props).sort();
    let html = `<table class="table">
    <thead><tr><th>key</th><th>type</th><th>value</th></tr></thead><tbody>`;

    for (const k of keys) {
        const p = props[k];
        const type = p?.type ?? "unknown";
        let v = "";

        if (type === "bytespace") {
            const metaLink = `/api/v1/things/${encodeURIComponent(thing.thing_id)}/bytespaces/${encodeURIComponent(k)}/meta`;
            const rawLink = `/api/v1/things/${encodeURIComponent(thing.thing_id)}/bytespaces/${encodeURIComponent(k)}`;
            v = `<a href="${rawLink}" target="_blank">open raw</a> · <a href="#" data-meta="${metaLink}" data-raw="${rawLink}">preview</a>
           <div class="subtle" id="meta-${k}"></div>
           <div id="preview-${k}"></div>`;
        } else {
            const text = (p && "value" in p) ? String(p.value) : JSON.stringify(p);
            v = `<span class="code">${linkifyThingId(text)}</span>`;
        }

        html += `<tr><td class="code">${k}</td><td>${type}</td><td>${v}</td></tr>`;
    }

    html += `</tbody></table>`;
    $("props").innerHTML = html;

    // Hook up preview clicks
    $("props").querySelectorAll("[data-meta]").forEach(el => {
        el.addEventListener("click", async (ev) => {
            ev.preventDefault();
            const metaUrl = el.getAttribute("data-meta");
            const rawUrl = el.getAttribute("data-raw");
            const key = decodeURIComponent(rawUrl.split("/bytespaces/")[1].split("?")[0]);
            await previewBytespace(key, metaUrl, rawUrl);
        });
    });
}

async function previewBytespace(key, metaUrl, rawUrl) {
    const metaEl = $(`meta-${key}`);
    const prevEl = $(`preview-${key}`);
    if (!metaEl || !prevEl) return;

    metaEl.textContent = "Loading…";
    prevEl.innerHTML = "";

    try {
        const meta = await api.get(metaUrl);
        metaEl.textContent = `size=${meta.size ?? "?"} type=${meta.content_type ?? "?"} gen=${meta.gen ?? "?"}`;

        const ct = (meta.content_type || "").toLowerCase();
        const cap = 256 * 1024;
        const url = meta.size && meta.size > cap ? `${rawUrl}?offset=0&len=${cap}` : rawUrl;

        if (ct.startsWith("image/")) {
            prevEl.innerHTML = `<div style="margin-top:8px"><img src="${rawUrl}" style="max-width:100%; border-radius:12px; border:1px solid #2b2b2b"></div>`;
            return;
        }

        // Text-ish fallback
        const r = await fetch(url);
        const buf = await r.arrayBuffer();
        const text = new TextDecoder("utf-8", { fatal: false }).decode(buf);
        prevEl.innerHTML = `<pre class="code" style="margin-top:8px; max-height:260px; overflow:auto; border:1px solid #232323; border-radius:12px; padding:10px">${text.replace(/[<>&]/g, c => ({ '<': '&lt;', '>': '&gt;', '&': '&amp;' }[c]))}</pre>`;
    } catch (e) {
        metaEl.textContent = `Error: ${e.message}`;
    }
}

async function goThing(id) {
    try {
        const thing = await api.fetchThing(id);
        renderThing(thing);
        if (state.live) startWatch();
    } catch (e) {
        $("thingTitle").textContent = `Error loading Thing ${id}`;
        $("thingMeta").textContent = e.message;
        $("rawJson").textContent = "{}";
        $("props").innerHTML = "";
    }
}

async function goPath(path) {
    try {
        const res = await api.resolvePath(path);
        if (res.thing_id) {
            setHash({ thing: res.thing_id });
        } else {
            $("thingTitle").textContent = `Path ${path}`;
            $("thingMeta").textContent = `No thing resolved.`;
            $("rawJson").textContent = JSON.stringify(res, null, 2);
            $("props").innerHTML = "";
        }
    } catch (e) {
        $("thingTitle").textContent = `Error resolving ${path}`;
        $("thingMeta").textContent = e.message;
    }
}

function stopWatch() {
    if (state.es) { state.es.close(); state.es = null; }
}

function startWatch() {
    stopWatch();
    if (!state.currentThing) return;

    state.es = api.watch(state.currentThing, async (event, data) => {
        if (event === "thing_changed" || event === "bytespace_changed") {
            try { await goThing(state.currentThing); } catch { }
        }
    });
}

function setLive(on) {
    state.live = on;
    const b = $("toggleLive");
    b.textContent = `LIVE: ${on ? "on" : "off"}`;
    b.classList.toggle("on", on);
    if (on) startWatch(); else stopWatch();
    // Re-render to show/hide badge
    if (state.currentThing) {
        $("thingTitle").innerHTML = `Thing ${state.currentThing}` +
            (on ? '<span class="badge live">LIVE</span>' : '');
    }
}

async function init() {
    renderRoots();

    // API info
    try {
        const info = await api.get("/api/v1/");
        $("apiInfo").textContent = `api=${info.api_version || "v1"} schema=${info.schema_hash || "?"}`;
    } catch (e) {
        $("apiInfo").textContent = `API not reachable: ${e.message}`;
    }

    $("goThing").onclick = async () => {
        const id = $("thingInput").value.trim();
        if (!id) return;
        setHash({ thing: id });
    };

    $("goPath").onclick = async () => {
        const p = $("pathInput").value.trim();
        if (!p) return;
        setHash({ path: p });
    };

    $("goHome").onclick = () => setHash({});

    $("toggleLive").onclick = () => setLive(!state.live);

    window.addEventListener("hashchange", async () => {
        const h = getHash();
        try {
            if (h.thing) await goThing(h.thing);
            else if (h.path) await goPath(h.path);
            else {
                $("thingTitle").textContent = "Welcome";
                $("thingMeta").textContent = "Enter a ThingId or a path.";
                $("rawJson").textContent = "{}";
                $("props").innerHTML = "";
                $("links").innerHTML = `<div class="subtle">Navigate to a Thing to see links.</div>`;
            }
        } catch (e) {
            $("thingTitle").textContent = "Error";
            $("thingMeta").textContent = e.message;
        }
    });

    // Kick hash handler once on load
    window.dispatchEvent(new Event("hashchange"));
}

// =============================================================================
// 3D Renderer Stub (render3d.js-like)
// =============================================================================

const render3d = {
    // Placeholder: receives the scene-ready graph format
    updateScene(snapshot) {
        console.log("3D scene update:", snapshot.nodes.length, "nodes,", snapshot.edges.length, "edges");
        // Future: pass to Three.js/Babylon scene
    },

    // Placeholder: mount canvas
    mount(canvas) {
        console.log("3D mount on canvas:", canvas);
        // Future: initialize 3D library here
    }
};

// Export for 3d.html to use
window.graphExplorer = { api, graphSnapshot, render3d };

init();
