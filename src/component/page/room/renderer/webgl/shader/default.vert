attribute vec4 a_vertex;
uniform mat4 u_translate;

void main() {
    gl_Position = u_translate * a_vertex;
}
