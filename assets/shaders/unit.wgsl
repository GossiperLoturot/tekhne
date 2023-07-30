@group(0) @binding(0)
var<uniform> view_matrix: mat4x4<f32>;

@group(1) @binding(2)
var<uniform> grid_size: f32;

@group(2) @binding(0)
var<uniform> shape_matrix: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) texcoord: vec2<f32>,
};

struct InstanceInput {
    @location(2) position: vec3<f32>,
    @location(3) shape: vec4<f32>,
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
    var shape_size = instance.shape.zw - instance.shape.xy;
    var position = (vertex.position * shape_size + instance.shape.xy);
    var world_position = (shape_matrix * vec4<f32>(position, 0.0, 1.0)).xyz + instance.position;

    var texcoord_size = instance.texcoord.zw - instance.texcoord.xy;
    var texcoord = (vertex.texcoord * texcoord_size + instance.texcoord.xy) / grid_size;

    var out: VertexOutput;
    out.clip_position = view_matrix * vec4<f32>(world_position, 1.0);
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
