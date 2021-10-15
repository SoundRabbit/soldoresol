precision mediump float;

uniform vec4 u_idColor;
uniform int u_flagRound;
varying vec2 v_textureCoord;
varying vec4 v_idColor;

vec4 roundedColor() {
    float x = (v_textureCoord.x - 0.5) * 2.0;
    float y = (v_textureCoord.y - 0.5) * 2.0;
    return x * x + y * y > 1.0 ? vec4(0.0, 0.0, 0.0, 0.0) : u_bgColor;
}

void main() {
    gl_FragColor = u_flagRound != 0 ? roundedColor() : u_bgColor + v_color;
}
