struct VertexOutput {
    @location(0) color: vec4<f32>,
    @builtin(position) clip_position: vec4<f32>,
};

//layout(set=0,binding=0) buffer

@group(0) @binding(0) var<uniform> inf: array<mat4x4<u32>>;
@group(0) @binding(1) var<uniform> xpos: array<mat4x4<u32>>;
@group(0) @binding(2) var<uniform> ypos: array<mat4x4<u32>>;


/*
@layout(set = 0, binding = 0) buffer Inf {
    
};
@layout(set = 0, binding = 1) buffer XPos {
    
};
@layout(set = 0, binding = 2) buffer YPos {
    
};

@block struct Uniforms {
    array<u32> inf;
    array<u32> xpos;
    array<u32> ypos;
}
*/

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    let i: u32 = in_vertex_index;
    var output: VertexOutput;
    output.clip_position = vec4<f32>(xpos[i], ypos[i],0.0,1.0);
    if (inf[i] == 1u) {
        output.color = vec4<f32>(1.0,1.0,0.0,1.0);
    } else {
        output.color = vec4<f32>(0.0,0.0,0.0,1.0);
    }
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}