/// Calculate the distance between two 2D points.
pub fn distance(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    return ((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2)).sqrt();
}
