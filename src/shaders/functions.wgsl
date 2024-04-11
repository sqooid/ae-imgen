@group(0)
@binding(0)
var<storage, read_write> v_dim: array<u32>;

@group(1)
@binding(1)
var<storage, read_write> v_rgb: array<vec3<u32>>;

@compute @workgroup_size(32)
fn sin() {
}

@compute
@workgroup_size(32)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    w = v_dim[0];
    h = v_dim[1];
    x = global_id % w;
    y = floor(global_id / h)
}