struct CameraRay {
    vec3 t;
    vec3 a;
};

struct Surface {
    vec3 p;
    vec3 n;
};

CameraRay g_cameraRay;
Surface g_surface;
int g_idValue;

bool implSetGSurfaceAs2dBox() {
    g_surface.p =  (u_modelMatrix * vec4(v_vertex, 1.0)).xyz;
    g_surface.n = v_normal;

    return false;
}

bool setGSurfaceAs2dBox() {
    return u_light == LIGHT_NONE ? false : implSetGSurfaceAs2dBox();
}

bool setGSurfaceAs2dCircle() {
    float x = (v_textureCoord.x - 0.5) * 2.0;
    float y = (v_textureCoord.y - 0.5) * 2.0;

    return x * x + y * y > 1.0 ? true : setGSurfaceAs2dBox();
}

bool setGSurfaceAs3dBox() {
    return setGSurfaceAs2dBox();
}

bool implSetGSurfaceAs3dSphare(float t) {
    vec3 p = g_cameraRay.t * t + g_cameraRay.a;
    g_surface.p = (u_modelMatrix * vec4(p, 1.0)).xyz;
    g_surface.n = normalize(p);

    return false;
}

bool setGSurfaceAs3dSphare() {
    float aa = dot(g_cameraRay.t, g_cameraRay.t);
    float bb = 2.0 * dot(g_cameraRay.t, g_cameraRay.a);
    float cc = dot(g_cameraRay.a, g_cameraRay.a) - 0.5 * 0.5;
    float dd = bb * bb - 4.0 * aa * cc;

    return
        dd < 0.0 ? true
        : u_light == LIGHT_NONE ? false
        : implSetGSurfaceAs3dSphare((-bb - sqrt(dd)) / (2.0 * aa));
}

bool implSetGSurfaceAs3dCylinderWhenLight(vec3 p) {
    g_surface.p = (u_modelMatrix * vec4(p, 1.0)).xyz;
    g_surface.n = normalize(vec3(p.xy, 0.0));

    return false;
}

bool implSetGSurfaceAs3dCylinderWithT(float t) {
    vec3 p = g_cameraRay.t * t + g_cameraRay.a;

    return
        p.z < -0.5 || 0.5 < p.z ? true
        : u_light == LIGHT_NONE ? false
        : implSetGSurfaceAs3dCylinderWhenLight(p);
}

bool implSetGSurfaceAs3dCylinder() {
    float aa = dot(g_cameraRay.t.xy, g_cameraRay.t.xy);
    float bb = 2.0 * dot(g_cameraRay.t.xy, g_cameraRay.a.xy);
    float cc = dot(g_cameraRay.a.xy, g_cameraRay.a.xy) - 0.5 * 0.5;
    float dd = bb * bb - 4.0 * aa * cc;

    return dd < 0.0 ? true : implSetGSurfaceAs3dCylinderWithT((-bb - sqrt(dd)) / (2.0 * aa));
}

bool setGSurfaceAs3dCylinder() {
    return length(v_vertex.xy) < 0.5 ? setGSurfaceAs3dBox() : implSetGSurfaceAs3dCylinder();
}

#define COLOR_BLEND_OUT_A(bg, fr) (bg.w * fr.w + fr.w * (1.0 - bg.w) + bg.w * (1.0 - fr.w))
#define COLOR_BLEND_OUT_RGB(bg, fr) ((fr.xyz * fr.w + bg.xyz * bg.w * (1.0 - fr.w)) / COLOR_BLEND_OUT_A(bg, fr))
#define COLOR_BLEND_OUT(bg, fr) (vec4(COLOR_BLEND_OUT_RGB(bg, fr), COLOR_BLEND_OUT_A(bg, fr)))
#define COLOR_BLEND(bg, fr) (COLOR_BLEND_OUT_A(bg, fr) > 0.0 ? COLOR_BLEND_OUT(bg, fr) : vec4(0.0, 0.0, 0.0, 0.0))

#define COLOR_MASK(bg, mk) (vec4(bg.xyz, bg.w * (1.0 - mk.w)))

