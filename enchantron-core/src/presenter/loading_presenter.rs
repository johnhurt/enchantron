use super::MainMenuPresenter;
use crate::application_context::Ao;
use crate::event::{EventBus, LoadResources};
use crate::native::{Animations, RuntimeResources, SystemInterop, Textures};
use crate::ui::{HasIntValue, HasMutableFloatValue, TransitionService};
use crate::view::{LoadingView, NativeView};
use crate::view_types::ViewTypes;
use std::sync::Arc;

pub struct LoadingPresenter<T>
where
    T: ViewTypes,
{
    view: T::LoadingView,
    system_interop: Ao<T::SystemInterop>,
    resources_sink: Box<dyn Fn(RuntimeResources<T>) + Send + Sync>,
    event_bus: EventBus,
}

impl<T> LoadingPresenter<T>
where
    T: ViewTypes,
{
    async fn load_resources(&self) {
        let resource_loader: &T::ResourceLoader =
            &self.system_interop.get_resource_loader();

        let progress_bar = self.view.get_progress_bar();

        let textures = Textures::<T>::new(resource_loader, &|p| {
            progress_bar.set_value(p);
        });

        let animations = Animations::<T>::new(resource_loader, &textures);

        (self.resources_sink)(RuntimeResources::new(textures, animations));

        println!("done loading resources");

        MainMenuPresenter::<T>::new(
            self.system_interop.clone(),
            self.event_bus.clone(),
        )
        .await;
    }

    async fn bind(self) -> Arc<LoadingPresenter<T>> {
        let result = Arc::new(self);

        let load_resources_future =
            result.event_bus.register_for_one::<LoadResources>();

        let this = result.clone();

        let _ = result.event_bus.spawn(async move {
            if load_resources_future.await.is_some() {
                this.load_resources().await
            }
        });

        result.event_bus.post(LoadResources {});

        result
    }

    pub async fn new(
        view: T::LoadingView,
        system_interop: Ao<T::SystemInterop>,
        event_bus: EventBus,
        resources_sink: Box<dyn Fn(RuntimeResources<T>) + Send + Sync>,
    ) {
        let si = system_interop.clone();

        let result = LoadingPresenter {
            view,
            system_interop,
            event_bus,
            resources_sink,
        }
        .bind()
        .await;

        result.view.set_presenter(Box::new(result.clone()));
        si.get_transition_service()
            .transition_to_loading_view(&result.view, true);
    }
}

impl<T> Drop for LoadingPresenter<T>
where
    T: ViewTypes,
{
    fn drop(&mut self) {
        info!("Dropping Loading Presenter")
    }
}
