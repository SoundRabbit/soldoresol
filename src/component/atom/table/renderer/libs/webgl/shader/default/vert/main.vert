void defaultMain() {
    v_vertex = a_vertex.xyz;
    v_normal = a_normal;
    v_id = a_id;
    v_textureCoord = a_textureCoord;
    v_vColor = a_vColor;
    
    gl_Position = u_translate * a_vertex;
}