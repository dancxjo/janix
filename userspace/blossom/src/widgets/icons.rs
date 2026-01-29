use abi::schema::kinds;
use stem::petals::builder::Node;
use stem::petals::{Icon, Styled};

// Correct kind names from abi::schema::kinds
// Some kinds might not have constants yet, so we use string literals if needed
// or just skip them if they aren't critical for the visualizer right now.

pub struct ThingosIcon;

impl ThingosIcon {
    pub fn new(name: &str) -> Icon {
        Icon::new(name)
    }

    pub fn for_kind(kind: &str) -> Icon {
        let name = match kind {
            kinds::UI_CROWN => "ui.root",
            kinds::UI_WINDOW => "ui.widget",
            kinds::PROC_KERNEL => "proc.kernel",
            // kinds::PROC_TASK not found, using string literal or fallback
            "proc.Task" => "proc.task",
            "proc.Thread" => "proc.thread",
            "mem.Page" => "mem.page",
            "mem.Stack" => "mem.stack",
            "mem.Heap" => "mem.heap",
            kinds::BOOT_MODULE => "bran.bran",
            kinds::SVC_SCHEDULER => "svc.scheduler",
            "svc.Init" => "svc.init",
            "svc.Cambium" => "svc.cambium",
            kinds::CLOCK => "time.clock",
            _ => {
                if kind.starts_with("svc.") {
                    "svc.service"
                } else if kind.starts_with("dev.") {
                    "dev.host"
                } else if kind.starts_with("meta.") {
                    "meta.graph"
                } else {
                    "ui.widget"
                }
            }
        };
        Icon::new(name)
    }
}

pub struct TangoIcon;

impl TangoIcon {
    pub fn new(category: &str, name: &str) -> Icon {
        let mut full_name = alloc::string::String::from("tango/");
        full_name.push_str(category);
        full_name.push('/');
        full_name.push_str(name);
        Icon::new(&full_name)
    }

    pub fn apps(name: &str) -> Icon {
        Self::new("apps", name)
    }

    pub fn actions(name: &str) -> Icon {
        Self::new("actions", name)
    }

    pub fn categories(name: &str) -> Icon {
        Self::new("categories", name)
    }

    pub fn devices(name: &str) -> Icon {
        Self::new("devices", name)
    }

    pub fn emotes(name: &str) -> Icon {
        Self::new("emotes", name)
    }

    pub fn mimetypes(name: &str) -> Icon {
        Self::new("mimetypes", name)
    }

    pub fn places(name: &str) -> Icon {
        Self::new("places", name)
    }

    pub fn status(name: &str) -> Icon {
        Self::new("status", name)
    }

    /// Legacy mapping for photosynthesis nodes if they want Tango style
    pub fn for_kind(kind: &str) -> Icon {
        let name = match kind {
            kinds::BOOT_MODULE => "system-software-update",
            _ => "utilities-system-monitor",
        };
        Self::apps(name)
    }
}
