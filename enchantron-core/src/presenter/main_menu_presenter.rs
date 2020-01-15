use std::sync::{Arc, Weak};

use crate::ui::{ClickHandler, HandlerRegistration, HasClickHandlers, HasText};

use crate::view::{BaseView, MainMenuView};

use crate::event::{EventBus, ListenerRegistration, StartGame};

use crate::view_types::ViewTypes;

use tokio::stream::StreamExt;
use tokio::sync::{Mutex, RwLock};

pub struct MainMenuPresenter<T: ViewTypes> {
    view: T::MainMenuView,
    handler_registrations: Mutex<Vec<Box<dyn HandlerRegistration>>>,
    listener_registrations: Mutex<Vec<ListenerRegistration>>,
    weak_self: RwLock<Option<Box<Weak<MainMenuPresenter<T>>>>>,
    event_bus: EventBus,
}

impl<T: ViewTypes> MainMenuPresenter<T> {
    async fn add_handler_registration(&self, hr: Box<dyn HandlerRegistration>) {
        self.handler_registrations.lock().await.push(hr);
    }

    /// Get a weak arc pointer to this presenter or panic if none has been
    /// created yet
    async fn weak_self(&self) -> Weak<MainMenuPresenter<T>> {
        let ref weak_self_lock = self.weak_self.read().await;

        weak_self_lock
            .as_ref()
            .map(Box::as_ref)
            .map(Clone::clone)
            .unwrap_or_else(|| {
                error!("No weak self pointer created yet");
                panic!("No weak self pointer created yet");
            })
    }

    async fn bind(self) -> Arc<MainMenuPresenter<T>> {
        let copied_event_bus = self.event_bus.clone();

        self.add_handler_registration(Box::new(
            self.view.get_start_new_game_button().add_click_handler(
                create_click_handler!({
                    copied_event_bus.post(StartGame { new: true })
                }),
            ),
        ))
        .await;

        let result = Arc::new(self);

        {
            let weak_self = Arc::downgrade(&result);
            let mut weak_self_opt = result.weak_self.write().await;

            *weak_self_opt = Some(Box::new(weak_self));
        }

        let (listener_registration, mut event_stream) =
            result.event_bus.register::<StartGame>();

        result
            .listener_registrations
            .lock()
            .await
            .push(listener_registration);

        let weak_self = result.weak_self().await;

        result.event_bus.spawn(async move {
            while let Some(event) = event_stream.next().await {
                if let Some(presenter) = weak_self.upgrade() {
                    presenter.view.transition_to_game_view();
                } else {
                    break;
                }
            }
        });

        result
            .view
            .get_start_new_game_button()
            .set_text("New Game".to_owned());

        result
    }

    pub async fn new(
        view: T::MainMenuView,
        event_bus: EventBus,
    ) -> Arc<MainMenuPresenter<T>> {
        info!("Starting to build main menu");

        view.initialize_pre_bind();

        let result = MainMenuPresenter {
            view: view,
            handler_registrations: Mutex::new(Vec::new()),
            listener_registrations: Mutex::new(Vec::new()),
            event_bus: event_bus,
            weak_self: RwLock::new(None),
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
