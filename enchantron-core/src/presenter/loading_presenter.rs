use crate::application_context::Ao;
use crate::event::{EventBus, LoadResources};
use crate::native::{Animations, RuntimeResources, SystemView, Textures};
use crate::ui::{HasIntValue, HasText};
use crate::view::{LoadingView, LoadingViewImpl, NativeView};
use crate::view_types::ViewTypes;
use std::sync::Arc;

pub struct LoadingPresenter<T>
where
    T: ViewTypes,
{
    view: LoadingViewImpl<T>,
    system_view: Ao<T::SystemView>,
    resources_sink: Box<dyn Fn(RuntimeResources<T>) + Send + Sync>,
    event_bus: EventBus,
}

impl<T> LoadingPresenter<T>
where
    T: ViewTypes,
{
    async fn load_resources(&self) {
        let resource_loader: &T::ResourceLoader =
            &self.system_view.get_resource_loader();

        let textures = Textures::<T>::new(resource_loader, &|p| {});

        let animations = Animations::<T>::new(resource_loader, &textures);

        (self.resources_sink)(RuntimeResources::new(textures, animations));

        self.view.transition_to_main_menu_view();
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
        view: T::NativeView,
        system_view: Ao<T::SystemView>,
        event_bus: EventBus,
        resources_sink: Box<dyn Fn(RuntimeResources<T>) + Send + Sync>,
    ) {
        view.initialize_pre_bind();

        let result: Arc<LoadingPresenter<T>> = LoadingPresenter {
            view: LoadingViewImpl::new(view),
            system_view,
            event_bus,
            resources_sink,
        }
        .bind()
        .await;

        result.view.initialize_post_bind(Box::new(result.clone()));
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
