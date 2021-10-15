attribute vec4 a_vertex;
attribute vec2 a_textureCoord;
uniform mat4 u_translate;
varying vec2 v_textureCoord;
varying float v_fragDepth;

void main() {
    v_textureCoord = a_textureCoord;
    gl_Position = u_translate * a_vertex;

    float far = gl_DepthRange.far;
    float near = gl_DepthRange.near;
    vec4 p =  u_translate * vec4(a_vertex.x, a_vertex.y - 0.5, a_vertex.zw);
    float ndc_depth = p.z / p.w;
    v_fragDepth = (((far-near) * ndc_depth) + near + far) / 2.0;
}