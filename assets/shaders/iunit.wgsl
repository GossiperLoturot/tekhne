@group(0) @binding(0)
var<uniform> vp_matrix: mat4x4<f32>;

struct InstanceInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    instance: InstanceInput,
) -> VertexOutput {
    var position: vec3<f32>;

    if in_vertex_index == u32(0) {
        position = vec3<f32>(0.0, 0.0, 0.0);
    } else if in_vertex_index == u32(1) {
        position = vec3<f32>(1.0, 0.0, 0.0);
    } else if in_vertex_index == u32(2) {
        position = vec3<f32>(1.0, 1.0, 0.0);
    } else if in_vertex_index == u32(3) {
        position = vec3<f32>(1.0, 1.0, 0.0);
    } else if in_vertex_index == u32(4) {
        position = vec3<f32>(0.0, 1.0, 0.0);
    } else if in_vertex_index == u32(5) {
        position = vec3<f32>(0.0, 0.0, 0.0);
    }

    position = position + instance.position;

    var out: VertexOutput;
    out.clip_position = vp_matrix * vec4<f32>(position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
