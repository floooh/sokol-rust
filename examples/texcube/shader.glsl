//------------------------------------------------------------------------------
//  Shader code for texcube-sapp sample.
//
//  NOTE: This source file also uses the '#pragma sokol' form of the
//  custom tags.
//------------------------------------------------------------------------------
@header use crate::math as m
@ctype mat4 m::Mat4
@ctype vec4 m::Vec4
@ctype vec3 m::Vec3
@ctype vec2 m::Vec2

@vs vs
uniform vs_params {
    mat4 mvp;
};

in vec4 pos;
in vec4 color0;
in vec2 texcoord0;

out vec4 color;
out vec2 uv;

void main() {
    gl_Position = mvp * pos;
    color = color0;
    uv = texcoord0 * 5.0;
}
@end

@fs fs
uniform sampler2D tex;

in vec4 color;
in vec2 uv;
out vec4 frag_color;

void main() {
    frag_color = texture(tex, uv) * color;
}
@end

@program texcube vs fs
