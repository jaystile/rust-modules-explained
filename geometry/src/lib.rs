
/// Point struct represents a point in three dimensional space
#[allow(dead_code)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

/// Type enum list of supported dimensions
pub enum Type {
    D1,
    D2,
    D3,
}

/// Dimensional trait supplies methods for 3D geometry calculations
pub trait Dimensional {
    fn dimensions(&self) -> Type;
}
