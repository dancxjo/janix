use crate::thing_kind;
use crate::declare::type_tag::fnv1a64;
use crate::declare::type_tag;
use crate::builtins::ids::*;
use crate::builtins::symbols::*;
// Use the core module types
use crate::core::time::TimeNow;
use crate::core::process::{ProcessBody, ThreadBody};
use crate::core::capability::CapabilityBody;
use crate::core::vgs::{GraphBody, MountBody, GraphProviderBody};
use crate::core::buffer::{BufferBody, StreamBody};
use crate::core::input::{KeyboardBody, KeyEventBody};


thing_kind! {
    kind TimeNow {
        id: crate::builtins::ids::THING_TIME_NOW_KIND,
        sym: SYM_TIME_NOW,
        version: 1,
        body: TimeNow,
        type_tag: "thingos.TimeNow.v1",
        schema_id: crate::builtins::ids::THING_TIME_NOW_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Process {
        id: crate::builtins::ids::THING_PROCESS_KIND,
        sym: SYM_PROCESS,
        version: 1,
        body: ProcessBody,
        type_tag: "thingos.ProcessBody.v1",
        schema_id: crate::builtins::ids::THING_PROCESS_SCHEMA,
        links {
            predicate THING_OWNS_KIND min 0 max many;
            predicate THING_HAS_CAP_KIND min 0 max many;
        }
    }
}

thing_kind! {
    kind Thread {
        id: crate::builtins::ids::THING_THREAD_KIND,
        sym: SYM_THREAD,
        version: 1,
        body: ThreadBody,
        type_tag: "thingos.ThreadBody.v1",
        schema_id: crate::builtins::ids::THING_THREAD_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Capability {
        id: crate::builtins::ids::THING_CAPABILITY_KIND,
        sym: SYM_CAPABILITY,
        version: 1,
        body: CapabilityBody,
        type_tag: "thingos.CapabilityBody.v1",
        schema_id: crate::builtins::ids::THING_CAPABILITY_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Graph {
        id: crate::builtins::ids::THING_GRAPH_KIND,
        sym: SYM_GRAPH,
        version: 1,
        body: GraphBody,
        type_tag: "thingos.GraphBody.v1",
        schema_id: crate::builtins::ids::THING_GRAPH_SCHEMA,
        links {
            predicate THING_BACKED_BY_KIND min 0 max 1;
        }
    }
}

thing_kind! {
    kind Mount {
        id: crate::builtins::ids::THING_MOUNT_KIND,
        sym: SYM_MOUNT,
        version: 1,
        body: MountBody,
        type_tag: "thingos.MountBody.v1",
        schema_id: crate::builtins::ids::THING_MOUNT_SCHEMA,
        links {
            predicate THING_MOUNTS_KIND min 1 max 1;
        }
    }
}

thing_kind! {
    kind GraphProvider {
        id: crate::builtins::ids::THING_GRAPH_PROVIDER_KIND,
        sym: SYM_GRAPH_PROVIDER,
        version: 1,
        body: GraphProviderBody,
        type_tag: "thingos.GraphProviderBody.v1",
        schema_id: crate::builtins::ids::THING_GRAPH_PROVIDER_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Buffer {
        id: crate::builtins::ids::THING_BUFFER_KIND,
        sym: SYM_BUFFER,
        version: 1,
        body: BufferBody,
        type_tag: "thingos.BufferBody.v1",
        schema_id: crate::builtins::ids::THING_BUFFER_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Stream {
        id: crate::builtins::ids::THING_STREAM_KIND,
        sym: SYM_STREAM,
        version: 1,
        body: StreamBody,
        type_tag: "thingos.StreamBody.v1",
        schema_id: crate::builtins::ids::THING_STREAM_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Keyboard {
        id: crate::builtins::ids::THING_KEYBOARD_KIND,
        sym: SYM_KEYBOARD,
        version: 1,
        body: KeyboardBody,
        type_tag: "thingos.KeyboardBody.v1",
        schema_id: crate::builtins::ids::THING_KEYBOARD_SCHEMA,
        links {} // meta (0..1) implied by v0 rules
    }
}

thing_kind! {
    kind KeyEvent {
        id: crate::builtins::ids::THING_KEY_EVENT_KIND,
        sym: SYM_KEY_EVENT,
        version: 1,
        body: KeyEventBody,
        type_tag: "thingos.KeyEventBody.v1",
        schema_id: crate::builtins::ids::THING_KEY_EVENT_SCHEMA,
        links {} // meta (0..1) implied by v0 rules
    }
}

