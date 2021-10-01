precision mediump float;

uniform mat4 u_model;
uniform mat4 u_vp;
uniform mat4 u_invModel;
uniform vec3 u_camera;
uniform int u_shape;
uniform vec4 u_idColor;
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

// vec4 convRGBA(float depth){
//     float r = fract(depth);
//     float g = fract(r * 256.0);
//     float b = fract(g * 256.0);
//     float a = fract(b * 256.0);
    
//     r = floor(r * 256.0) / 256.0;
//     g = floor(g * 256.0) / 256.0;
//     b = floor(b * 256.0) / 256.0;
//     a = floor(a * 256.0) / 256.0;

//     return vec4(r, g, b, a);
// }

float fragDepth(vec3 s) {
    vec4 p =  u_vp * vec4(s, 1.0);
    float ndc_depth = p.z / p.w;
    float far = gl_DepthRange.far;
    float near = gl_DepthRange.near;
    return (((far-near) * ndc_depth) + near + far) / 2.0;
}

float linerDepth(vec3 s) {
    vec4 p = u_vp * vec4(s.xy, s.z + 0.05, 1.0);
    float near = 0.5;
    float far  = 100.0;
    return p.z / p.w / (far - near);
}

void main(void){
    // getCameraRay
    cameraRay cr;
    {
        vec4 c = u_invModel * vec4(u_camera, 1.0);
        cr.a =c.xyz;
        cr.t = v_vertex - c.xyz;
    }

    bool disable = false;
    if(u_shape == 1) {
        // sphareShader
        float aa = dot(cr.t, cr.t);
        float bb = 2.0 * dot(cr.t, cr.a);
        float cc = dot(cr.a, cr.a) - 0.5 * 0.5;
        float dd = bb * bb - 4.0 * aa * cc;
        if(dd < 0.0) {
            disable = true;
        }
    } else if(u_shape == 2) {
        // cylinderShader
        if(length(v_vertex.xy) >= 0.5) {
            float aa = dot(cr.t.xy, cr.t.xy);
            float bb = 2.0 * dot(cr.t.xy, cr.a.xy);
            float cc = dot(cr.a.xy, cr.a.xy) - 0.5 * 0.5;
            float dd = bb * bb - 4.0 * aa * cc;
            if(dd < 0.0) {
                s.disable = true;
            } else {
                float t = (-bb - sqrt(dd)) / (2.0 * aa);
                vec3 p = cr.t * t + cr.a;
                if(p.z < -0.5 || 0.5 < p.z) {
                    s.disable = true;
                }
            }
        }
    } else {
        // cubeShader
        s.p = (u_model * vec4(v_vertex, 1.0)).xyz;
        s.disable = false;
    }
    gl_FragColor = s.disable ? vec4(0.0, 0.0, 0.0, 0.0) : convRGBA(linerDepth(s.p));
    gl_FragDepthEXT = s.disable ? 1.0 : fragDepth(s.p);
}