vec4 colorWithLightAsNone() {
    vec4 bgColor1 = u_bgColor1 == COLOR_NONE ? vec4(0.0) : u_bgColor1Value;
    vec4 bgColor2 = u_bgColor2 == COLOR_NONE ? vec4(0.0) : u_bgColor2Value;
    vec4 texColor0 = u_texture0 == TEXTURE_NONE ? vec4(0.0) : texture2D(u_texture0Sampler, v_textureCoord);
    vec4 texColor1 = u_texture1 == TEXTURE_NONE ? vec4(0.0) : texture2D(u_texture1Sampler, v_textureCoord);
    vec4 texColor2 = u_texture2 == TEXTURE_NONE ? vec4(0.0) : texture2D(u_texture2Sampler, v_textureCoord);

    vec4 color = COLOR_BLEND(v_vColor, COLOR_BLEND(bgColor1, bgColor2));
    color = u_texture0 == TEXTURE_MASK ? COLOR_MASK(color, texColor0) : COLOR_BLEND(color, texColor0);
    color = u_texture1 == TEXTURE_MASK ? COLOR_MASK(color, texColor1) : COLOR_BLEND(color, texColor1);
    color = u_texture2 == TEXTURE_MASK ? COLOR_MASK(color, texColor2) : COLOR_BLEND(color, texColor2);

    return color;
}

#define NORMAL_VEC_INTENSITY_DEFUSE(light) (clamp(dot(g_surface.n, light), 0.0, 1.0) * u_shadeIntensity + 1.0 - u_shadeIntensity)
#define NORMAL_VEC_INTENSITY(light) (NORMAL_VEC_INTENSITY_DEFUSE(light) * u_lightIntensity)
#define COLOR_WITH_LIGHT_INTENSITY(_i) vec4(colorWithLightAsNone() * vec4(u_lightColor.xyz * (_i), 1.0))


vec4 colorWithLightAsAmbient() {
    vec3 normalizedLp = normalize(u_lightPosition);
    float lightIntensity = NORMAL_VEC_INTENSITY(normalizedLp);
    return COLOR_WITH_LIGHT_INTENSITY(lightIntensity);
}

#define IS_MAX(x, y, z) (x>=y && x>=z)
#define LIGHT_INTENSITY(i, a, len) (a > 0.0 ? pow(i / max(1.0, len - a  + 1.0), 2.0) : pow(u_lightIntensity, 2.0))


#define ID_FROM_int_COLOR(r, g, b, a) (a * 0x01000000 + r * 0x00010000 + g * 0x00000100 + b)
#define F_TO_I(x) (int(x * 255.0))
#define ID_FROM_VEC_COLOR(_v) (ID_FROM_int_COLOR(F_TO_I(_v.x), F_TO_I(_v.y), F_TO_I(_v.z), F_TO_I(_v.w)))

#define LIGHT_MAP(x, y, z, lv) (\
    IS_MAX(x, y, z) ? (lv.x > 0.0 ? u_lightMapPx : u_lightMapNx)\
    : IS_MAX(y, z, x) ? (lv.y > 0.0 ? u_lightMapPy : u_lightMapNy)\
    : (lv.z > 0.0 ? u_lightMapPz : u_lightMapNz)\
)

vec4 colorWithLightAsPointWithId() {
    vec3 lightVec = g_surface.p - u_lightPosition;
    float absX = abs(lightVec.x);
    float absY = abs(lightVec.y);
    float absZ = abs(lightVec.z);
    float len = length(lightVec);

    mat4 lightVp =
        IS_MAX(absX, absY, absZ) ? (lightVec.x > 0.0 ? u_lightVpPx : u_lightVpNx)
        : IS_MAX(absY, absZ, absX) ? (lightVec.y > 0.0 ? u_lightVpPy : u_lightVpNy)
        : (lightVec.z > 0.0 ? u_lightVpPz : u_lightVpNz);

    vec4 pLightWorld = lightVp * vec4(g_surface.p, 1.0);
    vec2 lightMapCoord = (pLightWorld.xy / pLightWorld.w + vec2(1.0)) * 0.5;
    vec4 idColorLightMap = texture2D(u_lightMapPz, lightMapCoord);
//LIGHT_MAP(absX, absY, absZ, lightVec)
    vec3 normalizedInvLp = normalize(-lightVec.xyz);
    float lightIntensity =
        ID_FROM_VEC_COLOR(idColorLightMap) !=  g_idValue ? 0.0
        : NORMAL_VEC_INTENSITY(normalizedInvLp) * LIGHT_INTENSITY(u_lightIntensity, u_lightAttenation, len);
    return COLOR_WITH_LIGHT_INTENSITY(lightIntensity);
}

float fragDepth() {
    vec4 p =  u_vpMatrix * vec4(g_surface.p, 1.0);
    float ndc_depth = p.z / p.w;
    float far = gl_DepthRange.far;
    float near = gl_DepthRange.near;
    return (((far-near) * ndc_depth) + near + far) / 2.0;
}
