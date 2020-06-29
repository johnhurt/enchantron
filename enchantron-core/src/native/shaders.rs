use super::{ResourceLoader, Shader, ShaderVariableType};
use crate::game::constants::*;
use crate::view_types::ViewTypes;
use ShaderVariableType::*;

fn u8_rgb_to_f64_rgba(val: &[u8; 3], a: u8) -> [f64; 4] {
    [
        val[0] as f64 / 255.,
        val[1] as f64 / 255.,
        val[2] as f64 / 255.,
        a as f64 / 255.,
    ]
}

macro_rules! shader_var {
    ($name:ident, $arg:ident : f64) => {
        $name.add_shader_variable(
            stringify!($arg).to_owned(),
            ertewt
            Float.type_name(),
        );
    };
    ($name:ident, $arg:ident : [f64; 2]) => {
        $name.add_shader_variable(
            stringify!($arg).to_owned(),
            Vec2Float.type_name(),
        );
    };
    ($name:ident, $arg:ident : [f64; 3]) => {
        $name.add_shader_variable(
            stringify!($arg).to_owned(),
            Vec3Float.type_name(),
        );
    };
    ($name:ident, $arg:ident : [f64; 4]) => {
        $name.add_shader_variable(
            stringify!($arg).to_owned(),
            Vec4Float.type_name(),
        );
    };
    ($name:ident, $arg:ident : f64 = $arg_val:expr ) => {
        $name.add_shader_constant_f64(stringify!($arg).to_owned(), $arg_val);
    };
    ($name:ident, $arg:ident : [f64; 2] = $arg_val:expr ) => {
        $name.add_shader_constant_vec2_f64(
            stringify!($arg).to_owned(),
            ($arg_val)[0],
            ($arg_val)[1]
        );
    };
    ($name:ident, $arg:ident : [f64; 3] = $arg_val:expr ) => {
        $name.add_shader_constant_vec3_f64(
            stringify!($arg).to_owned(),
            ($arg_val)[0],
            ($arg_val)[1],
            ($arg_val)[2]
        );
    };
    ($name:ident, $arg:ident : [f64; 4] = $arg_val:expr ) => {
        $name.add_shader_constant_vec4_f64(
            stringify!($arg).to_owned(),
            ($arg_val)[0],
            ($arg_val)[1],
            ($arg_val)[2],
            ($arg_val)[3],
        );
    };
}

macro_rules! shader_vars {
    ($name:ident, { $arg:ident : $var_type:tt $( = $var_val:expr)? }) => {
        shader_var! { $name, $arg : $var_type $( = $var_val )? }
    };
    ($name:ident, { $arg:ident : $var_type:tt $( = $var_val:expr)? $(, $more_arg:ident : $more_var_type:tt $( = $more_var_val:expr)? )+ }) => {
        shader_var! { $name, $arg : $var_type $( = $var_val )? }
        shader_vars!{ $name, { $( $more_arg : $more_var_type $( = $more_var_val )? ),+ }};
    }
}

macro_rules! define_shaders {
    ( $($name:ident = $shader_name:ident $var_block:tt );* ) => {

        pub struct Shaders<T: ViewTypes> {
            $(
                pub $name : T::Shader
            ),*
        }

        impl <T: ViewTypes> Shaders<T> {

            pub fn new(resource_loader: &T::ResourceLoader) -> Shaders<T> {

                $(
                    let $name = resource_loader.load_shader(stringify!($shader_name).to_owned());
                    shader_vars!{
                        $name, $var_block
                    };
                )*

                Shaders {
                $(
                    $name
                ),*
                }
            }

        }

    }
}

define_shaders! {
    terrain_shader = TerrainShader {
        TERRAIN_RECT: [f64; 4],
        GREEN: [f64; 4] = u8_rgb_to_f64_rgba(&GRASS_GREEN_RGB, 255),
        BROWN: [f64; 4] = u8_rgb_to_f64_rgba(&DIRT_BROWN_RGB, 255)
    }
}
