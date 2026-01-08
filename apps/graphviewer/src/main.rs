#![no_std]
#![no_main]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use thing_std::*;
use models::*;
use thing_codec::GraphClient;

/// Generate a short OpenGQL-ish summary of the current graph state.
/// This traverses from graph.windows and summarizes what it finds.
fn summarize_graph_short(_client: &SyscallGraphClient) -> String {
    use core::fmt::Write;
    
    let mut summary = String::with_capacity(512);
    
    // Try to find graph.windows as our entry point
    let windows_graph = thing_find("graph.windows");
    
    // Count nodes and edges we can see
    let mut node_count: u32 = 0;
    let mut edge_count: u32 = 0;
    let mut kinds: Vec<SymbolId> = Vec::new();
    
    if let Some(graph_id) = windows_graph {
        // Get windows contained in graph.windows
        let rel_contains = symbol_intern("predicate.contains");
        let windows = graph_outgoing(graph_id, rel_contains);
        
        node_count += 1; // graph.windows itself
        edge_count += windows.len() as u32;
        
        for window_id in windows.iter().take(8) {
            node_count += 1;
            
            // Get window kind
            if let Some(header) = get_thing_header(*window_id) {
                if !kinds.contains(&header.kind) && kinds.len() < 8 {
                    kinds.push(header.kind);
                }
            }
            
            // Check for content_root relationship
            let rel_content = symbol_intern("content_root");
            let content_roots = graph_outgoing(*window_id, rel_content);
            edge_count += content_roots.len() as u32;
            
            for layout_id in content_roots.iter().take(4) {
                node_count += 1;
                
                if let Some(header) = get_thing_header(*layout_id) {
                    if !kinds.contains(&header.kind) && kinds.len() < 8 {
                        kinds.push(header.kind);
                    }
                }
                
                // Check for child relationships
                let rel_child = symbol_intern("child");
                let children = graph_outgoing(*layout_id, rel_child);
                edge_count += children.len() as u32;
                
                for child_id in children.iter().take(8) {
                    node_count += 1;
                    
                    if let Some(header) = get_thing_header(*child_id) {
                        if !kinds.contains(&header.kind) && kinds.len() < 8 {
                            kinds.push(header.kind);
                        }
                    }
                }
            }
        }
    }
    
    // Format as OpenGQL-ish output
    let _ = writeln!(summary, "-- GraphViewer Summary --");
    let _ = writeln!(summary, "MATCH (n) RETURN count(n) AS nodes;");
    let _ = writeln!(summary, "-- Result: {} visible nodes", node_count);
    let _ = writeln!(summary);
    let _ = writeln!(summary, "MATCH ()-[r]->() RETURN count(r) AS rels;");
    let _ = writeln!(summary, "-- Result: {} visible edges", edge_count);
    let _ = writeln!(summary);
    let _ = writeln!(summary, "MATCH (n) RETURN labels(n) LIMIT 8;");
    let _ = write!(summary, "-- Kinds: ");
    
    for (i, kind) in kinds.iter().enumerate() {
        if i > 0 {
            let _ = write!(summary, ", ");
        }
        // Try to resolve symbol name
        if let Some(name_bytes) = symbol_resolve(*kind) {
            if let Ok(name) = core::str::from_utf8(&name_bytes) {
                let _ = write!(summary, "{}", name);
            } else {
                let _ = write!(summary, "sym:{}", kind.0);
            }
        } else {
            let _ = write!(summary, "sym:{}", kind.0);
        }
    }
    let _ = writeln!(summary);
    
    summary
}

