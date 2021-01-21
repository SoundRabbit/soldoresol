attribute vec4 a_vertex;
attribute vec2 a_textureCoord;
uniform mat4 u_translate;
varying vec2 v_textureCoord;

void main() {
    v_textureCoord = a_textureCoord;
    gl_Position = u_translate * a_vertex;
}
