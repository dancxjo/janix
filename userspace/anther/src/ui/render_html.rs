extern crate alloc;

use alloc::fmt::Write;
use alloc::string::String;

use crate::ui::graph_decode::{UiNode, UiNodeKind, UiTree};

pub fn render_window_html(tree: &UiTree, scene_gen: u64) -> String {
    let mut out = String::new();
    let window = tree.window_id.to_u64_lossy();

    let _ = write!(
        out,
        "<!doctype html><html><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>Petals UI {}</title>",
        window
    );
    out.push_str("<style>");
    out.push_str(CSS_BASE);
    out.push_str("</style></head><body>");
    let _ = write!(
        out,
        "<main id=\"petals-root\" class=\"petals-window\" data-window=\"{}\" data-target=\"{}\" data-scene-gen=\"{}\">",
        window,
        window,
        scene_gen
    );

    render_node(tree, tree.root, window, &mut out);

    out.push_str("</main><script src=\"/ui/petals_ui.js\"></script></body></html>");
    out
}

pub fn render_tree_json(tree: &UiTree, scene_gen: u64) -> String {
    let mut out = String::new();
    let _ = write!(
        out,
        "{{\"window\":{},\"scene_gen\":{},\"root\":{},\"nodes\":[",
        tree.window_id.to_u64_lossy(),
        scene_gen,
        tree.root
    );
    for (i, node) in tree.nodes.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let _ = write!(
            out,
            "{{\"id\":{},\"kind\":\"{}\",\"visible\":{},\"enabled\":{},\"checked\":{},\"action_id\":{}",
            node.id.to_u64_lossy(),
            kind_str(node.kind),
            if node.visible { "true" } else { "false" },
            if node.enabled { "true" } else { "false" },
            if node.checked { "true" } else { "false" },
            node.action_id,
        );
        out.push_str(",\"children\":[");
        for (cidx, child) in node.children.iter().enumerate() {
            if cidx > 0 {
                out.push(',');
            }
            let _ = write!(out, "{}", child);
        }
        out.push(']');
        if let Some(text) = node.text.as_ref() {
            let _ = write!(out, ",\"text\":\"{}\"", escape_json(text));
        }
        if let Some(placeholder) = node.placeholder.as_ref() {
            let _ = write!(out, ",\"placeholder\":\"{}\"", escape_json(placeholder));
        }
        out.push('}');
    }
    out.push_str("]}");
    out
}

fn render_node(tree: &UiTree, idx: usize, window: u64, out: &mut String) {
    let node = &tree.nodes[idx];
    if !node.visible {
        return;
    }

    let id = node.id.to_u64_lossy();
    let style = node_style(node);
    match node.kind {
        UiNodeKind::Window | UiNodeKind::Column | UiNodeKind::Row | UiNodeKind::Box | UiNodeKind::Unknown => {
            let _ = write!(
                out,
                "<div class=\"petals-node petals-{}\" data-window=\"{}\" data-target=\"{}\"{} style=\"{}\">",
                kind_str(node.kind),
                window,
                id,
                if node.enabled { "" } else { " aria-disabled=\"true\"" },
                style
            );
            for child in node.children.iter() {
                render_node(tree, *child, window, out);
            }
            out.push_str("</div>");
        }
        UiNodeKind::Text => {
            let text = node.text.as_deref().unwrap_or("");
            let _ = write!(
                out,
                "<p class=\"petals-node petals-text\" data-window=\"{}\" data-target=\"{}\" style=\"{}\">{}</p>",
                window,
                id,
                style,
                escape_html(text)
            );
        }
        UiNodeKind::TextInput => {
            let value = node.text.as_deref().unwrap_or("");
            let placeholder = node.placeholder.as_deref().unwrap_or("");
            let _ = write!(
                out,
                "<input class=\"petals-node petals-text-input\" type=\"text\" data-window=\"{}\" data-target=\"{}\" value=\"{}\" placeholder=\"{}\"{} style=\"{}\">",
                window,
                id,
                escape_attr(value),
                escape_attr(placeholder),
                if node.enabled { "" } else { " disabled" },
                style
            );
        }
        UiNodeKind::Button => {
            let text = node.text.as_deref().unwrap_or("Button");
            let _ = write!(
                out,
                "<button class=\"petals-node petals-button\" type=\"button\" data-window=\"{}\" data-target=\"{}\"{} style=\"{}\">{}</button>",
                window,
                id,
                if node.enabled { "" } else { " disabled" },
                style,
                escape_html(text)
            );
        }
        UiNodeKind::Checkbox => {
            let text = node.text.as_deref().unwrap_or("");
            let _ = write!(
                out,
                "<label class=\"petals-node petals-checkbox\" data-window=\"{}\" data-target=\"{}\" style=\"{}\"><input type=\"checkbox\" {}{}><span>{}</span></label>",
                window,
                id,
                style,
                if node.checked { "checked " } else { "" },
                if node.enabled { "" } else { "disabled" },
                escape_html(text)
            );
        }
    }
}

