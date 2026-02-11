use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Default)]
pub struct InstanceState {
    pub last_seen: u64,
    pub expires_at: u64,
    pub stale: bool,
    pub saw_ptr: bool,
    pub saw_srv: bool,
    pub saw_txt: bool,
}

#[derive(Debug, Default)]
pub struct DiscoveryState {
    instances: BTreeMap<String, InstanceState>,
}

impl DiscoveryState {
    pub fn observe_ptr(&mut self, instance_fqdn: &str, now_s: u64, ttl_s: u32) {
        self.touch(instance_fqdn, now_s, ttl_s, |s| s.saw_ptr = true);
    }

    pub fn observe_srv(&mut self, instance_fqdn: &str, now_s: u64, ttl_s: u32) {
        self.touch(instance_fqdn, now_s, ttl_s, |s| s.saw_srv = true);
    }

    pub fn observe_txt(&mut self, instance_fqdn: &str, now_s: u64, ttl_s: u32) {
        self.touch(instance_fqdn, now_s, ttl_s, |s| s.saw_txt = true);
    }

    fn touch<F: FnOnce(&mut InstanceState)>(
        &mut self,
        instance_fqdn: &str,
        now_s: u64,
        ttl_s: u32,
        f: F,
    ) {
        let item = self.instances.entry(instance_fqdn.into()).or_default();
        item.last_seen = now_s;
        item.expires_at = now_s.saturating_add(ttl_s as u64);
        item.stale = false;
        f(item);
    }

    pub fn sweep(&mut self, now_s: u64) -> Vec<String> {
        let mut stale = Vec::new();
        for (name, state) in self.instances.iter_mut() {
            if !state.stale && state.expires_at <= now_s {
                state.stale = true;
                stale.push(name.clone());
            }
        }
        stale
    }

    pub fn instance_count(&self) -> usize {
        self.instances.len()
    }

    pub fn instance(&self, name: &str) -> Option<&InstanceState> {
        self.instances.get(name)
    }
}

pub fn reconcile_desired(desired: &[String], active: &[String]) -> (Vec<String>, Vec<String>) {
    let desired_set: BTreeSet<_> = desired.iter().cloned().collect();
    let active_set: BTreeSet<_> = active.iter().cloned().collect();

    let publish = desired_set
        .difference(&active_set)
        .cloned()
        .collect::<Vec<_>>();
    let unpublish = active_set
        .difference(&desired_set)
        .cloned()
        .collect::<Vec<_>>();

    (publish, unpublish)
}

#[cfg(test)]
mod tests {
    use super::{reconcile_desired, DiscoveryState};
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn ttl_expiry_marks_instance_stale() {
        let mut state = DiscoveryState::default();
        state.observe_ptr("printer._ipp._tcp.local", 100, 1);
        assert!(!state.instance("printer._ipp._tcp.local").unwrap().stale);

        let stale = state.sweep(102);
        assert_eq!(stale, vec!["printer._ipp._tcp.local".to_string()]);
        assert!(state.instance("printer._ipp._tcp.local").unwrap().stale);
    }

    #[test]
    fn merge_updates_do_not_duplicate_instance() {
        let mut state = DiscoveryState::default();
        state.observe_ptr("svc._http._tcp.local", 10, 120);
        state.observe_srv("svc._http._tcp.local", 11, 120);
        state.observe_txt("svc._http._tcp.local", 12, 120);

        assert_eq!(state.instance_count(), 1);
        let item = state.instance("svc._http._tcp.local").unwrap();
        assert!(item.saw_ptr);
        assert!(item.saw_srv);
        assert!(item.saw_txt);
    }

    #[test]
    fn churn_updates_last_seen_without_new_nodes() {
        let mut state = DiscoveryState::default();
        state.observe_ptr("tv._airplay._tcp.local", 50, 30);
        state.observe_ptr("tv._airplay._tcp.local", 55, 30);

        assert_eq!(state.instance_count(), 1);
        let item = state.instance("tv._airplay._tcp.local").unwrap();
        assert_eq!(item.last_seen, 55);
        assert_eq!(item.expires_at, 85);
    }

    #[test]
    fn desired_state_apply_calculates_publish_and_unpublish() {
        let desired = vec!["a._http._tcp.local".to_string()];
        let active = vec![];
        let (publish, unpublish) = reconcile_desired(&desired, &active);
        assert_eq!(publish, vec!["a._http._tcp.local".to_string()]);
        assert!(unpublish.is_empty());

        let desired = vec![];
        let active = vec!["a._http._tcp.local".to_string()];
        let (publish, unpublish) = reconcile_desired(&desired, &active);
        assert!(publish.is_empty());
        assert_eq!(unpublish, vec!["a._http._tcp.local".to_string()]);
    }
}
