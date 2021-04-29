precision mediump float;

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
uniform int u_f_isShadowmap;
varying vec4 v_color;
varying vec4 v_position;

#define IF(x) (x != 0)

#define IS_MAX(x, y, z) (x>=y && x>=z)

float restDepth(vec4 RGBA){
    const float rMask = 1.0;
    const float gMask = 1.0 / 255.0;
    const float bMask = 1.0 / (255.0 * 255.0);
    const float aMask = 1.0 / (255.0 * 255.0 * 255.0);
    float depth = dot(RGBA, vec4(rMask, gMask, bMask, aMask));
    return depth;
}

vec4 shadowmappedBy(mat4 lightVp, sampler2D shadowmap) {
    vec4 lightPosition = lightVp * v_position;
    vec2 shadowPosition = vec2((lightPosition.x + 1.0) * 0.5, (lightPosition.y + 1.0) * 0.5);
    float depth = restDepth(texture2D(shadowmap, shadowPosition));
    vec4 color = depth - 0.0001 <= lightPosition.z ? v_color : vec4(0.0, 0.0, 0.0, 1.0);

    return color;
}

vec4 shadowmapped() {
    float absX = abs(v_position.x);
    float absY = abs(v_position.y);
    float absZ = abs(v_position.z);

    vec4 color = IS_MAX(absX, absY, absZ) ? (
        v_position.x > 0.0 ? shadowmappedBy(u_lightVpPx, u_shadowmapPx) : shadowmappedBy(u_lightVpNx, u_shadowmapNx)
    ) : IS_MAX(absY, absZ, absX) ? (
        v_position.y > 0.0 ? shadowmappedBy(u_lightVpPy, u_shadowmapPy) : shadowmappedBy(u_lightVpNy, u_shadowmapNy)
    ) : (
        v_position.z > 0.0 ? shadowmappedBy(u_lightVpPz, u_shadowmapPz) : shadowmappedBy(u_lightVpNz, u_shadowmapNz)
    );

    return color;
}

void main() {
    gl_FragColor = IF(u_f_isShadowmap) ? shadowmapped() : v_color;
}