fn node_style(node: &UiNode) -> String {
    let mut style = String::new();

    match node.kind {
        UiNodeKind::Window | UiNodeKind::Column => {
            style.push_str("display:flex;flex-direction:column;");
        }
        UiNodeKind::Row => {
            style.push_str("display:flex;flex-direction:row;");
        }
        UiNodeKind::Box | UiNodeKind::Unknown => {
            if !node.children.is_empty() {
                style.push_str("display:flex;flex-direction:column;");
            }
        }
        _ => {}
    }

    if node.layout.gap > 0 {
        let _ = write!(style, "gap:{}px;", node.layout.gap);
    }
    if node.layout.padding > 0 {
        let _ = write!(style, "padding:{}px;", node.layout.padding);
    }
    if node.layout.margin > 0 {
        let _ = write!(style, "margin:{}px;", node.layout.margin);
    }
    if node.layout.flex_grow > 0 {
        let _ = write!(style, "flex:{} 1 0%;", node.layout.flex_grow);
    }
    if let Some(w) = node.layout.width {
        if w > 0 {
            let _ = write!(style, "width:{}px;", w);
        }
    }
    if let Some(h) = node.layout.height {
        if h > 0 {
            let _ = write!(style, "height:{}px;", h);
        }
    }

    style
}

fn kind_str(kind: UiNodeKind) -> &'static str {
    match kind {
        UiNodeKind::Window => "window",
        UiNodeKind::Column => "column",
        UiNodeKind::Row => "row",
        UiNodeKind::Box => "box",
        UiNodeKind::Text => "text",
        UiNodeKind::TextInput => "text_input",
        UiNodeKind::Button => "button",
        UiNodeKind::Checkbox => "checkbox",
        UiNodeKind::Unknown => "unknown",
    }
}

fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

fn escape_attr(s: &str) -> String {
    escape_html(s)
}

fn escape_json(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'"' => out.push_str("\\\""),
            b'\\' => out.push_str("\\\\"),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            b if b < 32 => {
                let _ = write!(out, "\\u{:04x}", b);
            }
            _ => out.push(b as char),
        }
    }
    out
}

const CSS_BASE: &str = r#"
:root {
  --bg: #f6f7fb;
  --fg: #202734;
  --surface: #ffffff;
  --muted: #748197;
  --accent: #1764d2;
  --border: #d8deea;
  --radius: 10px;
}
* { box-sizing: border-box; }
html, body {
  margin: 0;
  padding: 0;
  min-height: 100%;
  font-family: "IBM Plex Sans", "Segoe UI", sans-serif;
  color: var(--fg);
  background: radial-gradient(circle at 20% 0%, #ffffff, var(--bg));
}
.petals-window {
  min-height: 100vh;
  padding: 16px;
}
.petals-node { min-width: 0; }
.petals-text { margin: 0; color: var(--fg); }
.petals-button {
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--surface);
  color: var(--fg);
  cursor: pointer;
  padding: 8px 12px;
}
.petals-button:hover { border-color: var(--accent); }
.petals-text-input {
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 8px 10px;
  width: 100%;
  background: var(--surface);
  color: var(--fg);
}
.petals-text-input:focus {
  outline: 2px solid color-mix(in srgb, var(--accent) 35%, white);
  border-color: var(--accent);
}
.petals-checkbox {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  color: var(--fg);
}
"#;
