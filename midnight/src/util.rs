use cgmath::Point2;

pub fn toIso(point: Point2<f64>) -> Point2<f64> {
    let mut tempPt = Point2::new(0.0, 0.0);
    tempPt.x = (point.x - point.y) / 2.0;
    tempPt.y = ((point.x + point.y) / 2.0) / 2.0;
    tempPt
}

pub fn to2D(point: Point2<f64>) -> Point2<f64> {
    let mut tempPt = Point2::new(0.0, 0.0);
    tempPt.x = 2.0 * point.y + point.x;
    tempPt.y = 2.0 * point.y - point.x;
    tempPt
}

pub fn toIsoT(x: f64, y: f64) -> (f64, f64) {
    ((x - y) / 2.0, ((x + y) / 2.0) / 2.0)
}

pub fn to2DT(x: f64, y: f64) -> (f64, f64) {
    (2.0 * y + x, 2.0 * y - x)
}
