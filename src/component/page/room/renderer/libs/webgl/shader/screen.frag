precision mediump float;

varying vec2 v_textureCoord;
uniform sampler2D u_texture0Sampler;
uniform vec2 u_screenSize;

vec2 texPos(vec2 screenPos, vec2 offset) {
    return (screenPos + offset) / u_screenSize;
}

void main() {
    vec2 screenPos = v_textureCoord * u_screenSize;
    gl_FragColor = texture2D(u_texture0Sampler, texPos(screenPos, vec2(0, 0)));
}
