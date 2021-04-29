attribute vec4 a_vertex;
uniform mat4 u_translate;
varying vec4 v_position;

void main() {
    vec4 p = u_translate * a_vertex;
    v_position = p;
    gl_Position = p;
}
