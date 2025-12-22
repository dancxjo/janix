use abi::ThingId;
use thing_models::{AlarmRequest, PropValue};

#[test]
fn test_alarm_request_create_pending() {
    let owner_process = ThingId(100);
    let owner_thread = ThingId(200);
    let props = AlarmRequest::create_pending(123456789, 500, owner_process, owner_thread);

    let props_map: std::collections::HashMap<String, PropValue> =
        props.into_iter().map(|(k, v)| (k, v)).collect();

    assert_eq!(
        props_map.get("target_unix_seconds"),
        Some(&PropValue::I64(123456789))
    );
    assert_eq!(
        props_map.get("target_unix_nanos"),
        Some(&PropValue::U64(500))
    );
    assert_eq!(props_map.get("owner_process"), Some(&PropValue::U64(100)));
    assert_eq!(props_map.get("owner_thread"), Some(&PropValue::U64(200)));
    assert_eq!(props_map.get("armed"), Some(&PropValue::Bool(false)));
    assert_eq!(props_map.get("fired"), Some(&PropValue::Bool(false)));
}
