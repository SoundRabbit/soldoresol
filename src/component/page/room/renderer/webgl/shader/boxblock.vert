attribute vec4 a_vertex;
attribute vec3 a_normal;
uniform mat4 u_translate;
varying vec3 v_vertex;
varying vec3 v_normal;

void main() {
    v_vertex = a_vertex.xyz;
    v_normal = a_normal;
    gl_Position = u_translate * a_vertex;;
}
