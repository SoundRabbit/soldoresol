attribute vec4 a_vertex;
uniform mat4 u_translate;
varying vec3 v_vertex;

void main() {
    v_vertex = a_vertex.xyz;
    gl_Position = u_translate * a_vertex;
}
