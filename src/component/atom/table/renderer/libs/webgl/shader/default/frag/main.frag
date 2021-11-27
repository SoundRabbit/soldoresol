float defaultMain() {
    g_idValue =
        u_id == ID_U_READ || u_id == ID_U_WRITE ? u_idValue
        : u_id == ID_V_READ || u_id == ID_V_WRITE ? int(floor(v_id / 2.0 + 0.5) * 2.0) + u_idValue
        : 0;

    vec4 c = u_invModelMatrix * vec4(u_cameraPosition, 1.0);
    g_cameraRay.a =c.xyz;
    g_cameraRay.t = v_vertex - c.xyz;

    bool is_disable =
        u_shape == SHAPE_2D_BOX ? setGSurfaceAs2dBox()
        : u_shape == SHAPE_2D_CIRCLE ? setGSurfaceAs2dCircle()
        : u_shape == SHAPE_3D_BOX ? setGSurfaceAs3dBox()
        : u_shape == SHAPE_3D_SPHERE ? setGSurfaceAs3dSphare()
        : u_shape == SHAPE_3D_CYLINDER ? setGSurfaceAs3dCylinder()
        : true;

    gl_FragColor =
        is_disable ? vec4(0.0)
        : u_id == ID_U_WRITE || u_id == ID_V_WRITE ? floatToRgb(float(g_idValue))
        : u_light == LIGHT_NONE ? colorWithLightAsNone()
        : u_light == LIGHT_AMBIENT ? colorWithLightAsAmbient()
        : u_light == LIGHT_POINT_WITH_ID ? colorWithLightAsPointWithId()
        : vec4(0.0);

    return is_disable ? 1.0 : fragDepth();
}
