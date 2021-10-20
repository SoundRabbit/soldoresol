attribute vec4 a_vertex;
attribute vec2 a_textureCoord;
attribute vec4 a_idColor;
uniform mat4 u_translate;
varying vec2 v_textureCoord;
varying vec4 v_idColor;

void main() {
    v_textureCoord = a_textureCoord;
    v_idColor = a_idColor;
    gl_Position = u_translate * a_vertex;
}
