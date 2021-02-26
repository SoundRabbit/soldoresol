precision mediump float;

uniform vec4 u_bgColor;
varying vec2 v_textureCoord;
uniform sampler2D u_texture;
uniform sampler2D u_texture1;
uniform sampler2D u_texture2;
uniform int u_texture2IsAvailable;
uniform int u_useTextureAsMask;

#define IF(x) (x != 0)

vec4 blend(vec4 bg, vec4 fr) {
    float dist_a = bg.w;
    float src_a = fr.w;
    float out_a = dist_a * src_a + src_a * (1.0 - dist_a) + dist_a * (1.0 - src_a);
    vec3 out_rgb  = out_a > 0.0 ? (fr.xyz * src_a + bg.xyz * dist_a * (1.0 - src_a)) / out_a : vec3(0.0, 0.0, 0.0);
    return vec4(out_rgb, out_a);
}

vec4 mask(vec4 bg, vec4 mk) {
    return vec4(bg.xyz, bg.w * (1.0 - mk.w));
}

void main() {
    vec4 color;
    vec4 tex_color;
    color = vec4(0.0, 0.0, 0.0, 0.0);
    color = blend(color, u_bgColor);
    color = IF(u_texture2IsAvailable) ? blend(color, texture2D(u_texture2, v_textureCoord)) : color;
    tex_color = texture2D(u_texture1, v_textureCoord);
    tex_color = IF(u_useTextureAsMask) ?  mask(tex_color, texture2D(u_texture, v_textureCoord)) : blend(tex_color, texture2D(u_texture, v_textureCoord));
    color = blend(color, tex_color);
    gl_FragColor = color;
}
