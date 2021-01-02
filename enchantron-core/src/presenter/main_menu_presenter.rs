use super::GamePresenter;
use crate::application_context::Ao;
use crate::event::{EventBus, StartGame};
use crate::native::{RuntimeResources, SystemInterop};
use crate::ui::{
    ClickHandler, HandlerRegistration, HasClickHandlers, HasText,
    TransitionService,
};
use crate::view::{MainMenuView, NativeView};
use crate::view_types::ViewTypes;
use std::sync::Arc;

pub struct MainMenuPresenter<T: ViewTypes> {
    view: T::MainMenuView,
    handler_registrations: Vec<Box<dyn HandlerRegistration>>,
    system_interop: Ao<T::SystemInterop>,
    runtime_resources: Ao<RuntimeResources<T>>,
    event_bus: EventBus,
}

impl<T: ViewTypes> MainMenuPresenter<T> {
    async fn bind(mut self) -> Arc<MainMenuPresenter<T>> {
        let copied_event_bus = self.event_bus.clone();

        let click_handler = create_click_handler!({
            copied_event_bus.post(StartGame::new(true))
        });

        self.handler_registrations.push(Box::new(
            self.view
                .get_start_new_game_button()
                .add_click_handler(click_handler),
        ));

        let result = Arc::new(self);

        let start_game_event_future =
            result.event_bus.register_for_one::<StartGame>();

        let this = result.clone();

        result.event_bus.spawn(async move {
            if start_game_event_future.await.is_some() {
                GamePresenter::<T>::run(
                    this.system_interop.create_game_view(),
                    this.event_bus.clone(),
                    this.runtime_resources.clone(),
                    this.system_interop.clone(),
                )
                .await;
            }
        });

        result
    }

    pub async fn new(
        system_interop: Ao<T::SystemInterop>,
        runtime_resources: Ao<RuntimeResources<T>>,
        event_bus: EventBus,
    ) -> Arc<MainMenuPresenter<T>> {
        info!("Starting to build main menu");

        let view = system_interop.create_main_menu_view();

        let result = MainMenuPresenter {
            view,
            handler_registrations: Default::default(),
            event_bus,
            system_interop: system_interop.clone(),
            runtime_resources,
        };

        let result: Arc<MainMenuPresenter<T>> = result.bind().await;

        result.view.set_presenter(Box::new(result.clone()));

        system_interop
            .get_transition_service()
            .transition_to_main_menu_view(&result.view, true);

        result
    }
}

impl<T: ViewTypes> Drop for MainMenuPresenter<T> {
    fn drop(&mut self) {
        info!("Dropping Main Menu Presenter")
    }
}
