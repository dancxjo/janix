use abi::schema::keys as props;
use abi::types::HandleId;
use abi::wire::ThingId;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::geometry::{Color, Rect, RectF, Transform};
use crate::svg::ir::{
    FillRule, LineCap, LineJoin, Paint, Path2D, PathCommand, SvgIrDocument, SvgOp,
};
use crate::svg::parse;
use crate::svg::walk::SvgGraph;

#[derive(Debug)]
pub enum CompileError {
    NoRoot,
    InvalidRoot,
    GraphError,
}

#[derive(Clone, Debug)]
struct StyleState {
    transform: Transform,
    fill: Option<Paint>,
    stroke: Option<Paint>,
    stroke_width: f32,
    opacity: f32,
    fill_rule: FillRule,
    // ... linecap, linejoin, etc
}

impl Default for StyleState {
    fn default() -> Self {
        Self {
            transform: Transform::identity(),
            fill: Some(Paint::Solid(Color::BLACK)), // SVG default is black fill
            stroke: None,
            stroke_width: 1.0,
            opacity: 1.0,
            fill_rule: FillRule::NonZero,
        }
    }
}

pub fn compile_graph_to_ir(
    graph: &dyn SvgGraph,
    doc_id: ThingId,
) -> Result<SvgIrDocument, CompileError> {
    let root_id = graph.get_root(doc_id).ok_or(CompileError::NoRoot)?;
    let tag = graph
        .get_prop_str(root_id, props::TAG)
        .ok_or(CompileError::GraphError)?;

    if tag != "svg" {
        return Err(CompileError::InvalidRoot);
    }

    let mut doc = SvgIrDocument {
        width: None,
        height: None,
        view_box: None,
        ops: Vec::new(),
    };

    let attrs = graph.get_attributes(root_id);
    for (name, val) in &attrs {
        match name.as_str() {
            "width" => doc.width = val.parse().ok(),
            "height" => doc.height = val.parse().ok(),
            "viewBox" => {
                let parts: Vec<f32> = val
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if parts.len() == 4 {
                    doc.view_box = Some(RectF::new(
                        parts[0],
                        parts[1],
                        parts[2],
                        parts[3],
                    )); 
                }
            }
            _ => {}
        }
    }

    let initial_state = StyleState::default();
    compile_element(graph, root_id, &initial_state, &mut doc.ops)?;

    Ok(doc)
}

fn compile_element(
    graph: &dyn SvgGraph,
    node: ThingId,
    parent_state: &StyleState,
    ops: &mut Vec<SvgOp>,
) -> Result<(), CompileError> {
    // Resolve state for this node
    let mut state = parent_state.clone();
    let attrs = graph.get_attributes(node);

    // 1. Transform
    for (k, v) in &attrs {
        if k == "transform" {
            let t = parse::parse_transform(v);
            state.transform = state.transform.multiply(&t);
        }
    }

    // 2. Style (Presentation Attributes)
    // Precedence: explicit attr > style string (unsupported in v1 parse) > inherited
    for (k, v) in &attrs {
        match k.as_str() {
            "fill" => state.fill = parse::parse_color(v).map(Paint::Solid),
            "stroke" => state.stroke = parse::parse_color(v).map(Paint::Solid),
            "opacity" => {
                if let Ok(o) = v.parse::<f32>() {
                    state.opacity *= o
                }
            }
            "stroke-width" => {
                if let Ok(w) = v.parse::<f32>() {
                    state.stroke_width = w
                }
            }
            // "style" => ... parse style string
            _ => {}
        }
    }

    let tag = graph.get_prop_str(node, props::TAG).unwrap_or_default();

    match tag.as_str() {
        "svg" | "g" => {
            // Container, recurse
            let children = graph.get_children(node);
            for child in children {
                // If text node, skip? Or handle text?
                // For now skip non-element check (rely on TAG assumption or kind check)
                compile_element(graph, child, &state, ops)?;
            }
        }
        "rect" => {
            // parse x, y, width, height
            let mut x = 0.0;
            let mut y = 0.0;
            let mut w = 0.0;
            let mut h = 0.0;
            for (k, v) in &attrs {
                match k.as_str() {
                    "x" => x = v.parse().unwrap_or(0.0),
                    "y" => y = v.parse().unwrap_or(0.0),
                    "width" => w = v.parse().unwrap_or(0.0),
                    "height" => h = v.parse().unwrap_or(0.0),
                    _ => {}
                }
            }
            use crate::svg::ir::PointF;
            let path = Path2D {
                verbs: alloc::vec![
                    PathCommand::MoveTo(PointF { x, y }),
                    PathCommand::LineTo(PointF { x: x + w, y }),
                    PathCommand::LineTo(PointF { x: x + w, y: y + h }),
                    PathCommand::LineTo(PointF { x, y: y + h }),
                    PathCommand::Close
                ],
            };
            emit_shape(path, &state, ops);
        }
        "path" => {
            // parse d
            if let Some((_, d)) = attrs.iter().find(|(k, _)| k == "d") {
                let path = parse::parse_path_d(d);
                emit_shape(path, &state, ops);
            }
        }
        // ... circle, etc.
        _ => {}
    }

    Ok(())
}

