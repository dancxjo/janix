use crate::*;



#[derive(Debug)]
pub struct EventView {
    pub event_id: ThingId,
    pub target: Option<ThingId>,
    pub rel_from: Option<ThingId>,
    pub rel_to: Option<ThingId>,
    pub rel_predicate: Option<SymbolId>,
}

pub fn decode_event(event_id: ThingId) -> EventView {
    let mut view = EventView {
        event_id,
        target: None,
        rel_from: None,
        rel_to: None,
        rel_predicate: None,
    };

    let mut rels_buf = [ThingId(0); 32];
    let count = relationships_from_into(event_id, &mut rels_buf);
    let rels = &rels_buf[0..count];
    
    let pred_targets = symbol_intern("predicate.targets");

    for &r_id in rels {
        let mut buf = [0u8; 40];
        // Check if this relationship is "predicate.targets"
        if thing_get_payload(r_id, &mut buf) >= 40 {
            // buf contains [from, to, pred] of the RELATIONSHIP (r_id)
            // r_id is (event --targets--> target)
            // from=event, to=target, pred=targets
            
            let pred_bytes = &buf[32..40];
            let pred = SymbolId(u64::from_le_bytes(pred_bytes.try_into().unwrap()));
            
            if pred == pred_targets {
                let to_bytes = &buf[16..32];
                let target_id = ThingId(u128::from_le_bytes(to_bytes.try_into().unwrap()));
                
                view.target = Some(target_id);

                let mut t_buf = [0u8; 40];
                let p_len = thing_get_payload(target_id, &mut t_buf);
                if p_len >= 40 {
                   // ...
                   let r_from = ThingId(u128::from_le_bytes(t_buf[0..16].try_into().unwrap()));
                   let r_to = ThingId(u128::from_le_bytes(t_buf[16..32].try_into().unwrap()));
                   let r_pred = SymbolId(u64::from_le_bytes(t_buf[32..40].try_into().unwrap()));
                   
                   view.rel_from = Some(r_from);
                   view.rel_to = Some(r_to);
                   view.rel_predicate = Some(r_pred);
                }
            }
        }
    }
    view
}
