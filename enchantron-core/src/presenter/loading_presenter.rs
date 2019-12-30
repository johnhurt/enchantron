use crate::event::{
    EnchantronEvent, EventBus, EventListener, HasListenerRegistrations,
    ListenerRegistration, LoadResources,
};

use std::sync::{Arc, Mutex};

use crate::native::{RuntimeResources, SystemView, Textures};

use crate::ui::{HasIntValue, HasText, LoadingView};

pub struct LoadingPresenter<V, S>
where
    V: LoadingView,
    S: SystemView,
{
    view: V,
    system_view: Arc<S>,
    resources_sink: Box<dyn Fn(RuntimeResources<S>) + Send + Sync>,
    event_bus: EventBus<EnchantronEvent>,
    listener_registrations: Mutex<Vec<ListenerRegistration>>,
}

impl<V, S> EventListener<EnchantronEvent, LoadResources>
    for LoadingPresenter<V, S>
where
    V: LoadingView,
    S: SystemView,
{
    fn on_event(&self, _: &LoadResources) {
        let textures =
            Textures::new(&self.system_view.get_texture_loader(), &|p| {
                self.view
                    .get_progress_indicator()
                    .set_int_value((p * 100.) as i64);
            });

        (self.resources_sink)(RuntimeResources::new(textures));

        self.view.transition_to_main_menu_view();
    }
}

impl<V, S> HasListenerRegistrations for LoadingPresenter<V, S>
where
    V: LoadingView,
    S: SystemView,
{
    fn add_listener_registration(
        &self,
        listener_registration: ListenerRegistration,
    ) {
        if let Ok(mut locked_list) = self.listener_registrations.lock() {
            info!("Adding listener registration to loading presenter");
            locked_list.push(listener_registration);
        } else {
            error!("Failed to add listener registration");
        }
    }
}

impl<V, S> LoadingPresenter<V, S>
where
    V: LoadingView,
    S: SystemView,
{
    async fn bind(self) -> Arc<LoadingPresenter<V, S>> {
        let result = Arc::new(self);

        result
            .event_bus
            .register(LoadResources::default(), Arc::downgrade(&result)).await;

        result
            .view
            .get_progress_indicator()
            .set_text(format!("Loading..."));

        result.event_bus.post(LoadResources {});

        result
    }

    pub async fn new(
        view: V,
        system_view: Arc<S>,
        event_bus: EventBus<EnchantronEvent>,
        resources_sink: Box<dyn Fn(RuntimeResources<S>) + Send + Sync>,
    ) -> Arc<LoadingPresenter<V, S>> {
        LoadingPresenter {
            view: view,
            system_view: system_view,
            event_bus: event_bus,
            resources_sink: resources_sink,
            listener_registrations: Mutex::new(Vec::new()),
        }
        .bind().await
    }
}

impl<V, S> Drop for LoadingPresenter<V, S>
where
    V: LoadingView,
    S: SystemView,
{
    fn drop(&mut self) {
        info!("Dropping Loading Presenter")
    }
}
