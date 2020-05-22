
void main() {
    vec4 BROWN = vec4(0.396, 0.263, 0.129, 1.0);
    vec4 GREEN = vec4(0.565, 0.933, 0.565, 1.0);
    
    // figure out how big individual pixels are in texture space
    vec2 one_over_size = 1.0 / 128;
    float sample_offset = one_over_size.x / 2.;
    
    float sx = floor(v_tex_coord.x / one_over_size.x) * one_over_size.x + sample_offset;
    float sy = floor(v_tex_coord.y / one_over_size.y) * one_over_size.y + sample_offset;
    
    // read the new coordinate from our texture and send it back, , taking into
    // account node transparency
    vec4 curr_color = texture2D(u_texture, vec2(sx , sy));
    
    float d = distance(BROWN, curr_color);
    
    if (d < 0.001) {
        gl_FragColor = GREEN;
    }
    else {
        gl_FragColor = BROWN;
    }
}
