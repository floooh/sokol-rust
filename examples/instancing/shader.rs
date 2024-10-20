#![allow(dead_code)]
use crate::math as m;
use sokol::gfx as sg;
/*
    #version:1# (machine generated, don't edit!)

    Generated by sokol-shdc (https://github.com/floooh/sokol-tools)

    Cmdline:
        sokol-shdc -i examples/instancing/shader.glsl -o examples/instancing/shader.rs -l glsl430:metal_macos:hlsl5 -f sokol_rust

    Overview:
    =========
    Shader program: 'instancing':
        Get shader desc: instancing_shader_desc(sg::query_backend());
        Vertex Shader: vs
        Fragment Shader: fs
        Attributes:
            ATTR_INSTANCING_POS => 0
            ATTR_INSTANCING_COLOR0 => 1
            ATTR_INSTANCING_INST_POS => 2
    Bindings:
        Uniform block 'vs_params':
            Rust struct: VsParams
            Bind slot: UB_VS_PARAMS => 0
*/
pub const ATTR_INSTANCING_POS: usize = 0;
pub const ATTR_INSTANCING_COLOR0: usize = 1;
pub const ATTR_INSTANCING_INST_POS: usize = 2;
pub const UB_VS_PARAMS: usize = 0;
#[repr(C, align(16))]
pub struct VsParams {
    pub mvp: m::Mat4,
}
/*
    #version 430

    uniform vec4 vs_params[4];
    layout(location = 0) in vec3 pos;
    layout(location = 2) in vec3 inst_pos;
    layout(location = 0) out vec4 color;
    layout(location = 1) in vec4 color0;

    void main()
    {
        gl_Position = mat4(vs_params[0], vs_params[1], vs_params[2], vs_params[3]) * vec4(pos + inst_pos, 1.0);
        color = color0;
    }

*/
pub const VS_SOURCE_GLSL430: [u8; 335] = [
    0x23, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x34, 0x33, 0x30, 0x0a, 0x0a, 0x75, 0x6e, 0x69,
    0x66, 0x6f, 0x72, 0x6d, 0x20, 0x76, 0x65, 0x63, 0x34, 0x20, 0x76, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61,
    0x6d, 0x73, 0x5b, 0x34, 0x5d, 0x3b, 0x0a, 0x6c, 0x61, 0x79, 0x6f, 0x75, 0x74, 0x28, 0x6c, 0x6f, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x30, 0x29, 0x20, 0x69, 0x6e, 0x20, 0x76, 0x65, 0x63,
    0x33, 0x20, 0x70, 0x6f, 0x73, 0x3b, 0x0a, 0x6c, 0x61, 0x79, 0x6f, 0x75, 0x74, 0x28, 0x6c, 0x6f, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x32, 0x29, 0x20, 0x69, 0x6e, 0x20, 0x76, 0x65, 0x63,
    0x33, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x5f, 0x70, 0x6f, 0x73, 0x3b, 0x0a, 0x6c, 0x61, 0x79, 0x6f, 0x75,
    0x74, 0x28, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x30, 0x29, 0x20, 0x6f,
    0x75, 0x74, 0x20, 0x76, 0x65, 0x63, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0x0a, 0x6c, 0x61,
    0x79, 0x6f, 0x75, 0x74, 0x28, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x31,
    0x29, 0x20, 0x69, 0x6e, 0x20, 0x76, 0x65, 0x63, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x3b,
    0x0a, 0x0a, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x28, 0x29, 0x0a, 0x7b, 0x0a, 0x20,
    0x20, 0x20, 0x20, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20,
    0x6d, 0x61, 0x74, 0x34, 0x28, 0x76, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x5b, 0x30, 0x5d,
    0x2c, 0x20, 0x76, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x5b, 0x31, 0x5d, 0x2c, 0x20, 0x76,
    0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x5b, 0x32, 0x5d, 0x2c, 0x20, 0x76, 0x73, 0x5f, 0x70,
    0x61, 0x72, 0x61, 0x6d, 0x73, 0x5b, 0x33, 0x5d, 0x29, 0x20, 0x2a, 0x20, 0x76, 0x65, 0x63, 0x34, 0x28,
    0x70, 0x6f, 0x73, 0x20, 0x2b, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x5f, 0x70, 0x6f, 0x73, 0x2c, 0x20, 0x31,
    0x2e, 0x30, 0x29, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20,
    0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x3b, 0x0a, 0x7d, 0x0a, 0x0a, 0x00,
];
/*
    #version 430

    layout(location = 0) out vec4 frag_color;
    layout(location = 0) in vec4 color;

    void main()
    {
        frag_color = color;
    }

*/
pub const FS_SOURCE_GLSL430: [u8; 135] = [
    0x23, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x34, 0x33, 0x30, 0x0a, 0x0a, 0x6c, 0x61, 0x79,
    0x6f, 0x75, 0x74, 0x28, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x30, 0x29,
    0x20, 0x6f, 0x75, 0x74, 0x20, 0x76, 0x65, 0x63, 0x34, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f,
    0x6c, 0x6f, 0x72, 0x3b, 0x0a, 0x6c, 0x61, 0x79, 0x6f, 0x75, 0x74, 0x28, 0x6c, 0x6f, 0x63, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x30, 0x29, 0x20, 0x69, 0x6e, 0x20, 0x76, 0x65, 0x63, 0x34, 0x20,
    0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0x0a, 0x0a, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x6d, 0x61, 0x69, 0x6e,
    0x28, 0x29, 0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f, 0x6c,
    0x6f, 0x72, 0x20, 0x3d, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0x0a, 0x7d, 0x0a, 0x0a, 0x00,
];
/*
    cbuffer vs_params : register(b0)
    {
        row_major float4x4 _33_mvp : packoffset(c0);
    };


    static float4 gl_Position;
    static float3 pos;
    static float3 inst_pos;
    static float4 color;
    static float4 color0;

    struct SPIRV_Cross_Input
    {
        float3 pos : TEXCOORD0;
        float4 color0 : TEXCOORD1;
        float3 inst_pos : TEXCOORD2;
    };

    struct SPIRV_Cross_Output
    {
        float4 color : TEXCOORD0;
        float4 gl_Position : SV_Position;
    };

    void vert_main()
    {
        gl_Position = mul(float4(pos + inst_pos, 1.0f), _33_mvp);
        color = color0;
    }

    SPIRV_Cross_Output main(SPIRV_Cross_Input stage_input)
    {
        pos = stage_input.pos;
        inst_pos = stage_input.inst_pos;
        color0 = stage_input.color0;
        vert_main();
        SPIRV_Cross_Output stage_output;
        stage_output.gl_Position = gl_Position;
        stage_output.color = color;
        return stage_output;
    }
*/
pub const VS_SOURCE_HLSL5: [u8; 842] = [
    0x63, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x20, 0x76, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73,
    0x20, 0x3a, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x28, 0x62, 0x30, 0x29, 0x0a, 0x7b,
    0x0a, 0x20, 0x20, 0x20, 0x20, 0x72, 0x6f, 0x77, 0x5f, 0x6d, 0x61, 0x6a, 0x6f, 0x72, 0x20, 0x66, 0x6c,
    0x6f, 0x61, 0x74, 0x34, 0x78, 0x34, 0x20, 0x5f, 0x33, 0x33, 0x5f, 0x6d, 0x76, 0x70, 0x20, 0x3a, 0x20,
    0x70, 0x61, 0x63, 0x6b, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x28, 0x63, 0x30, 0x29, 0x3b, 0x0a, 0x7d,
    0x3b, 0x0a, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34,
    0x20, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x3b, 0x0a, 0x73, 0x74, 0x61,
    0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x33, 0x20, 0x70, 0x6f, 0x73, 0x3b, 0x0a, 0x73,
    0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x33, 0x20, 0x69, 0x6e, 0x73, 0x74,
    0x5f, 0x70, 0x6f, 0x73, 0x3b, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61,
    0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20,
    0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x3b, 0x0a, 0x0a, 0x73,
    0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73,
    0x5f, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61,
    0x74, 0x33, 0x20, 0x70, 0x6f, 0x73, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44,
    0x30, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c,
    0x6f, 0x72, 0x30, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44, 0x31, 0x3b, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x33, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x5f, 0x70,
    0x6f, 0x73, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44, 0x32, 0x3b, 0x0a, 0x7d,
    0x3b, 0x0a, 0x0a, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43,
    0x72, 0x6f, 0x73, 0x73, 0x5f, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20,
    0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3a, 0x20, 0x54,
    0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44, 0x30, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f,
    0x61, 0x74, 0x34, 0x20, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3a,
    0x20, 0x53, 0x56, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x3b, 0x0a, 0x7d, 0x3b, 0x0a,
    0x0a, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x76, 0x65, 0x72, 0x74, 0x5f, 0x6d, 0x61, 0x69, 0x6e, 0x28, 0x29,
    0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x3d, 0x20, 0x6d, 0x75, 0x6c, 0x28, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x28, 0x70, 0x6f,
    0x73, 0x20, 0x2b, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x5f, 0x70, 0x6f, 0x73, 0x2c, 0x20, 0x31, 0x2e, 0x30,
    0x66, 0x29, 0x2c, 0x20, 0x5f, 0x33, 0x33, 0x5f, 0x6d, 0x76, 0x70, 0x29, 0x3b, 0x0a, 0x20, 0x20, 0x20,
    0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x3b, 0x0a,
    0x7d, 0x0a, 0x0a, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x4f, 0x75,
    0x74, 0x70, 0x75, 0x74, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x28, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43,
    0x72, 0x6f, 0x73, 0x73, 0x5f, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f,
    0x69, 0x6e, 0x70, 0x75, 0x74, 0x29, 0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x70, 0x6f, 0x73, 0x20,
    0x3d, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x70, 0x6f, 0x73,
    0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x5f, 0x70, 0x6f, 0x73, 0x20, 0x3d, 0x20,
    0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x69, 0x6e, 0x73, 0x74, 0x5f,
    0x70, 0x6f, 0x73, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x20, 0x3d,
    0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x63, 0x6f, 0x6c, 0x6f,
    0x72, 0x30, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x76, 0x65, 0x72, 0x74, 0x5f, 0x6d, 0x61, 0x69, 0x6e,
    0x28, 0x29, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f,
    0x73, 0x73, 0x5f, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f,
    0x75, 0x74, 0x70, 0x75, 0x74, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f,
    0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x2e, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x3d, 0x20, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x3b, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x2e,
    0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0x0a, 0x20, 0x20,
    0x20, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f, 0x75,
    0x74, 0x70, 0x75, 0x74, 0x3b, 0x0a, 0x7d, 0x0a, 0x00,
];
/*
    static float4 frag_color;
    static float4 color;

    struct SPIRV_Cross_Input
    {
        float4 color : TEXCOORD0;
    };

    struct SPIRV_Cross_Output
    {
        float4 frag_color : SV_Target0;
    };

    void frag_main()
    {
        frag_color = color;
    }

    SPIRV_Cross_Output main(SPIRV_Cross_Input stage_input)
    {
        color = stage_input.color;
        frag_main();
        SPIRV_Cross_Output stage_output;
        stage_output.frag_color = frag_color;
        return stage_output;
    }
*/
pub const FS_SOURCE_HLSL5: [u8; 435] = [
    0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x66, 0x72, 0x61,
    0x67, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x69, 0x63, 0x20, 0x66,
    0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0x0a, 0x0a, 0x73, 0x74, 0x72,
    0x75, 0x63, 0x74, 0x20, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x49,
    0x6e, 0x70, 0x75, 0x74, 0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34,
    0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3a, 0x20, 0x54, 0x45, 0x58, 0x43, 0x4f, 0x4f, 0x52, 0x44,
    0x30, 0x3b, 0x0a, 0x7d, 0x3b, 0x0a, 0x0a, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x53, 0x50, 0x49,
    0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x0a, 0x7b,
    0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f,
    0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3a, 0x20, 0x53, 0x56, 0x5f, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74,
    0x30, 0x3b, 0x0a, 0x7d, 0x3b, 0x0a, 0x0a, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f,
    0x6d, 0x61, 0x69, 0x6e, 0x28, 0x29, 0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x72, 0x61, 0x67,
    0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0x0a, 0x7d,
    0x0a, 0x0a, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72, 0x6f, 0x73, 0x73, 0x5f, 0x4f, 0x75, 0x74,
    0x70, 0x75, 0x74, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x28, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72,
    0x6f, 0x73, 0x73, 0x5f, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69,
    0x6e, 0x70, 0x75, 0x74, 0x29, 0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72,
    0x20, 0x3d, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2e, 0x63, 0x6f,
    0x6c, 0x6f, 0x72, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x6d, 0x61, 0x69,
    0x6e, 0x28, 0x29, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x53, 0x50, 0x49, 0x52, 0x56, 0x5f, 0x43, 0x72,
    0x6f, 0x73, 0x73, 0x5f, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f,
    0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65,
    0x5f, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x2e, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f, 0x6c, 0x6f,
    0x72, 0x20, 0x3d, 0x20, 0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0x0a, 0x20,
    0x20, 0x20, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x6f,
    0x75, 0x74, 0x70, 0x75, 0x74, 0x3b, 0x0a, 0x7d, 0x0a, 0x00,
];
/*
    #include <metal_stdlib>
    #include <simd/simd.h>

    using namespace metal;

    struct vs_params
    {
        float4x4 mvp;
    };

    struct main0_out
    {
        float4 color [[user(locn0)]];
        float4 gl_Position [[position]];
    };

    struct main0_in
    {
        float3 pos [[attribute(0)]];
        float4 color0 [[attribute(1)]];
        float3 inst_pos [[attribute(2)]];
    };

    vertex main0_out main0(main0_in in [[stage_in]], constant vs_params& _33 [[buffer(0)]])
    {
        main0_out out = {};
        out.gl_Position = _33.mvp * float4(in.pos + in.inst_pos, 1.0);
        out.color = in.color0;
        return out;
    }

*/
pub const VS_SOURCE_METAL_MACOS: [u8; 564] = [
    0x23, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x20, 0x3c, 0x6d, 0x65, 0x74, 0x61, 0x6c, 0x5f, 0x73,
    0x74, 0x64, 0x6c, 0x69, 0x62, 0x3e, 0x0a, 0x23, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x20, 0x3c,
    0x73, 0x69, 0x6d, 0x64, 0x2f, 0x73, 0x69, 0x6d, 0x64, 0x2e, 0x68, 0x3e, 0x0a, 0x0a, 0x75, 0x73, 0x69,
    0x6e, 0x67, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x61,
    0x6c, 0x3b, 0x0a, 0x0a, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x76, 0x73, 0x5f, 0x70, 0x61, 0x72,
    0x61, 0x6d, 0x73, 0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x78,
    0x34, 0x20, 0x6d, 0x76, 0x70, 0x3b, 0x0a, 0x7d, 0x3b, 0x0a, 0x0a, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74,
    0x20, 0x6d, 0x61, 0x69, 0x6e, 0x30, 0x5f, 0x6f, 0x75, 0x74, 0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20,
    0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x5b, 0x5b, 0x75, 0x73,
    0x65, 0x72, 0x28, 0x6c, 0x6f, 0x63, 0x6e, 0x30, 0x29, 0x5d, 0x5d, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20,
    0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x67, 0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x5b, 0x5b, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5d, 0x5d, 0x3b, 0x0a, 0x7d,
    0x3b, 0x0a, 0x0a, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x30, 0x5f, 0x69,
    0x6e, 0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x33, 0x20, 0x70, 0x6f,
    0x73, 0x20, 0x5b, 0x5b, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x28, 0x30, 0x29, 0x5d,
    0x5d, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c,
    0x6f, 0x72, 0x30, 0x20, 0x5b, 0x5b, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x28, 0x31,
    0x29, 0x5d, 0x5d, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x33, 0x20, 0x69,
    0x6e, 0x73, 0x74, 0x5f, 0x70, 0x6f, 0x73, 0x20, 0x5b, 0x5b, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75,
    0x74, 0x65, 0x28, 0x32, 0x29, 0x5d, 0x5d, 0x3b, 0x0a, 0x7d, 0x3b, 0x0a, 0x0a, 0x76, 0x65, 0x72, 0x74,
    0x65, 0x78, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x30, 0x5f, 0x6f, 0x75, 0x74, 0x20, 0x6d, 0x61, 0x69, 0x6e,
    0x30, 0x28, 0x6d, 0x61, 0x69, 0x6e, 0x30, 0x5f, 0x69, 0x6e, 0x20, 0x69, 0x6e, 0x20, 0x5b, 0x5b, 0x73,
    0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x5d, 0x5d, 0x2c, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x74, 0x61,
    0x6e, 0x74, 0x20, 0x76, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x26, 0x20, 0x5f, 0x33, 0x33,
    0x20, 0x5b, 0x5b, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x28, 0x30, 0x29, 0x5d, 0x5d, 0x29, 0x0a, 0x7b,
    0x0a, 0x20, 0x20, 0x20, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x30, 0x5f, 0x6f, 0x75, 0x74, 0x20, 0x6f, 0x75,
    0x74, 0x20, 0x3d, 0x20, 0x7b, 0x7d, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x6f, 0x75, 0x74, 0x2e, 0x67,
    0x6c, 0x5f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x5f, 0x33, 0x33, 0x2e,
    0x6d, 0x76, 0x70, 0x20, 0x2a, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x28, 0x69, 0x6e, 0x2e, 0x70,
    0x6f, 0x73, 0x20, 0x2b, 0x20, 0x69, 0x6e, 0x2e, 0x69, 0x6e, 0x73, 0x74, 0x5f, 0x70, 0x6f, 0x73, 0x2c,
    0x20, 0x31, 0x2e, 0x30, 0x29, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x6f, 0x75, 0x74, 0x2e, 0x63, 0x6f,
    0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20, 0x69, 0x6e, 0x2e, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x30, 0x3b, 0x0a,
    0x20, 0x20, 0x20, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x6f, 0x75, 0x74, 0x3b, 0x0a, 0x7d,
    0x0a, 0x0a, 0x00,
];
/*
    #include <metal_stdlib>
    #include <simd/simd.h>

    using namespace metal;

    struct main0_out
    {
        float4 frag_color [[color(0)]];
    };

    struct main0_in
    {
        float4 color [[user(locn0)]];
    };

    fragment main0_out main0(main0_in in [[stage_in]])
    {
        main0_out out = {};
        out.frag_color = in.color;
        return out;
    }

*/
pub const FS_SOURCE_METAL_MACOS: [u8; 315] = [
    0x23, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x20, 0x3c, 0x6d, 0x65, 0x74, 0x61, 0x6c, 0x5f, 0x73,
    0x74, 0x64, 0x6c, 0x69, 0x62, 0x3e, 0x0a, 0x23, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x20, 0x3c,
    0x73, 0x69, 0x6d, 0x64, 0x2f, 0x73, 0x69, 0x6d, 0x64, 0x2e, 0x68, 0x3e, 0x0a, 0x0a, 0x75, 0x73, 0x69,
    0x6e, 0x67, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x61,
    0x6c, 0x3b, 0x0a, 0x0a, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x30, 0x5f,
    0x6f, 0x75, 0x74, 0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20,
    0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x5b, 0x5b, 0x63, 0x6f, 0x6c, 0x6f,
    0x72, 0x28, 0x30, 0x29, 0x5d, 0x5d, 0x3b, 0x0a, 0x7d, 0x3b, 0x0a, 0x0a, 0x73, 0x74, 0x72, 0x75, 0x63,
    0x74, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x30, 0x5f, 0x69, 0x6e, 0x0a, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20,
    0x66, 0x6c, 0x6f, 0x61, 0x74, 0x34, 0x20, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x5b, 0x5b, 0x75, 0x73,
    0x65, 0x72, 0x28, 0x6c, 0x6f, 0x63, 0x6e, 0x30, 0x29, 0x5d, 0x5d, 0x3b, 0x0a, 0x7d, 0x3b, 0x0a, 0x0a,
    0x66, 0x72, 0x61, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x30, 0x5f, 0x6f, 0x75,
    0x74, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x30, 0x28, 0x6d, 0x61, 0x69, 0x6e, 0x30, 0x5f, 0x69, 0x6e, 0x20,
    0x69, 0x6e, 0x20, 0x5b, 0x5b, 0x73, 0x74, 0x61, 0x67, 0x65, 0x5f, 0x69, 0x6e, 0x5d, 0x5d, 0x29, 0x0a,
    0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x30, 0x5f, 0x6f, 0x75, 0x74, 0x20, 0x6f,
    0x75, 0x74, 0x20, 0x3d, 0x20, 0x7b, 0x7d, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x6f, 0x75, 0x74, 0x2e,
    0x66, 0x72, 0x61, 0x67, 0x5f, 0x63, 0x6f, 0x6c, 0x6f, 0x72, 0x20, 0x3d, 0x20, 0x69, 0x6e, 0x2e, 0x63,
    0x6f, 0x6c, 0x6f, 0x72, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20,
    0x6f, 0x75, 0x74, 0x3b, 0x0a, 0x7d, 0x0a, 0x0a, 0x00,
];
pub fn instancing_shader_desc(backend: sg::Backend) -> sg::ShaderDesc {
    let mut desc = sg::ShaderDesc::new();
    desc.label = c"instancing_shader".as_ptr();
    match backend {
        sg::Backend::Glcore => {
            desc.vertex_func.source = &VS_SOURCE_GLSL430 as *const _ as *const _;
            desc.vertex_func.entry = c"main".as_ptr();
            desc.fragment_func.source = &FS_SOURCE_GLSL430 as *const _ as *const _;
            desc.fragment_func.entry = c"main".as_ptr();
            desc.attrs[0].glsl_name = c"pos".as_ptr();
            desc.attrs[1].glsl_name = c"color0".as_ptr();
            desc.attrs[2].glsl_name = c"inst_pos".as_ptr();
            desc.uniform_blocks[0].stage = sg::ShaderStage::Vertex;
            desc.uniform_blocks[0].layout = sg::UniformLayout::Std140;
            desc.uniform_blocks[0].size = 64;
            desc.uniform_blocks[0].glsl_uniforms[0]._type = sg::UniformType::Float4;
            desc.uniform_blocks[0].glsl_uniforms[0].array_count = 4;
            desc.uniform_blocks[0].glsl_uniforms[0].glsl_name = c"vs_params".as_ptr();
        },
        sg::Backend::D3d11 => {
            desc.vertex_func.source = &VS_SOURCE_HLSL5 as *const _ as *const _;
            desc.vertex_func.d3d11_target = c"vs_5_0".as_ptr();
            desc.vertex_func.entry = c"main".as_ptr();
            desc.fragment_func.source = &FS_SOURCE_HLSL5 as *const _ as *const _;
            desc.fragment_func.d3d11_target = c"ps_5_0".as_ptr();
            desc.fragment_func.entry = c"main".as_ptr();
            desc.attrs[0].hlsl_sem_name = c"TEXCOORD".as_ptr();
            desc.attrs[0].hlsl_sem_index = 0;
            desc.attrs[1].hlsl_sem_name = c"TEXCOORD".as_ptr();
            desc.attrs[1].hlsl_sem_index = 1;
            desc.attrs[2].hlsl_sem_name = c"TEXCOORD".as_ptr();
            desc.attrs[2].hlsl_sem_index = 2;
            desc.uniform_blocks[0].stage = sg::ShaderStage::Vertex;
            desc.uniform_blocks[0].layout = sg::UniformLayout::Std140;
            desc.uniform_blocks[0].size = 64;
            desc.uniform_blocks[0].hlsl_register_b_n = 0;
        },
        sg::Backend::MetalMacos => {
            desc.vertex_func.source = &VS_SOURCE_METAL_MACOS as *const _ as *const _;
            desc.vertex_func.entry = c"main0".as_ptr();
            desc.fragment_func.source = &FS_SOURCE_METAL_MACOS as *const _ as *const _;
            desc.fragment_func.entry = c"main0".as_ptr();
            desc.uniform_blocks[0].stage = sg::ShaderStage::Vertex;
            desc.uniform_blocks[0].layout = sg::UniformLayout::Std140;
            desc.uniform_blocks[0].size = 64;
            desc.uniform_blocks[0].msl_buffer_n = 0;
        },
        _ => {},
    }
    desc
}
