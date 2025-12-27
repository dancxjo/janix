use models::prelude::*;
use models::*;

#[test]
fn test_log_builder() {
    let l = LogEntryBody::new(LogLevel::Warn, sym!("test"), "message")
        .ts(100)
        .cpu(1)
        .thread(2)
        .process(3)
        .seq(4);

    assert_eq!(l.level, LogLevel::Warn);
    assert_eq!(l.timestamp_ns, 100);
    assert_eq!(l.cpu_id, 1);
}

#[test]
fn test_macros() {
    let t = thing!(
        id: ThingId(1),
        kind: ThingId(2),
        body: LogEntryBody::new(LogLevel::Info, sym!("sys"), "ok"),
    );
    assert_eq!(t.id.0, 1);

    // Test link macro
    let l = link!(ThingId(1), ThingId(2), ThingId(3));
    assert_eq!(l.from.0, 1);
    assert_eq!(l.to.0, 3);
    assert_eq!(l.predicate.0, 2);
}

#[test]
fn test_builders() {
    let log = LogEntryBody::new(LogLevel::Error, sym!("kern"), "panic");
    let t = Thing::log(ThingId(10), &log);
    assert_eq!(t.kind, THING_LOG_ENTRY_KIND);
}
