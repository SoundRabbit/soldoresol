attribute vec4 a_vertex;
attribute vec2 a_textureCoord;
varying vec2 v_textureCoord;

void main() {
    v_textureCoord = a_textureCoord;
    gl_Position = a_vertex;
}
