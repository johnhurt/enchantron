use super::{Animation, ResourceLoader, Textures};
use crate::view_types::ViewTypes;

const LOOP: bool = true;

macro_rules! create_animations {
    {
        $( $name:ident: $($looped:ident)? [ $($atlas:ident.$texture:ident),* ] )*
    } => {
        pub struct Animations<T: ViewTypes> {
            $(
                pub $name: T::Animation,
            )*
        }

        impl<T: ViewTypes> Animations<T> {
            pub fn new(
                resource_loader: &T::ResourceLoader,
                textures: &Textures<T>
            ) -> Animations<T> {

                $(
                    let $name = resource_loader.create_animation();

                    $($name.set_is_loop($looped);)?
                    $name.set_name(stringify!($name).to_owned());

                    $(
                        $name.add_texture(textures.$atlas.$texture());
                    )*
                )*

                Animations {$(
                    $name
                ),*}
            }
        }
    };
}

create_animations! {
    player_walk_south: LOOP [
        character.south_rest,
        character.south_step_left,
        character.south_step_mid,
        character.south_step_right
    ]
}
