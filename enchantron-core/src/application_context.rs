use crate::event::{EnchantronEvent, EventBus};
use log::SetLoggerError;
use simplelog::{CombinedLogger, Config, LevelFilter, SimpleLogger};
use tokio::prelude::*;
use tokio::runtime::{Builder, Runtime};

use std::ops::Deref;
use std::sync::{Arc, RwLock};

use crate::{
    GameView, LoadingView, MainMenuView, SystemView, WrappedGamePresenter,
    WrappedLoadingPresenter, WrappedMainMenuPresenter,
};

use crate::native::RuntimeResources;

use crate::presenter::{GamePresenter, LoadingPresenter, MainMenuPresenter};

lazy_static! {
    static ref LOGGER_RESULT: Result<(), SetLoggerError> = CombinedLogger::init(
        vec![SimpleLogger::new(LevelFilter::Debug, Config::default())]
    );
}

pub struct ApplicationContext(Arc<ApplicationContextInner>);

impl ApplicationContext {
    pub fn new(system_view: SystemView) -> ApplicationContext {
        if LOGGER_RESULT.is_err() {
            println!("Failed to set logger")
        }

        let runtime = Builder::new()
            .threaded_scheduler()
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

impl Deref for ApplicationContext {
    type Target = ApplicationContextInner;

    fn deref(&self) -> &ApplicationContextInner {
        &self.0
    }
}

pub struct ApplicationContextInner {
    tokio_runtime: Runtime,
    event_bus: EventBus<EnchantronEvent>,
    system_view: Arc<SystemView>,
    runtime_resources: RwLock<Option<Arc<RuntimeResources<SystemView>>>>,
}

impl ApplicationContext {
    pub fn bind_to_loading_view(
        &self,
        view: LoadingView,
    ) -> WrappedLoadingPresenter {
        let self_copy = self.0.clone();

        WrappedLoadingPresenter::new(LoadingPresenter::new(
            view,
            self.system_view.clone(),
            self.event_bus.clone(),
            Box::new(move |resources| {
                self_copy.set_runtime_resources(resources);
            }),
        ))
    }

    pub fn bind_to_main_menu_view(
        &self,
        view: MainMenuView,
    ) -> WrappedMainMenuPresenter {
        WrappedMainMenuPresenter::new(MainMenuPresenter::new(
            view,
            self.event_bus.clone(),
        ))
    }

    pub fn bind_to_game_view(&self, view: GameView) -> WrappedGamePresenter {
        WrappedGamePresenter::new(GamePresenter::new(
            view,
            self.event_bus.clone(),
            self.get_runtime_resources(),
        ))
    }
}

impl ApplicationContextInner {
    pub fn set_runtime_resources(
        &self,
        runtime_resources: RuntimeResources<SystemView>,
    ) {
        if let Ok(mut runtime_resources_guard) = self.runtime_resources.write()
        {
            *runtime_resources_guard = Some(Arc::new(runtime_resources));
        } else {
            error!("Failed to unlock runtime_resources for writing");
        }
    }

    pub fn get_runtime_resources(&self) -> Arc<RuntimeResources<SystemView>> {
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
