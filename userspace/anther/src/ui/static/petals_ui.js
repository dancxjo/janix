(function () {
  function parseU64(value) {
    if (!value) return null;
    const n = Number(value);
    return Number.isFinite(n) && n >= 0 ? Math.floor(n) : null;
  }

  function root() {
    return document.getElementById("petals-root");
  }

  function currentWindowId() {
    const el = root();
    return parseU64(el && el.dataset ? el.dataset.window : null);
  }

  function currentSceneGen() {
    const el = root();
    return parseU64(el && el.dataset ? el.dataset.sceneGen : null) || 0;
  }

  function targetFromEvent(ev) {
    const path = ev.composedPath ? ev.composedPath() : [ev.target];
    for (const item of path) {
      if (!item || !item.dataset) continue;
      const target = parseU64(item.dataset.target);
      const windowId = parseU64(item.dataset.window);
      if (target != null && windowId != null) {
        return { target, window: windowId };
      }
    }
    const win = currentWindowId();
    return { target: win, window: win };
  }

  function postEvent(payload) {
    return fetch("/ui/event", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    }).catch(() => {});
  }

  function wireEvents() {
    document.addEventListener("click", (ev) => {
      const hit = targetFromEvent(ev);
      if (hit.window == null || hit.target == null) return;
      postEvent({ kind: "activate", window: hit.window, target: hit.target });
    });

    document.addEventListener("focusin", (ev) => {
      const hit = targetFromEvent(ev);
      if (hit.window == null || hit.target == null) return;
      postEvent({ kind: "focus", window: hit.window, target: hit.target });
    });

    document.addEventListener("input", (ev) => {
      const t = ev.target;
      if (!(t instanceof HTMLInputElement) || t.type !== "text") return;
      const hit = targetFromEvent(ev);
      if (hit.window == null || hit.target == null) return;
      postEvent({
        kind: "text_input",
        window: hit.window,
        target: hit.target,
        text: t.value || "",
      });
    });

    document.addEventListener("keydown", (ev) => {
      if (ev.key !== "Enter") return;
      const hit = targetFromEvent(ev);
      if (hit.window == null || hit.target == null) return;
      postEvent({ kind: "submit", window: hit.window, target: hit.target });
    });

    document.addEventListener(
      "wheel",
      (ev) => {
        const hit = targetFromEvent(ev);
        if (hit.window == null || hit.target == null) return;
        postEvent({
          kind: "scroll",
          window: hit.window,
          target: hit.target,
          dx: Math.trunc(ev.deltaX || 0),
          dy: Math.trunc(ev.deltaY || 0),
          mods: 0,
        });
      },
      { passive: true }
    );
  }

  function startPolling() {
    setInterval(async () => {
      const windowId = currentWindowId();
      if (windowId == null) return;
      const gen = currentSceneGen();
      try {
        const res = await fetch(`/ui/${windowId}?gen=${gen}`, {
          cache: "no-store",
        });
        if (res.status === 204) return;
        if (!res.ok) return;
        const html = await res.text();
        if (!html) return;
        document.open();
        document.write(html);
        document.close();
      } catch (_) {}
    }, 500);
  }

  wireEvents();
  startPolling();
})();
