precision mediump float;

#extension GL_EXT_frag_depth : enable

uniform vec4 u_bgColor;
uniform sampler2D u_texture;
uniform vec3 u_textColor;
varying vec2 v_textureCoord;
varying float v_fragDepth;

void main() {
    vec4 smpColor = texture2D(u_texture, v_textureCoord);
    float dist_a = u_bgColor.w;
    float src_a = smpColor.w;
    float out_a = src_a + dist_a * (1.0 - src_a);
    vec3 out_rgb  = (smpColor.xyz * src_a + u_bgColor.xyz * dist_a * (1.0 - src_a)) / out_a + u_textColor;
    gl_FragColor = vec4(out_rgb, out_a);
    gl_FragDepthEXT = v_fragDepth;
}
