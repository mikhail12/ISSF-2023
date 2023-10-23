@group(0) @binding(0) var<storage, read_write> data: array<f32>;

@compute @workgroup_size(1) fn computeSomething(
@builtin(global_invocation_id) id: vec3<u32>
) {
    let i: u32 = id.x;
    data[i] = doubler(data[i]);
}

fn doubler(num: f32) -> f32 {
    return num * 4.0;
}

