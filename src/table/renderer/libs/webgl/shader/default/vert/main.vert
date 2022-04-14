void defaultMain() {
    vec3 normal = normalize(a_normal);
    vec4 vertex = vec4(a_vertex.xyz + normal * u_expand, a_vertex.w);

    v_vertex = vertex.xyz;
    v_normal = normal;
    v_id = a_id;
    v_textureCoord = a_textureCoord;
    v_vColor =
        u_vColorMask == V_COLOR_MASK_NONE ? a_vColor :
        vec4(u_vColorMaskStrokeColor.xyz * a_vColor.xyz + u_vColorMaskFillColor.xyz * (vec3(1.0) - a_vColor.xyz), a_vColor.w);
    
    gl_Position = u_translate * vertex;
}