#[no_mangle]
pub fn main() {
    thing_std::init(0);
    log_info("GraphViewer Starting...");

    let mut client = SyscallGraphClient;

    // 1. Create Layout (Column: intro label, output label, button)
    let kind_layout = symbol_intern("kind.Layout");
    let layout_id = client.create_thing(kind_layout).expect("create layout");
    let layout = Layout {
        kind: LayoutKind::Column,
        padding: 16,
        gap: 12,
        align: 0, 
    };
    layout.write(&mut client, layout_id).expect("save layout");

    // 2. Create Window
    let kind_window = symbol_intern("kind.Window");
    let window_id = client.create_thing(kind_window).expect("create window");
    
    let window = Window {
        title: symbol_intern("GraphViewer"),
        x: 80,
        y: 80,
        width: 520,
        height: 340,
        z: 0,
        focused: false,
        min_width: 300,
        min_height: 200,
        style: WindowStyle {
            bg_rgba: 0xF8F8F8FF, 
            radius: 12,
            shadow: 1, 
            elevation: 2,
        },
        content_root: layout_id,
    };
    window.write(&mut client, window_id).expect("save window");

    // 3. Create Widgets
    
    // Intro label
    let kind_label = symbol_intern("kind.Label");
    let intro_label_id = client.create_thing(kind_label).expect("create intro label");
    let intro_label = Label {
        text: symbol_intern("Welcome. Press the button to show the current graph (short OpenGQL)."),
        style: TextStyle { size: 14, color_rgba: 0x333333FF },
    };
    intro_label.write(&mut client, intro_label_id).expect("save intro label");

    // Output label (starts with placeholder)
    let output_label_id = client.create_thing(kind_label).expect("create output label");
    let output_label = Label {
        text: symbol_intern("(graph not loaded yet)"),
        style: TextStyle { size: 12, color_rgba: 0x666666FF },
    };
    output_label.write(&mut client, output_label_id).expect("save output label");

    // Show current graph button
    let kind_button = symbol_intern("kind.Button");
    let button_id = client.create_thing(kind_button).expect("create button");
    let button = Button {
        text: symbol_intern("Show current graph"),
        style: ButtonStyle { bg_rgba: 0x4A90D9FF, radius: 6 },
    };
    button.write(&mut client, button_id).expect("save button");

    // 4. Links
    let rel_content = symbol_intern("content_root");
    relationship_create(rel_content, window_id, layout_id);

    let rel_child = symbol_intern("child");
    relationship_create(rel_child, layout_id, intro_label_id);
    relationship_create(rel_child, layout_id, output_label_id);
    relationship_create(rel_child, layout_id, button_id);

    // 5. Publish to graph.windows
    let graph_windows = thing_find("graph.windows").unwrap_or_else(|| {
        let gid = thing_create(symbol_intern("kind.Graph"), ThingId::from_parts(0,0));
        thing_register_name(gid, "graph.windows");
        gid
    });
    
    let rel_contains = symbol_intern("predicate.contains");
    relationship_create(rel_contains, graph_windows, window_id);

    log_info("GraphViewer Published!");

    // 6. Pulse initial Frame
    let kind_frame = symbol_intern("kind.Frame");
    let frame_id = client.create_thing(kind_frame).expect("create frame");
    let frame = Frame {
        window: window_id,
        seq: 1,
    };
    frame.write(&mut client, frame_id).expect("save frame");
    
    let rel_has_frame = symbol_intern("has_frame");
    relationship_create(rel_has_frame, window_id, frame_id);

    // 7. Main loop: poll for button clicks and update output
    let mut frame_seq: u32 = 2;
    
    loop {
        // Check for button click event
        // For now, we simulate a click check every second
        // In a real implementation, this would use event watches
        thing_std::time::sleep_ms(1000);
        
        // Check if button was clicked by reading its state
        // This is a simplified approach - real implementation would use event system
        if let Ok(_btn) = Button::read(&client, button_id) {
            // For now, we refresh on every loop iteration as a demonstration
            // A proper implementation would use event-driven updates
            
            // Generate graph summary
            let summary_text = summarize_graph_short(&client);
            log_info("GraphViewer: Updating graph summary...");
            
            // Update output label with summary
            let summary_sym = symbol_intern(&summary_text);
            let updated_output = Label {
                text: summary_sym,
                style: TextStyle { size: 12, color_rgba: 0x222222FF },
            };
            updated_output.write(&mut client, output_label_id).expect("update output");
            
            // Pulse new frame to trigger re-render
            let new_frame = Frame {
                window: window_id,
                seq:frame_seq as u64,
            };
            new_frame.write(&mut client, frame_id).expect("update frame");
            frame_seq = frame_seq.wrapping_add(1);
            
            // Only update once per boot for now (to avoid poll spam)
            // Wait longer before next potential update
            thing_std::time::sleep_ms(9000);
        }
    }
}
