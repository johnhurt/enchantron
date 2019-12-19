use specs::prelude::*;

pub struct CharacterSystem;

impl<'a> System<'a> for CharacterSystem {
    type SystemData = WriteStorage<'a, Pos>;

}
