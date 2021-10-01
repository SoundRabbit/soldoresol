attribute vec4 a_vertex;
attribute vec3 a_normal;
attribute vec4 a_color;
uniform mat4 u_translate;
varying vec3 v_vertex;
varying vec3 v_normal;
varying vec4 v_color;

void main() {
    v_vertex = a_vertex.xyz;
    v_normal = a_normal;
    v_color = a_color;
    gl_Position = u_translate * a_vertex;
}
