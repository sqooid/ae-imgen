use serde::Serialize;

#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Bounds {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    h: f32,
}

#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Resolution(pub u32, pub u32);

pub struct ImageConfig {
    pub resolution: Resolution,
    pub bounds: Bounds,
}
