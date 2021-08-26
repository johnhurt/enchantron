use super::location::LocationService;
use super::{Entity, Gor, Services};
use crate::event::EventBus;
use crate::event::*;
use crate::presenter::{EntityPresenter, PlayerPresenter};
use crate::view::EntityView;
use crate::view::PlayerView;
use futures::pin_mut;
use std::collections::HashMap;
use std::collections::HashSet;
use tokio::stream::StreamExt;

pub struct ViewService {
    presenters:
        HashMap<Entity, Box<dyn EntityPresenter<View = dyn EntityView>>>,
    event_bus: EventBus,
    location_service: LocationService,
}

impl ViewService {
    pub fn new(
        services: Services,
        event_bus: EventBus,
        presenters: HashMap<
            Entity,
            Box<dyn EntityPresenter<View = dyn EntityView>>,
        >,
    ) -> ViewService {
        ViewService {
            presenters,
            event_bus,
            location_service: services.location_service(),
        }
    }

    pub async fn run(mut self) {
        let (_registration, update_stream) =
            self.event_bus.register::<ViewportChange>();

        pin_mut!(update_stream);

        let mut viewport = if let Some(viewport) = update_stream.next().await {
            viewport
        } else {
            return;
        };

        //let mut entities_in_viewport = HashSet::new();

        while let Some(event) = update_stream.next().await {
            let viewport = event.new_viewport;
        }
    }
}
