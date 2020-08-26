use crate::event::EventBus;
use crate::view_types::ViewTypes;
use log::SetLoggerError;
use simplelog::{CombinedLogger, Config, LevelFilter, SimpleLogger};
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use tokio::runtime::{Builder, Runtime};

use crate::{GameView, LoadingView, MainMenuView, SystemView};

use crate::native::RuntimeResources;

use crate::presenter::{GamePresenter, LoadingPresenter, MainMenuPresenter};

lazy_static! {
    static ref LOGGER_RESULT: Result<(), SetLoggerError> = CombinedLogger::init(
        vec![SimpleLogger::new(LevelFilter::Debug, Config::default())]
    );
    pub static ref NUM_CPUS: usize = num_cpus::get();
}

pub struct ApplicationContext<T: ViewTypes>(Arc<ApplicationContextInner<T>>);

impl<T: ViewTypes> ApplicationContext<T> {
    pub fn new(system_view: T::SystemView) -> ApplicationContext<T> {
        if LOGGER_RESULT.is_err() {
            println!("Failed to set logger")
        }

        let runtime = Builder::new()
            .threaded_scheduler()
            .core_threads(*NUM_CPUS)
            .enable_time()
            .build()
            .unwrap_or_else(|e| {
                error!("Failed to create tokio runtime, {:?}", e);
                panic!("Failed to create tokio runtime");
            });

        let event_bus = EventBus::new(runtime.handle());

        ApplicationContext(Arc::new(ApplicationContextInner {
            tokio_runtime: runtime,
            event_bus,
            system_view: Arc::new(system_view),
            runtime_resources: RwLock::new(None),
        }))
    }
}

impl<T: ViewTypes> Deref for ApplicationContext<T> {
    type Target = ApplicationContextInner<T>;

    fn deref(&self) -> &ApplicationContextInner<T> {
        &self.0
    }
}

pub struct ApplicationContextInner<T: ViewTypes> {
    tokio_runtime: Runtime,
    event_bus: EventBus,
    system_view: Arc<T::SystemView>,
    runtime_resources: RwLock<Option<Arc<RuntimeResources<T>>>>,
}

impl<T: ViewTypes> ApplicationContext<T> {
    pub fn transition_to_loading_view(&self, view: T::LoadingView) {
        debug!("Transition to loading view");

        let self_copy = self.0.clone();

        (*self).tokio_runtime.handle().spawn(LoadingPresenter::new(
            view,
            self.system_view.clone(),
            self.event_bus.clone(),
            Box::new(move |resources| {
                self_copy.set_runtime_resources(resources);
            }),
        ));
    }

    pub fn transition_to_main_menu_view(&self, view: T::MainMenuView) {
        debug!("Transition to main menu view");

        (*self)
            .tokio_runtime
            .handle()
            .spawn(MainMenuPresenter::<T>::new(view, self.event_bus.clone()));
    }

    pub fn transition_to_game_view(&self, view: T::GameView) {
        (*self)
            .tokio_runtime
            .handle()
            .spawn(GamePresenter::<T>::new(
                view,
                self.event_bus.clone(),
                self.get_runtime_resources(),
                self.system_view.clone(),
            ));
    }
}

impl<T: ViewTypes> ApplicationContextInner<T> {
    pub fn set_runtime_resources(
        &self,
        runtime_resources: RuntimeResources<T>,
    ) {
        if let Ok(mut runtime_resources_guard) = self.runtime_resources.write()
        {
            *runtime_resources_guard = Some(Arc::new(runtime_resources));
        } else {
            error!("Failed to unlock runtime_resources for writing");
        }
    }

    pub fn get_runtime_resources(&self) -> Arc<RuntimeResources<T>> {
        if let Ok(runtime_resources_guard) = self.runtime_resources.read() {
            if let Some(runtime_resources) = runtime_resources_guard.as_ref() {
                runtime_resources.clone()
            } else {
                error!("Runtime resources has not been set");
                panic!("Runtime resources has not been set");
            }
        } else {
            error!("Failed to unlock runtime_resources for reading");
            panic!("Failed to unlock runtime_resources for reading");
        }
    }
}
