use abi::ids::ThingId;
use abi::wire::typed::TypedBytes;
use thing_models::link::LinkBody;
use thing_models::Thing;

// Generic helper to collect links from an iterator.
// This allows us to avoid boxing the iterator in handle_graph_op.
// Returns nothing, but appends to results.
pub fn collect_links<'a, I>(
    iter: I,
    target_to: Option<ThingId>,
    results: &mut alloc::vec::Vec<(ThingId, ThingId, ThingId)>,
) where
    I: Iterator<Item = &'a Thing>,
{
    for thing in iter {
        if let Ok(tb) = thing.body.decode::<TypedBytes>() {
            if let Ok(link) = postcard::from_bytes::<LinkBody>(&tb.bytes) {
                if let Some(to) = target_to {
                    if link.to != to {
                        continue;
                    }
                }
                results.push((link.from, link.to, link.predicate));
            }
        }
    }
}
