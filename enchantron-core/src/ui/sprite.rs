use crate::native::{Animation, Shader, Texture};

use super::{
    HasMutableLocation, HasMutableSize, HasMutableVisibility, HasMutableZLevel,
};

pub trait Sprite:
    HasMutableSize
    + HasMutableLocation
    + HasMutableVisibility
    + HasMutableZLevel
    + Send
    + Sync
    + Unpin
    + 'static
{
    type T: Texture;
    type A: Animation;
    type S: Shader;

    fn set_texture(&self, texture: &Self::T);

    fn propagate_events_to(&self, event_target: &Self);

    fn remove_from_parent(&self);

    fn animate(&self, animation: &Self::A, frame_duration_sec: f64);

    fn clear_animations(&self);

    fn set_shader(&self, shader: &Self::S);

    fn clear_shader(&self);

    fn set_shader_variable_f64(&self, variable_name: String, value: f64);

    fn set_shader_variable_vec2_f64(
        &self,
        variable_name: String,
        v0: f64,
        v1: f64,
    );

    fn set_shader_variable_vec3_f64(
        &self,
        variable_name: String,
        v0: f64,
        v1: f64,
        v2: f64,
    );

    fn set_shader_variable_vec4_f64(
        &self,
        variable_name: String,
        v0: f64,
        v1: f64,
        v2: f64,
        v3: f64,
    );
}
