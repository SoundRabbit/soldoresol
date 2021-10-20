// その他uniform変数
//必須
uniform vec3 u_cameraPosition;
//必須
uniform mat4 u_invModelMatrix;
//必須
uniform mat4 u_modelMatrix;
//必須
uniform int u_shape;
//必須
uniform mat4 u_vpMatrix;

// 背景色
//必須
uniform int u_bgColor1;
//必須
uniform int u_bgColor2;
//COLOR_NONE以外で必須
uniform vec4 u_bgColor1Value;
//COLOR_NONE以外で必須
uniform vec4 u_bgColor2Value;

// ID
//必須
uniform int u_id;
//ID_NONE以外で必須
uniform int u_idValue;

// テクスチャ
//必須
uniform int u_texture0;
//必須
uniform int u_texture1;
//必須
uniform int u_texture2;
//TEXTURE_NONE以外で必須
uniform sampler2D u_texture0Sampler;
//TEXTURE_NONE以外で必須
uniform sampler2D u_texture1Sampler;
//TEXTURE_NONE以外で必須
uniform sampler2D u_texture2Sampler;

// ライティング／シェ―ディング
//必須
uniform int u_light;
//LIGHT_POINT_WITH_IDで必須
uniform float u_lightAttenation;
//LIGHT_NONE以外で必須
uniform vec4 u_lightColor;
//LIGHT_NONE以外で必須
uniform float u_lightIntensity;
//LIGHT_POINT_WITH_IDで必須
uniform sampler2D u_lightMapNx;
//LIGHT_POINT_WITH_IDで必須
uniform sampler2D u_lightMapNy;
//LIGHT_POINT_WITH_IDで必須
uniform sampler2D u_lightMapNz;
//LIGHT_POINT_WITH_IDで必須
uniform sampler2D u_lightMapPx;
//LIGHT_POINT_WITH_IDで必須
uniform sampler2D u_lightMapPy;
//LIGHT_POINT_WITH_IDで必須
uniform sampler2D u_lightMapPz;
//LIGHT_NONE以外で必須
uniform vec3 u_lightPosition;
//LIGHT_POINT_WITH_IDで必須
uniform mat4 u_lightVpNx;
//LIGHT_POINT_WITH_IDで必須
uniform mat4 u_lightVpNy;
//LIGHT_POINT_WITH_IDで必須
uniform mat4 u_lightVpNz;
//LIGHT_POINT_WITH_IDで必須
uniform mat4 u_lightVpPx;
//LIGHT_POINT_WITH_IDで必須
uniform mat4 u_lightVpPy;
//LIGHT_POINT_WITH_IDで必須
uniform mat4 u_lightVpPz;
//LIGHT_NONE以外で必須
uniform float u_shadeIntensity;

varying vec4 v_idColor;
varying vec3 v_normal;
varying vec2 v_textureCoord;
varying vec4 v_vColor;
varying vec3 v_vertex;

#define COLOR_NONE 0x00000000
#define COLOR_SOME 0x00000001

#define ID_NONE 0x00000000
#define ID_U_READ 0x01000001
#define ID_U_WRITE 0x01000002
#define ID_V_READ 0x02000001
#define ID_V_WRITE 0x02000002

#define TEXTURE_NONE 0x00000000
#define TEXTURE_NORMAL 0x00000001
#define TEXTURE_MASK 0x00000002

#define LIGHT_NONE 0x00000000
#define LIGHT_AMBIENT 0x00000001
#define LIGHT_POINT_WITH_ID 0x01000001

#define SHAPE_2D_BOX 0x02000000
#define SHAPE_2D_CIRCLE 0x02000001
#define SHAPE_3D_BOX 0x03000000
#define SHAPE_3D_SPHERE 0x03000001
#define SHAPE_3D_CYLINDER 0x03000002
