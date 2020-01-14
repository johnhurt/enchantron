use crate::event::{EventBus, LoadResources};

use std::sync::{Arc, Weak};

use crate::native::{RuntimeResources, SystemView, Textures};

use crate::ui::{HasIntValue, HasText};

use crate::view::{BaseView, LoadingView};

use crate::view_types::ViewTypes;

use tokio::runtime::Handle;
use tokio::stream::StreamExt;
use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct LoadingPresenter<T>
where
    T: ViewTypes,
{
    view: T::LoadingView,
    system_view: Arc<T::SystemView>,
    resources_sink: Box<dyn Fn(RuntimeResources<T::SystemView>) + Send + Sync>,
    weak_self: RwLock<Option<Box<Weak<LoadingPresenter<T>>>>>,
    event_bus: EventBus,
}

impl<T> LoadingPresenter<T>
where
    T: ViewTypes,
{
    /// Get a weak arc pointer to this presenter or panic if none has been
    /// created yet
    async fn weak_self(&self) -> Weak<LoadingPresenter<T>> {
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

    async fn load_resources(&self) {
        let textures =
            Textures::new(&self.system_view.get_texture_loader(), &|p| {
                self.view
                    .get_progress_indicator()
                    .set_int_value((p * 100.) as i64);
            });

        (self.resources_sink)(RuntimeResources::new(textures));

        self.view.transition_to_main_menu_view();
    }

    async fn bind(self) -> Arc<LoadingPresenter<T>> {
        let result = Arc::new(self);

        {
            let weak_self = Arc::downgrade(&result);
            let mut weak_self_opt = result.weak_self.write().await;

            *weak_self_opt = Some(Box::new(weak_self));
        }

        let weak_self = result.weak_self().await;
        let mut load_resources_events =
            result.event_bus.register::<LoadResources>();

        let _ = result.event_bus.spawn(async move {
            while let Some(_) = load_resources_events.next().await {
                if let Some(arc_self) = weak_self.upgrade() {
                    arc_self.load_resources().await
                }
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
        resources_sink: Box<
            dyn Fn(RuntimeResources<T::SystemView>) + Send + Sync,
        >,
    ) {
        view.initialize_pre_bind();

        let result: Arc<LoadingPresenter<T>> = LoadingPresenter {
            view: view,
            system_view: system_view,
            event_bus: event_bus,
            resources_sink: resources_sink,
            weak_self: RwLock::new(None),
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
