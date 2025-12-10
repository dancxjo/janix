#![no_std]

use userland::prelude::*;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "alarm_demo: starting");

    let Some(clock) = SystemClock::discover(sys) else {
        println(sys, "alarm_demo: TimeSource not found");
        sys.exit_thread();
    };

    let (now_secs, now_nanos) = clock.now(sys);
    let target_secs = now_secs.saturating_add(5);
    let Some(alarm) = Alarm::request_at(sys, target_secs, now_nanos) else {
        println(sys, "alarm_demo: failed to create AlarmRequest");
        sys.exit_thread();
    };

    let (hour, minute, second) = seconds_to_hms(target_secs);
    log_dynamic(
        sys,
        format_args!(
            "alarm_demo: waiting for alarm at {:02}:{:02}:{:02}",
            hour, minute, second
        ),
    );

    loop {
        if let Some(state) = alarm.state(sys) {
            if state == "Fired" {
                let (fired_secs, _) = clock.now(sys);
                let (fh, fm, fs) = seconds_to_hms(fired_secs);
                log_dynamic(
                    sys,
                    format_args!("alarm_demo: alarm fired at {:02}:{:02}:{:02}", fh, fm, fs),
                );
                break;
            } else if state == "Cancelled" {
                println(sys, "alarm_demo: alarm cancelled");
                break;
            }
        }
        sys.yield_now();
    }

    sys.exit_thread();
}

fn seconds_to_hms(seconds: i64) -> (u32, u32, u32) {
    let secs_in_day = 86_400_i64;
    let mut secs = seconds.rem_euclid(secs_in_day);
    if secs < 0 {
        secs += secs_in_day;
    }
    let hour = (secs / 3_600) as u32;
    secs %= 3_600;
    let minute = (secs / 60) as u32;
    let second = (secs % 60) as u32;
    (hour, minute, second)
}

#[cfg(test)]
mod tests {
    use super::seconds_to_hms;
    use abi::{KernelRequest, KernelResponse, PropValue, ThingId};
    use userland::prelude::*;
    use userland_std::{AlarmRequest, alarm::Alarm, doc_helpers::DocSys};

    #[test]
    fn formats_alarm_target_times() {
        assert_eq!(seconds_to_hms(7 * 3_600 + 5 * 60 + 9), (7, 5, 9));
        assert_eq!(seconds_to_hms(86_399), (23, 59, 59));
    }

    #[test]
    fn alarm_request_records_target() {
        let mut responses = Vec::new();
        responses.push(KernelResponse::ThingCreated { id: ThingId(100) });
        let mut sys = DocSys::with_responses(responses);
        let alarm = Alarm::request_at(&mut sys, 42, 5).expect("alarm creation failed");
        assert_eq!(alarm.id, ThingId(100));
        let requests = sys.requests.borrow();
        match requests.last().expect("missing request") {
            KernelRequest::ThingCreate { kind, props } => {
                assert_eq!(*kind, AlarmRequest::KIND);
                let target_secs = props
                    .iter()
                    .find(|(key, _)| *key == "target_unix_seconds")
                    .and_then(|(_, value)| match value {
                        PropValue::I64(v) => Some(*v),
                        _ => None,
                    });
                assert_eq!(target_secs, Some(42));
            }
            other => panic!("unexpected request: {:?}", other),
        }
    }

    #[test]
    fn alarm_state_roundtrip_queries_kernel() {
        let mut props_vec = Vec::new();
        props_vec.push(("state", PropValue::Str(String::from("Fired"))));
        let props = DocSys::props_slice(props_vec);
        let mut responses = Vec::new();
        responses.push(KernelResponse::ThingData {
            id: ThingId(77),
            kind: AlarmRequest::KIND,
            props,
        });
        let mut sys = DocSys::with_responses(responses);
        let alarm = Alarm { id: ThingId(77) };
        assert_eq!(alarm.state(&mut sys), Some(String::from("Fired")));
        let requests = sys.requests.borrow();
        match requests.last().expect("missing request") {
            KernelRequest::ThingGet { id } => assert_eq!(id.0, 77),
            other => panic!("unexpected request: {:?}", other),
        }
    }
}
