attribute vec4 a_vertex;
uniform mat4 u_translate;
uniform float u_pointSize;

void main() {
    gl_PointSize = u_pointSize;
    gl_Position = u_translate * a_vertex;
}
