use elicit_wkt::{Coord, GeometryCollection, LineString, MultiPoint, Point, Polygon, WktItem};
use std::str::FromStr;

#[test]
fn constructors_and_parse_work() {
    let coord = Coord::new(1.0, 2.0);
    let point = Point::new(coord.clone());
    let line = LineString::new(vec![coord.clone()]);
    let polygon = Polygon::new(line.clone(), vec![]);
    let multi = MultiPoint::new(vec![point.clone()]);
    let item = WktItem::from_str("POINT(1 2)").expect("valid WKT");
    let collection = GeometryCollection::new(vec![item.clone()]);

    assert_eq!(coord.x(), 1.0);
    assert!(!point.is_empty());
    assert_eq!(line.len(), 1);
    assert_eq!(polygon.interiors_len(), 0);
    assert_eq!(multi.len(), 1);
    assert_eq!(item.geometry_type(), "Point");
    assert_eq!(collection.len(), 1);
}
