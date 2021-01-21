precision mediump float;

#extension GL_EXT_frag_depth : enable

uniform vec4 u_bgColor;
uniform int u_flagRound;
varying vec2 v_textureCoord;

void main() {
    float x = (v_textureCoord.x - 0.5) * 2.0;
    float y = (v_textureCoord.y - 0.5) * 2.0;
    if(u_flagRound != 0 && x * x + y * y > 1.0) {
        // 深度バッファに書き込みたくないのでdiscard
        discard;
    } else {
        gl_FragColor = u_bgColor;
    }
}
