use abi::{ThingId, PropKey, PropValue, PropType};
use thing_macros::Thing;

// Manual Thing implementation
pub struct ManualCounter {
    pub count: u64,
    pub active: bool,
}

impl userland_std::Thing for ManualCounter {
    const KIND: &'static str = "demo.ManualCounter";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("count", PropValue::U64(self.count)));
        out.push(("active", PropValue::Bool(self.active)));
    }

    fn from_props(_id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut count = 0;
        let mut active = false;

        for prop in props {
            if let Some((k, v)) = prop {
                match (*k, v) {
                    ("count", PropValue::U64(c)) => count = *c,
                    ("active", PropValue::Bool(a)) => active = *a,
                    _ => {}
                }
            }
        }

        ManualCounter { count, active }
    }

    fn schema() -> &'static [(&'static PropKey, PropType)] {
        static SCHEMA: &[(&'static PropKey, PropType)] = &[
            (&"count", PropType::U64),
            (&"active", PropType::Bool),
        ];
        SCHEMA
    }
}

// Derived Thing implementation
#[derive(Thing)]
pub struct AutoCounter {
    pub count: u64,
    pub active: bool,
}
