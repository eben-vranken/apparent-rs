#[Derive(Clone, Copy, Debug, PartialEq)]
pub struct vec3 {
    // I'm okay with these fields being public, since they don't need any validation
    // Any three numbers are a legitimate field
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
