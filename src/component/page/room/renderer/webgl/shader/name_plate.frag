precision mediump float;

uniform vec3 u_textColor1;
uniform vec3 u_textColor2;
uniform sampler2D u_texture;
varying vec2 v_textureCoord;

void main() {
    vec4 smpColor = texture2D(u_texture, v_textureCoord);
    vec3 out_rgb  = (smpColor.xyz + u_textColor1) * u_textColor2;
    gl_FragColor = vec4(out_rgb, 1.0);
}
