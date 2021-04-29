attribute vec4 a_vertex;
uniform mat4 u_translate;
varying vec4 v_position;

void main() {
    v_position = u_translate * a_vertex;
    gl_Position = v_position;
}
