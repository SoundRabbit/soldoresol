void defaultMain() {
    v_vertex = a_vertex.xyz;
    v_normal = a_normal;
    v_idColor = a_idColor;
    v_textureCoord = a_textureCoord;
    
    gl_Position = u_translate * a_vertex;
}