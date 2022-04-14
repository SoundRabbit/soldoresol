attribute float a_id;
attribute vec3 a_normal;
attribute vec2 a_textureCoord;
attribute vec4 a_vColor;
attribute vec4 a_vertex;

//必須
uniform mat4 u_translate;
//必須
uniform float u_expand;

//頂点色の編集
//必須
uniform int u_vColorMask;
//V_COLOR_MASK_SOMEで必須
uniform vec4 u_vColorMaskFillColor;
//V_COLOR_MASK_SOMEで必須
uniform vec4 u_vColorMaskStrokeColor;

varying float v_id;
varying vec3 v_normal;
varying vec2 v_textureCoord;
varying vec4 v_vColor;
varying vec3 v_vertex;

#define V_COLOR_MASK_NONE 0x00000000
#define V_COLOR_MASK_SOME 0x00000001
