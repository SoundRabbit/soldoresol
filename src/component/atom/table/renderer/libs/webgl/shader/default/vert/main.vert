void defaultMain() {
    vec3 normal = normalize(a_normal);
    vec4 vertex = vec4(a_vertex.xyz + normal * u_expand, a_vertex.w);

    v_vertex = vertex.xyz;
    v_normal = normal;
    v_id = a_id;
    v_textureCoord = a_textureCoord;
    v_vColor = a_vColor;
    
    gl_Position = u_translate * vertex;
}