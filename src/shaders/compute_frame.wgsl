struct Resolution {
  x: u32,
  y: u32
}

struct Bounds {
  x: f32,
  y: f32,
  w: f32,
  h: f32
}

@group(0)
@binding(2)
var<storage, read_write> result: array<u32>;

@group(0)
@binding(1)
var<storage> resolution: Resolution;

@group(0)
@binding(2)
var<storage> bounds: Bounds;


@compute
@workgroup_size(32)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var x: f32 = f32(global_id.x) / resolution.x * bounds.w + bounds.x;
    var y: f32 = f32(global_id.y) / resolution.y * bounds.h + bounds.y;
    var index: u32 = global_id.y * resolution.x + global_id.x;
    var value: f32 = 0.123456789;
    result[index] = value;
}