
void main() {
    vec4 WHITE = vec4(1., 1., 1., 1.0);
    vec4 BLACK = vec4(0., 0., 0., 1.0);
    
    // figure out how big individual pixels are in texture space
    vec2 one_over_size = 1.0 / square_texture_height;
    
    float sx = floor(v_tex_coord.x / one_over_size.x) * one_over_size.x;
    float sy = floor(v_tex_coord.y / one_over_size.y) * one_over_size.y;
    
    float grid_tolerance = 0.055 / square_texture_height;
    float tile_tolerance = grid_tolerance * 2;
    
    if (v_tex_coord.x < tile_tolerance || v_tex_coord.y < tile_tolerance
        || v_tex_coord.x > 1.0 - tile_tolerance || v_tex_coord.y > 1.0 - tile_tolerance) {
        gl_FragColor = BLACK;
        return 0;
    }
    
    if (square_texture_height < 16 && (abs(sx - v_tex_coord.x) < 0.04 / square_texture_height
        || abs(sy - v_tex_coord.y) < 0.04 / square_texture_height)) {
        gl_FragColor = WHITE;
        return 0;
    }
    
    // read the new coordinate from our texture and send it back, , taking into
    // account node transparency
    vec4 curr_color = texture2D(u_texture, vec2(sx , sy));
    
    float d = distance(BROWN, curr_color);
    
    if (d < 0.001) {
        gl_FragColor = BROWN;
    }
    else {
        gl_FragColor = GREEN;
    }
}
