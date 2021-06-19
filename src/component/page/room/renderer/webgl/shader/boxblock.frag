precision mediump float;

#extension GL_EXT_frag_depth : enable

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
uniform int u_shape;
varying vec3 v_vertex;
varying vec3 v_normal;

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

surface g_surface;

vec4 colorWithLight(float intensity) {
    return vec4(u_bgColor.xyz * u_lightColor.xyz * intensity, 1.0);
}

float normalVecIntensity(vec3 light) {
    float diffuse = clamp(dot(g_surface.n, light), 0.0, 1.0) * u_shadeIntensity + 1.0 - u_shadeIntensity;
    return diffuse * u_lightIntensity;
}

vec4 colorWithEnvLight() {
    float envIntensity = normalVecIntensity(normalize(u_light));
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
    float shadeIntensity = linerDepth > shadow ? 0.0 : 1.0;
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

    float envIntensity = normalVecIntensity(normalize(u_light - g_surface.p));

    float shadeIntensity = envIntensity * shadowmappedIntensity;

    float lightIntensity = u_attenation > 0.0 ? pow(u_lightIntensity / max(1.0, len - u_attenation  + 1.0), 2.0) : pow(u_lightIntensity, 2.0);
    return colorWithLight(shadeIntensity * lightIntensity);
}

float fragDepth(vec3 s) {
    vec4 p =  u_vp * vec4(s, 1.0);
    float ndc_depth = p.z / p.w;
    float far = gl_DepthRange.far;
    float near = gl_DepthRange.near;
    return (((far-near) * ndc_depth) + near + far) / 2.0;
}

void main() {

    // getCameraRay
    cameraRay cr;
    {
        vec4 c = u_invModel * vec4(u_camera, 1.0);
        cr.a =c.xyz;
        cr.t = v_vertex - c.xyz;
    }

    if(u_shape == 1) {
        // sphareShader
        float aa = dot(cr.t, cr.t);
        float bb = 2.0 * dot(cr.t, cr.a);
        float cc = dot(cr.a, cr.a) - 0.5 * 0.5;
        float dd = bb * bb - 4.0 * aa * cc;
        if(dd < 0.0) {
            g_surface.disable = true;
        } else {
            float t = (-bb - sqrt(dd)) / (2.0 * aa);
            vec3 p = cr.t * t + cr.a;
            g_surface.p = (u_model * vec4(p, 1.0)).xyz;
            g_surface.n = normalize(p);
            g_surface.disable = false;
        }
    } else if(u_shape == 2) {
        // cylinderShader
        if(length(v_vertex.xy) < 0.5) {
            g_surface.p = (u_model * vec4(v_vertex, 1.0)).xyz;
            g_surface.n = v_normal;
            g_surface.disable = false;
        } else {
            float aa = dot(cr.t.xy, cr.t.xy);
            float bb = 2.0 * dot(cr.t.xy, cr.a.xy);
            float cc = dot(cr.a.xy, cr.a.xy) - 0.5 * 0.5;
            float dd = bb * bb - 4.0 * aa * cc;
            if(dd < 0.0) {
                g_surface.disable = true;
            } else {
                float t = (-bb - sqrt(dd)) / (2.0 * aa);
                vec3 p = cr.t * t + cr.a;
                if(p.z < -0.5 || 0.5 < p.z) {
                    g_surface.disable = true;
                } else {
                    g_surface.p = (u_model * vec4(p, 1.0)).xyz;
                    g_surface.n = normalize(vec3(p.xy, 0.0));
                    g_surface.disable = false;
                }
            }
        }
    } else {
        // cubeShader
        g_surface.p = (u_model * vec4(v_vertex, 1.0)).xyz;
        g_surface.n = v_normal;
        g_surface.disable = false;
    }

    gl_FragColor =
        g_surface.disable ? vec4(0.0, 0.0, 0.0, 0.0)
        : IF(u_isShadowmap) ? shadowmapped()
        : colorWithEnvLight();
    
    gl_FragDepthEXT = g_surface.disable ? 1.0 : fragDepth(g_surface.p);
}
