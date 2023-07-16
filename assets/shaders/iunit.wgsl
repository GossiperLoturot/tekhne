@group(0) @binding(0)
var<uniform> vp_matrix: mat4x4<f32>;

@group(1) @binding(2)
var<uniform> grid_size: vec2<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texcoord: vec2<f32>,
};

struct InstanceInput {
    @location(2) position: vec3<f32>,
    @location(3) texcoord: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texcoord: vec2<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var position = vertex.position + instance.position;
    var texcoord = (vertex.texcoord + instance.texcoord) / grid_size;

    var out: VertexOutput;
    out.clip_position = vp_matrix * vec4<f32>(position, 1.0);
    out.texcoord = texcoord;
    return out;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(texture, texture_sampler, in.texcoord);

    if color.a < 0.5 {
        discard;
    }

    return color;
}
