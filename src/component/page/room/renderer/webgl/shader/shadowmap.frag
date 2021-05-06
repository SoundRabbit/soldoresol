precision mediump float;

uniform mat4 u_model;
uniform mat4 u_vp;
uniform mat4 u_invModel;
uniform vec3 u_camera;
varying vec3 v_vertex;

#extension GL_EXT_frag_depth : enable

struct cameraRay {
    vec3 t;
    vec3 a;
};

struct surface {
    vec3 p;
    bool disable;
};

cameraRay getCameraRay() {
    vec4 c = u_invModel * vec4(u_camera, 1.0);

    cameraRay res;
    res.a =c.xyz;
    res.t = v_vertex - c.xyz;

    return res;
}

surface cylinderShader(cameraRay a) {
    float r = length(a.t.xy + a.a.xy);

    surface s;

    if(r < 0.5) {
        vec3 p = a.t + a.a;
        s.p = (u_model * vec4(p, 1.0)).xyz;
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
                s.disable = false;
            }
        }
    }

    return s;
}

vec4 convRGBA(float depth){
    float r = fract(depth);
    float g = fract(r * 256.0);
    float b = fract(g * 256.0);
    float a = fract(b * 256.0);
    
    r = floor(r * 256.0) / 256.0;
    g = floor(g * 256.0) / 256.0;
    b = floor(b * 256.0) / 256.0;
    a = floor(a * 256.0) / 256.0;

    return vec4(r, g, b, a);
}

float fragDepth(vec3 s) {
    vec4 p =  u_vp * vec4(s, 1.0);
    float ndc_depth = p.z / p.w;
    float far = gl_DepthRange.far;
    float near = gl_DepthRange.near;
    return (((far-near) * ndc_depth) + near + far) / 2.0;
}

float linerDepth(vec3 s) {
    vec4 p = u_vp * vec4(s, 1.0);
    float near = 0.5;
    float far  = 100.0;
    return p.z / p.w / (far - near);
}

void main(void){
    surface s = cylinderShader(getCameraRay());
    gl_FragColor = s.disable ? vec4(1.0, 1.0, 1.0, 1.0) : convRGBA(linerDepth(s.p));
    gl_FragDepthEXT = s.disable ? 1.0 : fragDepth(s.p);
}
