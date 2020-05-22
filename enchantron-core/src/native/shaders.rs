use super::ResourceLoader;
use crate::view_types::ViewTypes;

macro_rules! define_shaders [
    ( $($name:ident: $texture_name:literal),* ) => {

        pub struct Shaders<T: ViewTypes> {
            $(
                pub $name : T::Shader
            ),*
        }

        impl <T: ViewTypes> Shaders<T> {

            pub fn new(resource_loader: &T::ResourceLoader) -> Shaders<T> {
                Shaders {
                $(
                    $name : resource_loader.load_shader($texture_name.to_owned())
                ),*
                }
            }

        }

    };
];

define_shaders![
    terrain_shader: "TerrainShader"
];
