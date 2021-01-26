precision mediump float;

uniform vec4 u_bgColor;
varying vec2 v_textureCoord;
uniform sampler2D u_texture;
uniform sampler2D u_texture1;
uniform sampler2D u_texture2;
uniform int u_texture2IsAvailable;

#define IF(x) (x != 0)

vec4 blend(vec4 bg, vec4 fr) {
    float dist_a = bg.w;
    float src_a = fr.w;
    float out_a = src_a + dist_a * (1.0 - src_a);
    vec3 out_rgb  = (fr.xyz * src_a + bg.xyz * dist_a * (1.0 - src_a)) / out_a;
    return vec4(out_rgb, out_a);
}

void main() {
    vec4 color = u_bgColor;
    color = IF(u_texture2IsAvailable) ? blend(color, texture2D(u_texture2, v_textureCoord)) : color;
    color = blend(color, texture2D(u_texture1, v_textureCoord));
    color = blend(color, texture2D(u_texture, v_textureCoord));
    gl_FragColor = color;
}
