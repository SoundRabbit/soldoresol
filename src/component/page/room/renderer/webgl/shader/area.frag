precision mediump float;

uniform vec4 u_bgColor1;
uniform vec4 u_bgColor2;
uniform int u_flagRound;
uniform vec2 u_areaSize;
varying vec2 v_textureCoord;

vec4 maskColor() {
    float w = u_areaSize.x;
    float h = u_areaSize.y;
    float x = v_textureCoord.x;
    float y = v_textureCoord.y;
    float f = mod(mod(floor(x * w * 3.0), 2.0) + mod(floor(y * h * 3.0), 2.0), 2.0);
    return f != 0.0 ? u_bgColor2 : u_bgColor1;
}

void main() {
    float x = (v_textureCoord.x - 0.5) * 2.0;
    float y = (v_textureCoord.y - 0.5) * 2.0;
    gl_FragColor = u_flagRound != 0 ? (x * x + y * y > 1.0 ? vec4(0.0, 0.0, 0.0, 0.0) : maskColor()) : maskColor();
}