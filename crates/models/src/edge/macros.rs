use crate::Thing;
use crate::LinkBody;
use crate::ThingBody;
use crate::builtins::ids::THING_LINK_KIND;

#[macro_export]
macro_rules! link_thing {
    (
      id: $id:expr,
      from: $from:expr,
      to: $to:expr,
      pred: $pred:expr $(,)?
    ) => {{
        $crate::Thing {
            id: $id,
            kind: $crate::builtins::ids::THING_LINK_KIND,
            body: $crate::ThingBody::from(
                &$crate::LinkBody {
                    from: $from,
                    to: $to,
                    predicate: $pred,
                }
            ).expect("link_thing!: LinkBody encode failed"),
        }
    }};
}
