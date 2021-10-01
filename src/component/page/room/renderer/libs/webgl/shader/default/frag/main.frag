vec4 colorFromId(uint id) {
    uint a = id;
    uint r = a * 0x100;
    uint g = r * 0x100;
    uint b = g * 0x100;

    a /= 0x01000000;
    r /= 0x01000000;
    g /= 0x01000000;
    b /= 0x01000000;

    return vec4(r / 255.0, g / 255.0, b / 255.0, a / 255.0);
}

void defaultMain() {
    g_idValue =
        u_id == ID_U_READ || u_id == ID_U_WRITE ? u_idValue
        : u_id == ID_V_READ || u_id == ID_V_WRITE ? ID_FROM_VEC_COLOR(v_idColor) + u_idValue
        : vec4(0.0);

    vec4 c = u_invModel * vec4(u_camera, 1.0);
    g_cameraRay.a =c.xyz;
    g_cameraRay.t = v_vertex - c.xyz;

    bool is_disable =
        u_shape == SHAPE_2D_BOX ? setGSurfaceAs2dBox()
        : u_shape == SHAPE_2D_CIRCLE ? setGSurfaceAs2dCircle()
        : u_shape == SHAPE_3D_BOX ? setGSurfaceAs3dBox()
        : u_shape == SHAPE_3D_SPHARE ? setGSurfaceAs3dSphare()
        : u_shape == SHAPE_3D_CYLINDER ? setGSurfaceAs3dCylinder()
        : true;

    gl_FragColor =
        is_disable ? vec4(0.0)
        : u_id == ID_U_WRITE || u_id == ID_V_WRITE ? colorFromId(g_idValue)
        : u_light == LIGHT_NONE ? colorWithLightAsNone()
        : u_light == LIGHT_AMBIENT ? colorWithLightAsAmbient()
        : u_light == LIGHT_POINT_WITH_ID ? colorWithLightAsPointWithId()
        : vec4(0.0);

    #ifdef USE_FRAG_DEPTH
    
    gl_FragDepthEXT = is_disable ? 1.0 : fragDepth()

    #endif
}
