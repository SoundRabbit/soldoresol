precision mediump float;

uniform vec4 u_bgColor;
varying vec2 v_textureCoord;
uniform sampler2D u_texture1;
uniform sampler2D u_texture2;
uniform int u_texture2IsAvailable;

vec4 blend(vec4 bg, vec4 fr) {
    float dist_a = bg.w;
    float src_a = fr.w;
    float out_a = src_a + dist_a * (1.0 - src_a);
    vec3 out_rgb  = (fr.xyz * src_a + bg.xyz * dist_a * (1.0 - src_a)) / out_a;
    return vec4(out_rgb, out_a);
}

void main() {
    vec4 smpColor0 = texture2D(u_texture1, v_textureCoord);
    vec4 smpColor1 = u_texture2IsAvailable != 0 ? texture2D(u_texture2, v_textureCoord) : vec4(0.0,0.0,0.0,0.0);
    vec4 color_a = u_bgColor;
    vec4 color_b = blend(color_a, smpColor1);
    vec4 color_c = blend(color_b, smpColor0);
    gl_FragColor = color_c;
}
