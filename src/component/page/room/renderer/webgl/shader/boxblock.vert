precision mediump float;

attribute vec4 a_vertex;
attribute vec3 a_normal;
uniform mat4 u_model;
uniform mat4 u_vp;
varying vec3 v_position;
varying vec3 v_normal;

#define IF(x) (x != 0)
#define IS_MAX(x, y, z) (x>=y && x>=z)

void main() {
    vec4 p = u_model * a_vertex;
    vec4 pCamera = u_vp * p;

    v_position = p.xyz;
    v_normal = a_normal;
    
    gl_Position = pCamera;
}
