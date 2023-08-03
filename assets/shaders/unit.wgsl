@group(0) @binding(0)
var<uniform> view_matrix: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texcoord: vec2<f32>,
};

struct InstanceInput {
    @location(2) position_min: vec3<f32>,
    @location(3) position_max: vec3<f32>,
    @location(4) texcoord: vec4<f32>,
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
    var shape_size = instance.position_max - instance.position_min;
    var position = instance.position_min + vertex.position * shape_size;

    var texcoord = instance.texcoord.xy + vertex.texcoord * instance.texcoord.zw;

    var out: VertexOutput;
    out.clip_position = view_matrix * vec4<f32>(position, 1.0);
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
