//! Tests for the spatial bridge (BoundingBox, LayoutContext).

use elicit_ui::{BoundingBox, LayoutContext, Viewport};
use accesskit::NodeId;
use std::collections::HashMap;

#[test]
fn bounding_box_right_and_bottom() {
    let bb = BoundingBox::new(10.0, 20.0, 100.0, 50.0);
    assert_eq!(bb.right(), 110.0);
    assert_eq!(bb.bottom(), 70.0);
}

#[test]
fn bounding_box_within_viewport() {
    let bb = BoundingBox::new(0.0, 0.0, 100.0, 50.0);
    let vp = Viewport::new(1920, 1080);
    assert!(bb.within_viewport(&vp));
}

#[test]
fn bounding_box_outside_viewport() {
    let bb = BoundingBox::new(0.0, 0.0, 2000.0, 50.0);
    let vp = Viewport::new(1920, 1080);
    assert!(!bb.within_viewport(&vp));
}

#[test]
fn bounding_box_negative_origin_outside_viewport() {
    let bb = BoundingBox::new(-10.0, 0.0, 100.0, 50.0);
    let vp = Viewport::new(1920, 1080);
    assert!(!bb.within_viewport(&vp));
}

#[test]
fn bounding_box_meets_touch_target() {
    let large = BoundingBox::new(0.0, 0.0, 44.0, 44.0);
    assert!(large.meets_touch_target());

    let small = BoundingBox::new(0.0, 0.0, 43.0, 43.0);
    assert!(!small.meets_touch_target());
}

#[test]
fn bounding_box_to_size() {
    let bb = BoundingBox::new(0.0, 0.0, 100.5, 50.7);
    let size = bb.to_size();
    assert_eq!(size.width, 100);
    assert_eq!(size.height, 50);
}

#[test]
fn layout_context_get_bounds() {
    let id = NodeId::from(1u64);
    let bb = BoundingBox::new(10.0, 20.0, 100.0, 50.0);
    let mut bounds = HashMap::new();
    bounds.insert(id, bb);
    let ctx = LayoutContext::new(Viewport::new(1920, 1080), bounds);

    assert!(ctx.get_bounds(&id).is_some());
    assert!(ctx.get_bounds(&NodeId::from(99u64)).is_none());
}

#[test]
fn layout_context_is_within_viewport() {
    let id = NodeId::from(1u64);
    let bb = BoundingBox::new(0.0, 0.0, 100.0, 50.0);
    let mut bounds = HashMap::new();
    bounds.insert(id, bb);
    let ctx = LayoutContext::new(Viewport::new(1920, 1080), bounds);

    assert_eq!(ctx.is_within_viewport(&id), Some(true));
    assert_eq!(ctx.is_within_viewport(&NodeId::from(99u64)), None);
}

#[test]
fn bounding_box_from_node() {
    use accesskit::{Node, Role};

    let mut node = Node::new(Role::Button);
    node.set_bounds(accesskit::Rect {
        x0: 10.0,
        y0: 20.0,
        x1: 110.0,
        y1: 70.0,
    });

    let bb = BoundingBox::from_node(&node).unwrap();
    assert_eq!(bb.x, 10.0);
    assert_eq!(bb.y, 20.0);
    assert_eq!(bb.width, 100.0);
    assert_eq!(bb.height, 50.0);
}

#[test]
fn bounding_box_from_node_without_bounds() {
    use accesskit::{Node, Role};
    let node = Node::new(Role::Button);
    assert!(BoundingBox::from_node(&node).is_none());
}
