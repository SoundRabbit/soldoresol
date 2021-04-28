attribute vec4 a_vertex;
attribute vec3 a_normal;
uniform mat4 u_translate;
uniform mat4 u_invModel;
uniform vec3 u_light;
uniform vec4 u_bgColor;
uniform float u_shadeIntensity;
uniform float u_envLightIntensity;
varying vec4 v_color;

void main() {
    vec3 invLight = normalize(u_invModel * vec4(u_light, 0.0)).xyz;
    float diffuse = clamp(dot(a_normal, invLight), 0.0, 1.0) * u_shadeIntensity + 1.0 - u_shadeIntensity;
    v_color = vec4(0.0, 0.0, 0.0, 1.0) + u_bgColor * vec4(vec3(diffuse), 1.0) * u_envLightIntensity;

    vec4 p = u_translate * a_vertex;
    gl_Position = p;
}
