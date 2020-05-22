use crate::event::{EventBus, LoadResources};

use std::sync::Arc;

use crate::native::{
    Animations, RuntimeResources, Shaders, SystemView, Textures,
};

use crate::ui::{HasIntValue, HasText};

use crate::view::{BaseView, LoadingView};

use crate::view_types::ViewTypes;

pub struct LoadingPresenter<T>
where
    T: ViewTypes,
{
    view: T::LoadingView,
    system_view: Arc<T::SystemView>,
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

        let textures = Textures::<T>::new(resource_loader, &|p| {
            self.view
                .get_progress_indicator()
                .set_int_value((p * 100.) as i64);
        });

        let animations = Animations::<T>::new(resource_loader, &textures);

        let shaders = Shaders::<T>::new(resource_loader);

        (self.resources_sink)(RuntimeResources::new(
            textures, animations, shaders,
        ));

        self.view.transition_to_main_menu_view();
    }

    async fn bind(self) -> Arc<LoadingPresenter<T>> {
        let result = Arc::new(self);

        let load_resources_future =
            result.event_bus.register_for_one::<LoadResources>();

        let this = result.clone();

        let _ = result.event_bus.spawn(async move {
            if let Some(_) = load_resources_future.await {
                this.load_resources().await
            }
        });

        result
            .view
            .get_progress_indicator()
            .set_text(format!("Loading..."));

        result.event_bus.post(LoadResources {});

        result
    }

    pub async fn new(
        view: T::LoadingView,
        system_view: Arc<T::SystemView>,
        event_bus: EventBus,
        resources_sink: Box<dyn Fn(RuntimeResources<T>) + Send + Sync>,
    ) {
        view.initialize_pre_bind();

        let result: Arc<LoadingPresenter<T>> = LoadingPresenter {
            view: view,
            system_view: system_view,
            event_bus: event_bus,
            resources_sink: resources_sink,
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
