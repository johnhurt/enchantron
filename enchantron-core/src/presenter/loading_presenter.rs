use crate::application_context::Ao;
use crate::event::{EventBus, LoadResources};
use crate::native::{Animations, RuntimeResources, SystemInterop, Textures};
use crate::ui::{HasIntValue, HasText};
use crate::view::{LoadingView, NativeView};
use crate::view_types::ViewTypes;
use std::sync::Arc;

pub struct LoadingPresenter<T, V>
where
    T: ViewTypes,
    V: LoadingView,
{
    view: V,
    system_interop: Ao<T::SystemInterop>,
    resources_sink: Box<dyn Fn(RuntimeResources<T>) + Send + Sync>,
    event_bus: EventBus,
}

impl<T, V> LoadingPresenter<T, V>
where
    T: ViewTypes,
    V: LoadingView,
{
    async fn load_resources(&self) {
        let resource_loader: &T::ResourceLoader =
            &self.system_interop.get_resource_loader();

        let textures = Textures::<T>::new(resource_loader, &|p| {});

        let animations = Animations::<T>::new(resource_loader, &textures);

        (self.resources_sink)(RuntimeResources::new(textures, animations));

        self.view.transition_to_main_menu_view();
    }

    async fn bind(self) -> Arc<LoadingPresenter<T, V>> {
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
        view: V,
        system_interop: Ao<T::SystemInterop>,
        event_bus: EventBus,
        resources_sink: Box<dyn Fn(RuntimeResources<T>) + Send + Sync>,
    ) {
        let result = LoadingPresenter {
            view,
            system_interop,
            event_bus,
            resources_sink,
        }
        .bind()
        .await;

        result.view.set_presenter(Box::new(result.clone()));
    }
}

impl<T, V> Drop for LoadingPresenter<T, V>
where
    T: ViewTypes,
    V: LoadingView,
{
    fn drop(&mut self) {
        info!("Dropping Loading Presenter")
    }
}
