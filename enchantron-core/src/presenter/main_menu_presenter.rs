use crate::event::{EventBus, StartGame};
use crate::ui::{ClickHandler, HandlerRegistration, HasClickHandlers, HasText};
use crate::view::{BaseView, MainMenuView};
use crate::view_types::ViewTypes;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MainMenuPresenter<T: ViewTypes> {
    view: T::MainMenuView,
    handler_registrations: Mutex<Vec<Box<dyn HandlerRegistration>>>,
    event_bus: EventBus,
}

impl<T: ViewTypes> MainMenuPresenter<T> {
    async fn bind(self) -> Arc<MainMenuPresenter<T>> {
        let copied_event_bus = self.event_bus.clone();

        let result = Arc::new(self);

        let start_game_event_future =
            result.event_bus.register_for_one::<StartGame>();

        let this = result.clone();

        result.event_bus.spawn(async move {
            if start_game_event_future.await.is_some() {
                this.view.transition_to_game_view();
            }
        });

        result
    }

    pub async fn new(
        view: T::MainMenuView,
        event_bus: EventBus,
    ) -> Arc<MainMenuPresenter<T>> {
        info!("Starting to build main menu");

        view.initialize_pre_bind();

        let result = MainMenuPresenter {
            view,
            handler_registrations: Mutex::new(Vec::new()),
            event_bus,
        };

        let result: Arc<MainMenuPresenter<T>> = result.bind().await;

        result.view.initialize_post_bind(Box::new(result.clone()));

        result
    }
}

impl<T: ViewTypes> Drop for MainMenuPresenter<T> {
    fn drop(&mut self) {
        info!("Dropping Main Menu Presenter")
    }
}
