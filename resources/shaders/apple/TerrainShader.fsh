
float random(vec2 xy) {
    return fract(sin(dot(xy, vec2(12.9898,78.233))) * 43758.5453123);
}

float get_grid_color_offset(vec2 terrain_point) {
    
    bool grid_class = mod(floor(terrain_point.x), 2.0) == mod(floor(terrain_point.y), 2.0);
    
    return grid_class ? -1. : 1.;
    
}

float noise(vec2 xy) {
    float rnd = ( 1.0 - 2.0 * random(xy));
    
    float rnd_to_fifth = rnd * rnd * rnd * rnd * rnd;
    
    return abs(rnd_to_fifth) > 0.98 ? 0.4 * rnd_to_fifth : 0.2 * rnd_to_fifth;
}

float grid_scale(vec2 terrain_spec_coords, float tex_size) {
    
    float frac_x = fract(terrain_spec_coords.x);
    float frac_y = fract(terrain_spec_coords.y);
    
    float dist_to_tile_edge = min(min(frac_x, 1.0 - frac_x), min(frac_y, 1.0 - frac_y));
    
    float zoom_scaler = tex_size > 64 ? 0 : 8 / tex_size;
    
    return zoom_scaler * 1.3 * (dist_to_tile_edge < 0.1 ? dist_to_tile_edge : 0.1);
}

vec2 get_terrain_spec_coords(
         float top_left_terrain_coord_x,
         float top_left_terrain_coord_y,
         float tex_x,
         float tex_y,
         float tex_size) {
    
    float frac_x = fract(tex_x);
    float frac_y = fract(tex_y);
    
    float spec_size = 64. / tex_size;
    if (spec_size < 1) {
        spec_size = 1;
    }
    
    return vec2(
        top_left_terrain_coord_x + floor(tex_x) + floor(frac_x * spec_size) / spec_size,
        top_left_terrain_coord_y + floor(tex_y) + floor(frac_y * spec_size) / spec_size);
}

vec4 adjust_color(vec4 color, float noise) {
    for (int i = 0; i < 3; i++) {
        color[i] = sqrt(color[i] * color[i] + noise * noise * sign(noise));
    }
    return color;
}

void main() {
    vec4 WHITE = vec4(1., 1., 1., 1.0);
    vec4 BLACK = vec4(0., 0., 0., 1.0);
    
    float top_left_x = TERRAIN_RECT[0];
    float top_left_y = TERRAIN_RECT[1];
    float tex_size = TERRAIN_RECT[2];
    
    float terrain_coord_in_tex_x = v_tex_coord.x * tex_size;
    float terrain_coord_in_tex_y = v_tex_coord.y * tex_size;
    
    
    vec2 terrain_spec_coords = get_terrain_spec_coords(
        top_left_x,
        top_left_y,
        terrain_coord_in_tex_x,
        terrain_coord_in_tex_y,
        tex_size);
    
    
    float chunked_tex_x = floor(terrain_coord_in_tex_x) / tex_size;
    float chunked_tex_y = floor(terrain_coord_in_tex_y) / tex_size;
    
    float grid_tolerance = 0.055 / tex_size;
    float tile_tolerance = grid_tolerance * 2;
    
    
    // read the new coordinate from our texture and send it back, , taking into
    // account node transparency
    vec4 curr_color = texture2D(u_texture, vec2(chunked_tex_x , chunked_tex_y));
    
    float d = distance(BROWN, curr_color);
    
    vec4 result;
    
    if (d < 0.001) {
        result = BROWN;
    }
    else {
        result = GREEN;
    }
    
    float noise_val = noise(terrain_spec_coords);
    float scale = grid_scale(terrain_spec_coords, tex_size);
    float grid_offset = get_grid_color_offset(terrain_spec_coords);
    
    result = adjust_color(adjust_color(result, grid_offset * scale), noise_val);
    
    gl_FragColor = result;
}
