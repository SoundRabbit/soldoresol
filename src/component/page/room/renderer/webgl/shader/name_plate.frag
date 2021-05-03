precision mediump float;

uniform vec3 u_textColor1;
uniform vec3 u_textColor2;
uniform vec2 u_areaSize;
uniform sampler2D u_texture;
varying vec2 v_textureCoord;

vec4 blend(vec4 bg, vec4 fr) {
    float dist_a = bg.w;
    float src_a = fr.w;
    float out_a = dist_a * src_a + src_a * (1.0 - dist_a) + dist_a * (1.0 - src_a);
    vec3 out_rgb  = out_a > 0.0 ? (fr.xyz * src_a + bg.xyz * dist_a * (1.0 - src_a)) / out_a : vec3(0.0, 0.0, 0.0);
    return vec4(out_rgb, out_a);
}

void main() {
    float r = 0.1;
    float x = max(0.0, abs((v_textureCoord.x - 0.5) * u_areaSize.x * 2.0) - (u_areaSize.x - r));
    float y = max(0.0, abs((v_textureCoord.y - 0.5) * u_areaSize.y * 2.0) - (u_areaSize.y - r));

    if(length(vec2(x, y)) > r) {
        discard;
    }

    vec4 smpColor = texture2D(u_texture, v_textureCoord);
    vec4 text  = vec4(u_textColor1, smpColor.w);
    vec4 bg = vec4(u_textColor2, 1.0);
    gl_FragColor = blend(bg, text);
}
