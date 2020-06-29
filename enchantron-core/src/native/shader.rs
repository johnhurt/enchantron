pub trait Shader: 'static + Send + Sync + Unpin {
    fn add_shader_variable(&self, name: String, var_type: String);
    fn add_shader_constant_f64(&self, name: String, value: f64);
    fn add_shader_constant_vec2_f64(&self, name: String, v0: f64, v1: f64);
    fn add_shader_constant_vec3_f64(
        &self,
        name: String,
        v0: f64,
        v1: f64,
        v2: f64,
    );
    fn add_shader_constant_vec4_f64(
        &self,
        name: String,
        v0: f64,
        v1: f64,
        v2: f64,
        v3: f64,
    );
}