fn emit_shape(path: Path2D, state: &StyleState, ops: &mut Vec<SvgOp>) {
    let path_arc = alloc::sync::Arc::new(path);
    // Fill
    if let Some(paint) = &state.fill {
        ops.push(SvgOp::FillPath {
            path: path_arc.clone(),
            paint: paint.clone(),
            transform: state.transform,
            fill_rule: state.fill_rule,
            opacity: state.opacity,
        });
    }

    // Stroke
    if let Some(paint) = &state.stroke {
        ops.push(SvgOp::StrokePath {
            path: path_arc,
            paint: paint.clone(),
            transform: state.transform,
            width: state.stroke_width,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 4.0,
            opacity: state.opacity,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::collections::BTreeMap;
    use alloc::string::ToString;
    use stem::thing::ThingId;
    use stem::xml::ingest::{ingest_xml_to_graph, GraphApply, XmlIngestError, XmlIngestOptions};

    // A graph that supports both Ingestion (Write) and SvgGraph (Read)
    struct TestGraph {
        nodes: u64,
        edges: Vec<(ThingId, String, ThingId)>,
        props: BTreeMap<(ThingId, String), String>,
        interner: BTreeMap<String, u64>,
        rev_interner: BTreeMap<u64, String>,
    }

    impl TestGraph {
        fn new() -> Self {
            Self {
                nodes: 0,
                edges: Vec::new(),
                props: BTreeMap::new(),
                interner: BTreeMap::new(),
                rev_interner: BTreeMap::new(),
            }
        }
    }

    impl GraphApply for TestGraph {
        fn create_node(&mut self, _kind: &str) -> Result<ThingId, XmlIngestError> {
            self.nodes += 1;
            Ok(ThingId::from_u64(self.nodes))
        }

        fn link(&mut self, src: ThingId, rel: &str, dst: ThingId) -> Result<(), XmlIngestError> {
            self.edges.push((src, rel.to_string(), dst));
            Ok(())
        }

        fn set_prop(&mut self, node: ThingId, key: &str, val: u64) -> Result<(), XmlIngestError> {
            // We store the resolved string value if possible, or just the ID string if we can't lookup?
            // Wait, ingest interns strings, then calls set_prop with u64.
            // But SvgGraph::get_prop_str expects to return String.
            // We need to resolve the u64 val back to string.
            // We have rev_interner.
            if let Some(s) = self.rev_interner.get(&val) {
                self.props.insert((node, key.to_string()), s.clone());
            } else {
                // Fallback (shouldn't happen in test flow)
                self.props
                    .insert((node, key.to_string()), format!("{}", val));
            }
            Ok(())
        }

        fn intern(&mut self, s: &str) -> Result<u64, XmlIngestError> {
            if let Some(&id) = self.interner.get(s) {
                Ok(id)
            } else {
                let id = self.interner.len() as u64 + 1;
                self.interner.insert(s.to_string(), id);
                self.rev_interner.insert(id, s.to_string());
                Ok(id)
            }
        }
    }

    impl SvgGraph for TestGraph {
        fn get_root(&self, doc: ThingId) -> Option<ThingId> {
            self.edges
                .iter()
                .find(|(s, r, _)| *s == doc && r == "HAS_ROOT")
                .map(|(_, _, d)| *d)
        }

        fn get_children(&self, elem: ThingId) -> Vec<ThingId> {
            self.edges
                .iter()
                .filter(|(s, r, _)| *s == elem && r == "HAS_CHILD")
                .map(|(_, _, d)| *d)
                .collect()
        }

        fn get_prop_str(&self, node: ThingId, key: &str) -> Option<String> {
            self.props.get(&(node, key.to_string())).cloned()
        }

        fn get_attributes(&self, elem: ThingId) -> Vec<(String, String)> {
            let mut res = Vec::new();
            // Find HAS_ATTR edges
            let attr_nodes: Vec<ThingId> = self
                .edges
                .iter()
                .filter(|(s, r, _)| *s == elem && r == "HAS_ATTR")
                .map(|(_, _, d)| *d)
                .collect();

            for attr in attr_nodes {
                let name = self.get_prop_str(attr, "attr_name");
                let val = self.get_prop_str(attr, "attr_value");
                if let (Some(n), Some(v)) = (name, val) {
                    res.push((n, v));
                }
            }
            res
        }
    }

    #[test]
    fn test_compile_rect() {
        let xml = r#"<svg width="100" height="100"><rect x="10" y="10" width="80" height="80" fill="red" /></svg>"#;
        let mut graph = TestGraph::new();
        let res = ingest_xml_to_graph(xml.as_bytes(), XmlIngestOptions::default(), &mut graph)
            .expect("ingest");

        // Compile
        let ir = compile_graph_to_ir(&graph, res.document).expect("compile");

        assert_eq!(ir.ops.len(), 1);
        match &ir.ops[0] {
            SvgOp::FillPath { paint, .. } => {
                if let Paint::Solid(c) = paint {
                    assert_eq!(c.r, 255);
                    assert_eq!(c.g, 0);
                } else {
                    panic!("Wrong paint");
                }
            }
            _ => panic!("Wrong op"),
        }
    }

    #[test]
    fn test_compile_transforms_nesting() {
        let xml = r#"<svg><g transform="translate(10, 20)"><g transform="scale(2)"><rect width="10" height="10" /></g></g></svg>"#;
        let mut graph = TestGraph::new();
        let res = ingest_xml_to_graph(xml.as_bytes(), XmlIngestOptions::default(), &mut graph)
            .expect("ingest");

        let ir = compile_graph_to_ir(&graph, res.document).expect("compile");
        assert_eq!(ir.ops.len(), 1);

        match &ir.ops[0] {
            SvgOp::FillPath { transform, .. } => {
                // Expected: translate(10, 20) -> scale(2)
                // Matrix: m11=2, m22=2, m12=0, m21=0, dx=10, dy=20 ?
                // Multiply order: T1 * T2?
                // translate(10, 20) applied first (outer), then scale(2) (inner).
                // Point p -> T_outer * T_inner * p
                // dx should be 10.
                assert_eq!(transform.dx, 10.0);
                assert_eq!(transform.m11, 2.0); // Wait, scale logic in parse helper needs verify
            }
            _ => panic!("Wrong op"),
        }
    }

    #[test]
    fn test_compile_float_viewbox() {
        let xml = r#"<svg viewBox="0 0 24.5 24.5"><rect width="24.5" height="24.5" /></svg>"#;
        let mut graph = TestGraph::new();
        let res = ingest_xml_to_graph(xml.as_bytes(), XmlIngestOptions::default(), &mut graph)
            .expect("ingest");

        let ir = compile_graph_to_ir(&graph, res.document).expect("compile");
        
        if let Some(vb) = ir.view_box {
            assert_eq!(vb.x(), 0.0);
            assert_eq!(vb.y(), 0.0);
            assert_eq!(vb.width(), 24.5);
            assert_eq!(vb.height(), 24.5);
        } else {
            panic!("Missing viewBox");
        }
    }
}
