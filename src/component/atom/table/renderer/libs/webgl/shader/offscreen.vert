attribute vec4 a_vertex;
attribute vec2 a_textureCoord;
attribute vec4 a_color;
uniform mat4 u_translate;
varying vec2 v_textureCoord;
varying vec4 v_color;

void main() {
    v_textureCoord = a_textureCoord;
    v_color = a_color;
    gl_Position = u_translate * a_vertex;
}
