precision mediump float;

uniform mat4 u_invModel;
uniform vec3 u_light;
uniform vec4 u_lightColor;
uniform mat4 u_lightVpPx;
uniform mat4 u_lightVpPy;
uniform mat4 u_lightVpPz;
uniform mat4 u_lightVpNx;
uniform mat4 u_lightVpNy;
uniform mat4 u_lightVpNz;
uniform sampler2D u_shadowmapPx;
uniform sampler2D u_shadowmapPy;
uniform sampler2D u_shadowmapPz;
uniform sampler2D u_shadowmapNx;
uniform sampler2D u_shadowmapNy;
uniform sampler2D u_shadowmapNz;
uniform int u_isShadowmap;
uniform float u_shadeIntensity;
uniform float u_lightIntensity;
uniform float u_attenation;
uniform mat4 u_model;
varying vec3 v_vertex;
varying vec3 v_normal;
varying vec4 v_color;

#define IF(x) (x != 0)
#define IS_MAX(x, y, z) (x>=y && x>=z)

struct surface {
    vec3 p;
    vec3 n;
};

surface g_surface;

vec4 colorWithLight(float intensity) {
    return vec4(v_color.xyz * u_lightColor.xyz * intensity, v_color.w);
}

float normalVecIntensity(vec3 invLight) {
    float diffuse = clamp(dot(g_surface.n, invLight), 0.0, 1.0) * u_shadeIntensity + 1.0 - u_shadeIntensity;
    return diffuse * u_lightIntensity;
}

vec4 colorWithEnvLight() {
    vec3 invLight = normalize(u_invModel * vec4(u_light, 0.0)).xyz;
    float envIntensity = normalVecIntensity(invLight);
    return colorWithLight(envIntensity);
}

float shadowmappedBy(mat4 lightVp, sampler2D shadowmap) {
    vec4 pLight = lightVp * vec4(g_surface.p, 1.0);
    vec2 texCoord = (pLight.xy / pLight.w + vec2(1.0)) * 0.5;
    vec4 shadowRgba = texture2D(shadowmap, texCoord);

    const float rMask = 1.0;
    const float gMask = 1.0 / 256.0;
    const float bMask = 1.0 / (256.0 * 256.0);
    const float aMask = 1.0 / (256.0 * 256.0 * 256.0);
    float shadow = dot(shadowRgba, vec4(rMask, gMask, bMask, aMask));

    float near = 0.5;
    float far  = 100.0;
    float linerDepth = pLight.z / pLight.w / (far - near);
    linerDepth = linerDepth;
    float shadeIntensity = smoothstep(-1.0/1024.0, 1.0/1024.0, shadow - linerDepth);
    return shadeIntensity;
}

vec4 shadowmapped() {
    vec3 lp = g_surface.p - u_light;
    float absX = abs(lp.x);
    float absY = abs(lp.y);
    float absZ = abs(lp.z);
    float len = length(lp);

    float shadowmappedIntensity =
        IS_MAX(absX, absY, absZ) ? (lp.x > 0.0 ? shadowmappedBy(u_lightVpPx, u_shadowmapPx) : shadowmappedBy(u_lightVpNx, u_shadowmapNx)) :
        IS_MAX(absY, absZ, absX) ? (lp.y > 0.0 ? shadowmappedBy(u_lightVpPy, u_shadowmapPy) : shadowmappedBy(u_lightVpNy, u_shadowmapNy)) :
        (lp.z > 0.0 ? shadowmappedBy(u_lightVpPz, u_shadowmapPz) : shadowmappedBy(u_lightVpNz, u_shadowmapNz));

    float envIntensity = normalVecIntensity(lp);

    float shadeIntensity = min(envIntensity, shadowmappedIntensity);

    float lightIntensity = u_attenation > 0.0 ? pow(u_lightIntensity / max(1.0, len - u_attenation  + 1.0), 2.0) : pow(u_lightIntensity, 2.0);
    return colorWithLight(shadeIntensity * lightIntensity);
}

void main() {
    // cubeShader
    g_surface.p = (u_model * vec4(v_vertex, 1.0)).xyz;
    g_surface.n = v_normal;

    gl_FragColor =
        IF(u_isShadowmap) ? shadowmapped()
        : colorWithEnvLight();
}
