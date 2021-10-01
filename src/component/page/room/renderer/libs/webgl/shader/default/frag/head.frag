// その他uniform変数
uniform vec3 u_cameraPosition;
uniform mat4 u_invModelMatrix;
uniform mat4 u_modelMatrix;
uniform int u_shape;
uniform mat4 u_vpMatrix;

// 背景色
uniform int u_bgColor1;
uniform int u_bgColor2;
uniform vec4 u_bgColor1Value;
uniform vec4 u_bgColor2Value;

// ID
uniform int u_id;
uniform int u_idValue;

// テクスチャ
uniform int u_texture0;
uniform int u_texture1;
uniform int u_texture2;
uniform sampler2D u_texture0Sampler;
uniform sampler2D u_texture1Sampler;
uniform sampler2D u_texture2Sampler;

// ライティング／シェ―ディング
uniform int u_light;
uniform float u_lightAttenation;
uniform vec4 u_lightColor;
uniform float u_lightIntensity;
uniform sampler2D u_lightMapNx;
uniform sampler2D u_lightMapNy;
uniform sampler2D u_lightMapNz;
uniform sampler2D u_lightMapPx;
uniform sampler2D u_lightMapPy;
uniform sampler2D u_lightMapPz;
uniform vec3 u_lightPosition;
uniform mat4 u_lightVpNx;
uniform mat4 u_lightVpNy;
uniform mat4 u_lightVpNz;
uniform mat4 u_lightVpPx;
uniform mat4 u_lightVpPy;
uniform mat4 u_lightVpPz;
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
#define TEXTURE_MASK 0x00000001

#define LIGHT_NONE 0x00000000
#define LIGHT_AMBIENT 0x00000001
#define LIGHT_POINT_WITH_ID 0x01000001

#define SHAPE_2D_BOX 0x02000000
#define SHAPE_2D_CIRCLE 0x02000001
#define SHAPE_3D_BOX 0x03000000
#define SHAPE_3D_SPHARE 0x03000001
#define SHAPE_3D_CYLINDER 0x03000002
