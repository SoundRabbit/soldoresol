attribute vec4 a_vertex;
attribute vec3 a_normal;
uniform mat4 u_model;
uniform mat4 u_vp;
uniform mat4 u_invModel;
uniform vec3 u_light;
uniform vec4 u_bgColor;
uniform float u_shadeIntensity;
uniform float u_envLightIntensity;
uniform int u_v_isShadowmap;
varying vec4 v_color;
varying vec4 v_position;

#define IF(x) (x != 0)

vec4 colorWithEnvLight() {
    vec3 invLight = normalize(u_invModel * vec4(u_light, 0.0)).xyz;
    float diffuse = clamp(dot(a_normal, invLight), 0.0, 1.0) * u_shadeIntensity + 1.0 - u_shadeIntensity;
    vec4 res = vec4(0.0, 0.0, 0.0, 1.0) + u_bgColor * vec4(vec3(diffuse), 1.0) * u_envLightIntensity;

    return res;
}

void main() {
    v_color = IF(u_v_isShadowmap) ? u_bgColor : colorWithEnvLight();
    vec4 p = u_model * a_vertex;
    v_position = p;
    gl_Position = u_vp * p;
}
