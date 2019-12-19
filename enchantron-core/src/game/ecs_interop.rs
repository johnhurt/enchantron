use specs::prelude::*;

pub struct EcsInterop {
    world: World,
    dispatcher: Dispatcher
}

impl Default for EcsInterop {

    fn default() -> EcsInterop {
        EcsInterop {
            world: World::new(),
            dispatcher: DispatcherBuilder::new().with( )
        }
    }

}