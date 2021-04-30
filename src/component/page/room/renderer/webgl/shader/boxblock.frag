precision mediump float;

uniform vec4 u_bgColor;
uniform mat4 u_invModel;
uniform vec3 u_light;
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
varying vec3 v_position;
varying vec3 v_normal;

#define IF(x) (x != 0)
#define IS_MAX(x, y, z) (x>=y && x>=z)

float normalVecIntensity(vec3 invLight) {
    float diffuse = clamp(dot(v_normal, invLight), 0.0, 1.0) * u_shadeIntensity + 1.0 - u_shadeIntensity;
    return diffuse * u_lightIntensity;
}

vec4 colorWithEnvLight() {
    vec3 invLight = normalize(u_invModel * vec4(u_light, 0.0)).xyz;
    float envIntensity = normalVecIntensity(invLight);
    vec4 res = u_bgColor * vec4(vec3(envIntensity), 1.0);
    return res;
}

float restDepth(vec4 RGBA){
    const float rMask = 1.0;
    const float gMask = 1.0 / 255.0;
    const float bMask = 1.0 / (255.0 * 255.0);
    const float aMask = 1.0 / (255.0 * 255.0 * 255.0);
    float depth = dot(RGBA, vec4(rMask, gMask, bMask, aMask));
    return depth;
}

vec4 texColorAround(sampler2D tex, vec2 coord) {
    vec2 movX = vec2(1.0 / 512.0, 0.0);
    vec2 movY = vec2(0.0, 1.0 / 512.0);

    return texture2D(tex, coord);
}

float shadowmappedBy(mat4 lightVp, sampler2D shadowmap, float len) {
    vec4 pLight = lightVp * vec4(v_position, 1.0);
    vec2 texCoord = (pLight.xy / pLight.w + vec2(1.0)) * 0.5;
    float shadow = restDepth(texColorAround(shadowmap, texCoord));
    float near = 0.5;
    float far  = 100.0;
    float linerDepth = 1.0 / (far - near);
    linerDepth *= pLight.z / pLight.w;
    float shadeIntensity = smoothstep(-1.0/1024.0, 1.0/1024.0, shadow - linerDepth);
    return shadeIntensity;
}

vec4 shadowmapped() {
    vec3 lp = v_position - u_light;
    float absX = abs(lp.x);
    float absY = abs(lp.y);
    float absZ = abs(lp.z);
    float len = length(lp);

    float shadowmappedIntensity =
        IS_MAX(absX, absY, absZ) ? (lp.x > 0.0 ? shadowmappedBy(u_lightVpPx, u_shadowmapPx, len) : shadowmappedBy(u_lightVpNx, u_shadowmapNx, len)) :
        IS_MAX(absY, absZ, absX) ? (lp.y > 0.0 ? shadowmappedBy(u_lightVpPy, u_shadowmapPy, len) : shadowmappedBy(u_lightVpNy, u_shadowmapNy, len)) :
        (lp.z > 0.0 ? shadowmappedBy(u_lightVpPz, u_shadowmapPz, len) : shadowmappedBy(u_lightVpNz, u_shadowmapNz, len));

    float envIntensity = normalVecIntensity(lp);

    float shadeIntensity = min(envIntensity, shadowmappedIntensity);

    float lightIntensity = u_attenation != 0.0 ? u_lightIntensity / pow(len * u_attenation, 2.0) : u_lightIntensity;
    return u_bgColor * vec4(vec3(shadeIntensity * lightIntensity), 1.0);
}

void main() {
    gl_FragColor = IF(u_isShadowmap) ? shadowmapped() : colorWithEnvLight();
}
