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

impl Bounds {
    pub fn new(x: f32, y: f32, z: f32, w: f32, h: f32) -> Self {
        Self { x, y, z, w, h }
    }
}

#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Resolution(pub u32, pub u32);

impl Resolution {
    pub fn new(x: u32, y: u32) -> Self {
        Self(x, y)
    }
}

pub struct ImageConfig {
    pub resolution: Resolution,
    pub bounds: Bounds,
}
