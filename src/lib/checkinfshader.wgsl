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


@group(0) @binding(0) var<uniform> infRad: f32;
@group(0) @binding(1) var<storage, read_write> inf: array<u32>;
@group(0) @binding(2) var<storage, read_write> xpos: array<f32>;
@group(0) @binding(3) var<storage, read_write> ypos: array<f32>;

/*
@compute @workgroup_size(1,1) fn checkInf(
    @builtin(global_invocation_id) id: vec3<u32>
) {
    let i: u32 = id.x;
    let j: u32 = id.y;

    let dx = xpos[i] - xpos[j];
    let dy = ypos[i] - ypos[j];

    if inf[i] == 1u {
        if inf[j] == 0u {
            if fma(dx,dx,dy * dy) <infRad {
                inf[j] = 1u;
            }
        }
    }
}
*/


@compute @workgroup_size(1,1,1) fn checkInf(
    @builtin(global_invocation_id) id: vec3<u32>
) {
    let i: u32 = id.x;
    let j: u32 = id.y;

    let dx = xpos[i] - xpos[j];
    let dy = ypos[i] - ypos[j];

    if inf[i] == 1u {
        if inf[j] == 0u {
            /*
            if xpos[i] > xpos[j] {
                if dx < infRad {
                    if ypos[i] > ypos[j] {
                        if dy < infRad {
                            if fma(dx,dx,dy * dy) <infRad {
                                inf[j] = 1u;
                            } else {
                                return;
                            }
                        } else {
                            return ; 
                        }
                    }  else {
                        if -1.0* dy <  infRad {
                            if fma(dx,dx,dy * dy) <infRad {
                                inf[j] = 1u;
                            } else {
                                return;
                            }
                        } else {
                            return;
                        }
                    }
                } else {
                    return;
                }
            } else {
                if -1.0*dx < infRad {
                    if ypos[i] > ypos[j] {
                        if dy < infRad {
                            if fma(dx,dx,dy * dy) <infRad {
                                inf[j] = 1u;
                            } else {
                                return;
                            }
                        } else {
                            return ; 
                        }
                    }  else {
                        if -1.0* dy < infRad {
                            if fma(dx,dx,dy * dy) <infRad {
                                inf[j] = 1u;
                            } else {
                                return;
                            }
                        } else {
                            return;
                        }
                    }
                }
            }
            */
            if dx < infRad {
                if dy < infRad {
                    inf[j] = 1u;
                } else {
                    return;
                }
            } else {
                return;
            }
        }
    }
}