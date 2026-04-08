//! Integration tests for `elicit_geo_types`.

use elicit_geo_types::{Coord, Line, MultiPoint, Point, Rect, Triangle};

#[test]
fn coord_round_trip() {
    let c = Coord::new(1.0, 2.0);
    assert_eq!(c.x(), 1.0);
    assert_eq!(c.y(), 2.0);
}

#[test]
fn point_accessors() {
    let p = Point::new(3.0, 4.0);
    assert_eq!(p.x(), 3.0);
    assert_eq!(p.y(), 4.0);
    assert_eq!(p.lng(), 3.0);
    assert_eq!(p.lat(), 4.0);
}

#[test]
fn rect_dimensions() {
    let r = Rect::new(Coord::new(0.0, 0.0), Coord::new(4.0, 3.0));
    assert_eq!(r.width(), 4.0);
    assert_eq!(r.height(), 3.0);
}

#[test]
fn line_accessors() {
    let l = Line::new(Coord::new(0.0, 0.0), Coord::new(3.0, 4.0));
    assert_eq!(l.dx(), 3.0);
    assert_eq!(l.dy(), 4.0);
}

#[test]
fn multi_point_count() {
    let points = vec![Point::new(0.0, 0.0), Point::new(1.0, 1.0)];
    let mp = MultiPoint::new(points);
    assert_eq!(mp.count(), 2);
}

#[test]
fn triangle_vertices() {
    let t = Triangle::new(
        Coord::new(0.0, 0.0),
        Coord::new(1.0, 0.0),
        Coord::new(0.0, 1.0),
    );
    assert_eq!(t.v1().x(), 0.0);
    assert_eq!(t.v2().x(), 1.0);
    assert_eq!(t.v3().y(), 1.0);
}
