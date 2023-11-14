/*
@group(0) @binding(0) var<storage, read_write> data: array<f32>;
@group(0) @binding(1) var<storage, read_write> secondData: array<f32>;
@group(0) @binding(2) var<storage, read_write> matrixSize: vec2<u32>;

@compute @workgroup_size(1) fn computeSomething(
@builtin(global_invocation_id) id: vec3<u32>
) {
    let i: u32 = id.x;
    data[i] = data[i] + secondData[i];
}

fn doubler(num: f32) -> f32 {
    return num * 4.0;
}

@compute @workgroup_size(1,1) fn matrixMult(
@builtin(global_invocation_id) id: vec3<u32>
) {
    let i: u32 = id.x;
    let j: u32 = id.y;

    var result: f32 = 0.0;
    for (var k = 0u; k < matrixSize[0]; k = k+1u) {
        result = result + data[i][k] * secondData[k][j];
    }


    data[i][j] = result;
}
*/

@group(0) @binding(0) var<uniform> x_edge: f32;
@group(0) @binding(1) var<uniform> y_edge: f32;
@group(0) @binding(2) var<storage, read_write> x_pos: array<f32>;
@group(0) @binding(3) var<storage, read_write> y_pos: array<f32>;
@group(0) @binding(4) var<storage, read_write> x_vel: array<f32>;
@group(0) @binding(5) var<storage, read_write> y_vel: array<f32>;

@compute @workgroup_size(1,1,1) fn movePosChange(
    @builtin(global_invocation_id) id: vec3<u32>
) {
    let i: u32 = id.x;
    let pos: vec2<f32> = vec2<f32>(x_pos[i] + x_vel[i], y_pos[i]+y_vel[i]);

    if pos[0] < 0.0 {
        x_pos[i] = -pos[0];
        x_vel[i] = - x_vel[i];
    } else if pos[0] > x_edge {
        x_pos[i] = 2.0 * x_edge - pos[0];
        x_vel[i] = - x_vel[i];
    } else {
        x_pos[i] = pos[0];
    }

    if pos[1] < 0.0 {
        y_pos[i] = -pos[1];
        y_vel[i] = - y_vel[i];
    } else if pos[1] > x_edge {
        y_pos[i] = 2.0 * y_edge - pos[1];
        y_vel[i] = - y_vel[i];
    } else {
        y_pos[i] = pos[1];
    }
}
