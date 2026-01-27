extern crate alloc;

pub mod builder;
pub mod pack;

pub use builder::{
    AlignItems, Checkbox, Color, Flex, FlexDirection, FontKey, Image, ImageFit, JustifyContent,
    Rect, Scene, Size, Styled, Text, Window,
};

use crate::errors::{Error, Result};
use crate::thing::ThingId;
use crate::thing::sys::{bytespace_create, bytespace_write, find, prop_get, prop_set};
use abi::errors::Errno;
use abi::ids::HandleId;
use abi::schema::{keys, kinds};

pub fn publish_window(scene: &Scene) -> Result<()> {
    let target = scene_window_id(scene).ok_or(Error::Errno(Errno::EINVAL))?;
    publish_scene(target, scene)
}

pub fn publish_desktop(scene: &Scene) -> Result<()> {
    let mut roots = [ThingId::default(); 1];
    let count = find(kinds::UI_ROOT, &mut roots).map_err(Error::Errno)?;
    if count == 0 {
        return Err(Error::Errno(Errno::ENOENT));
    }
    publish_scene(roots[0], scene)
}

fn publish_scene(target: ThingId, scene: &Scene) -> Result<()> {
    let bytes = pack::pack_scene(scene)?;
    let bs = bytespace_create(bytes.len(), 0, 0).map_err(Error::Errno)?;
    bytespace_write(bs, 0, &bytes).map_err(Error::Errno)?;
    prop_set(target, keys::UI_SCENE_BYTESPACE, bs.to_u64_lossy()).map_err(Error::Errno)?;
    let current = prop_get(target, keys::UI_SCENE_GEN).unwrap_or(0);
    let next = current.saturating_add(1);
    prop_set(target, keys::UI_SCENE_GEN, next).map_err(Error::Errno)?;
    Ok(())
}

fn scene_window_id(scene: &Scene) -> Option<ThingId> {
    let root = scene.root.as_ref()?;
    match &root.data {
        builder::NodeData::Window(window) => Some(window.wid),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::petals::builder::{Flex, Text, Window};
    use abi::ids::HandleId;
    use alloc::collections::BTreeMap;

    struct FakeGraph {
        props: BTreeMap<(ThingId, &'static str), u64>,
        bytespaces: Vec<Vec<u8>>,
    }

    impl FakeGraph {
        fn new() -> Self {
            Self {
                props: BTreeMap::new(),
                bytespaces: Vec::new(),
            }
        }
    }

    fn publish_with_fake(scene: &Scene, target: ThingId, graph: &mut FakeGraph) -> Result<()> {
        let bytes = pack::pack_scene(scene)?;
        let bs_id = ThingId::from_u64(graph.bytespaces.len() as u64 + 1);
        graph.bytespaces.push(bytes);
        graph
            .props
            .insert((target, keys::UI_SCENE_BYTESPACE), bs_id.to_u64_lossy());
        let current = graph
            .props
            .get(&(target, keys::UI_SCENE_GEN))
            .copied()
            .unwrap_or(0);
        graph
            .props
            .insert((target, keys::UI_SCENE_GEN), current.saturating_add(1));
        Ok(())
    }

    #[test]
    fn publish_increments_gen_and_updates_bytespace() {
        let wid = ThingId::from_u64(42);
        let scene = Scene::new().window(Window::new(wid).root(Flex::column().push(Text::new("x"))));
        let mut graph = FakeGraph::new();
        publish_with_fake(&scene, wid, &mut graph).expect("publish");
        let first_bs = graph
            .props
            .get(&(wid, keys::UI_SCENE_BYTESPACE))
            .copied()
            .unwrap_or(0);
        let first_gen = graph
            .props
            .get(&(wid, keys::UI_SCENE_GEN))
            .copied()
            .unwrap_or(0);
        publish_with_fake(&scene, wid, &mut graph).expect("publish");
        let second_bs = graph
            .props
            .get(&(wid, keys::UI_SCENE_BYTESPACE))
            .copied()
            .unwrap_or(0);
        let second_gen = graph
            .props
            .get(&(wid, keys::UI_SCENE_GEN))
            .copied()
            .unwrap_or(0);
        assert_ne!(first_bs, 0);
        assert_ne!(second_bs, 0);
        assert_ne!(first_bs, second_bs);
        assert_eq!(first_gen + 1, second_gen);
    }
}
