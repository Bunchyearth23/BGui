struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) frag_color: vec4<f32>
};

struct VertexInput {
    @location(0) position: vec2<f32>,
};

struct Uniforms {
    color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u_color: Uniforms;

@vertex
fn v_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(input.position, 0.0, 1.0);
    out.frag_color = u_color.color;
    return out;
}

@fragment
fn f_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.frag_color;
}
