precision mediump float;

uniform vec4 u_bgColor;
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
uniform vec3 u_camera;
uniform mat4 u_model;
uniform mat4 u_vp;
varying vec3 v_vertex;
varying vec3 v_normal;

#extension GL_EXT_frag_depth : enable

#define IF(x) (x != 0)
#define IS_MAX(x, y, z) (x>=y && x>=z)

struct cameraRay {
    vec3 t;
    vec3 a;
};

struct surface {
    vec3 p;
    vec3 n;
    bool disable;
};

vec4 colorWithLight(float intensity) {
    return vec4(u_bgColor.xyz * u_lightColor.xyz * intensity, 1.0);
}

float normalVecIntensity(vec3 invLight, vec3 n) {
    float diffuse = clamp(dot(n, invLight), 0.0, 1.0) * u_shadeIntensity + 1.0 - u_shadeIntensity;
    return diffuse * u_lightIntensity;
}

vec4 colorWithEnvLight(vec3 n) {
    vec3 invLight = normalize(u_invModel * vec4(u_light, 0.0)).xyz;
    float envIntensity = normalVecIntensity(invLight, n);
    return colorWithLight(envIntensity);
}

float restDepth(vec4 RGBA){
    const float rMask = 1.0;
    const float gMask = 1.0 / 256.0;
    const float bMask = 1.0 / (256.0 * 256.0);
    const float aMask = 1.0 / (256.0 * 256.0 * 256.0);
    float depth = dot(RGBA, vec4(rMask, gMask, bMask, aMask));
    return depth;
}

vec4 texColorAround(sampler2D tex, vec2 coord) {
    vec2 movX = vec2(1.0 / 512.0, 0.0);
    vec2 movY = vec2(0.0, 1.0 / 512.0);

    return texture2D(tex, coord);
}

float shadowmappedBy(mat4 lightVp, sampler2D shadowmap, surface s) {
    vec4 pLight = lightVp * vec4(s.p, 1.0);
    vec2 texCoord = (pLight.xy / pLight.w + vec2(1.0)) * 0.5;
    float shadow = restDepth(texColorAround(shadowmap, texCoord));
    float near = 0.5;
    float far  = 100.0;
    float linerDepth = pLight.z / pLight.w / (far - near);
    linerDepth = linerDepth;
    float shadeIntensity = smoothstep(-1.0/1024.0, 1.0/1024.0, shadow - linerDepth);
    return shadeIntensity;
}

vec4 shadowmapped(surface s) {
    vec3 lp = s.p - u_light;
    float absX = abs(lp.x);
    float absY = abs(lp.y);
    float absZ = abs(lp.z);
    float len = length(lp);

    float shadowmappedIntensity =
        IS_MAX(absX, absY, absZ) ? (lp.x > 0.0 ? shadowmappedBy(u_lightVpPx, u_shadowmapPx, s) : shadowmappedBy(u_lightVpNx, u_shadowmapNx, s)) :
        IS_MAX(absY, absZ, absX) ? (lp.y > 0.0 ? shadowmappedBy(u_lightVpPy, u_shadowmapPy, s) : shadowmappedBy(u_lightVpNy, u_shadowmapNy, s)) :
        (lp.z > 0.0 ? shadowmappedBy(u_lightVpPz, u_shadowmapPz, s) : shadowmappedBy(u_lightVpNz, u_shadowmapNz, s));

    float envIntensity = normalVecIntensity(lp, s.n);

    float shadeIntensity = min(envIntensity, shadowmappedIntensity);

    float lightIntensity = u_attenation > 0.0 ? pow(u_lightIntensity / max(1.0, len - u_attenation  + 1.0), 2.0) : pow(u_lightIntensity, 2.0);
    return colorWithLight(shadeIntensity * lightIntensity);
}

cameraRay getCameraRay() {
    vec4 c = u_invModel * vec4(u_camera, 1.0);

    cameraRay res;
    res.a =c.xyz;
    res.t = v_vertex - c.xyz;

    return res;
}

surface sphareShader(cameraRay a) {
    surface s;

    vec3 tmp_a = a.t * a.t;
    vec3 tmp_b = a.t * a.a;
    vec3 tmp_c = a.a * a.a;

    float aa = tmp_a.x + tmp_a.y + tmp_a.z;
    float bb = 2.0 * (tmp_b.x + tmp_b.y + tmp_b.z);
    float cc = tmp_c.x + tmp_c.y + tmp_c.z - 0.5 * 0.5;

    float dd = bb * bb - 4.0 * aa * cc;

    if(dd < 0.0) {
        s.disable = true;
    } else {
        float t = (-bb - sqrt(dd)) / (2.0 * aa);
        vec3 p = a.t * t + a.a;
        s.p = (u_model * vec4(p, 1.0)).xyz;
        s.n = normalize(p);
        s.disable = false;
    }

    return s;
}

surface cylinderShader(cameraRay a) {
    float r = length(a.t.xy + a.a.xy);

    surface s;

    if(r < 0.5) {
        vec3 p = a.t + a.a;
        s.p = (u_model * vec4(p, 1.0)).xyz;
        s.n = v_normal;
        s.disable = false;
    } else {
        vec2 tmp_a = a.t.xy * a.t.xy;
        vec2 tmp_b = a.t.xy * a.a.xy;
        vec2 tmp_c = a.a.xy * a.a.xy;

        float aa = tmp_a.x + tmp_a.y;
        float bb = 2.0 * (tmp_b.x + tmp_b.y);
        float cc = tmp_c.x + tmp_c.y - 0.5 * 0.5;

        float dd = bb * bb - 4.0 * aa * cc;

        if(dd < 0.0) {
            s.disable = true;
        } else {
            float t = (-bb - sqrt(dd)) / (2.0 * aa);
            vec3 p = a.t * t + a.a;
            if(p.z < -0.5 || 0.5 < p.z) {
                s.disable = true;
            } else {
                vec3 p = a.t * t + a.a;
                s.p = (u_model * vec4(p, 1.0)).xyz;
                s.n = normalize(vec3(p.xy, 0.0));
                s.disable = false;
            }
        }
    }

    return s;
}

float fragDepth(vec3 s) {
    vec4 p =  u_vp * vec4(s, 1.0);
    float ndc_depth = p.z / p.w;
    float far = gl_DepthRange.far;
    float near = gl_DepthRange.near;
    return (((far-near) * ndc_depth) + near + far) / 2.0;
}

void main() {
    surface s = cylinderShader(getCameraRay());
    gl_FragColor =
        s.disable ? vec4(0.0, 0.0, 0.0, 0.0)
        : IF(u_isShadowmap) ? shadowmapped(s)
        : colorWithEnvLight(s.n);
    
    gl_FragDepthEXT = s.disable ? 1.0 : fragDepth(s.p);
}
