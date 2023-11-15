struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};


@group(0) @binding(0) var<storage, read_write> inf: array<u32>;
@group(0) @binding(1) var<storage, read_write> xpos: array<f32>;
@group(0) @binding(2) var<storage, read_write> ypos: array<f32>;


@vertex
fn vs_main(&builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    let i: u32 = in_vertex_index;
    var output: VertexOutput;
    output.clip_position = vec4<f32>(xpos[i], ypos[i],0.0,1.0)
    if inf[i] == 1.0 {
        output.color = vec4<f32>(1.0,1.0,0.0,1.0);
    } else {
        output.color = vec4<f32>(0.0,0.0,0.0,1.0);
    }
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> [[@location(0)]] vec4<f32> {
    return input.color;
}