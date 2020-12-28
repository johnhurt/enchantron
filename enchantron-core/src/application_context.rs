use crate::event::EventBus;
use crate::native::{RuntimeResources, SystemInterop};
use crate::presenter::{GamePresenter, LoadingPresenter, MainMenuPresenter};
use crate::view::LoadingViewImpl;
use crate::view_types::ViewTypes;
use log::SetLoggerError;
use simplelog::{CombinedLogger, Config, LevelFilter, SimpleLogger};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::{Deref, Drop};
use std::sync::{Arc, Mutex, RwLock};
use tokio::runtime::{Builder, Runtime};

lazy_static! {
    static ref LOGGER_RESULT: Result<(), SetLoggerError> = CombinedLogger::init(
        vec![SimpleLogger::new(LevelFilter::Debug, Config::default())]
    );
    pub static ref NUM_CPUS: usize = num_cpus::get();
}

/// Reference that is owned by the application context, and can be shared freely
/// throughout the application, but not modified
pub struct Ao<T> {
    value: *const T,
}

// SAFETY: Application owned pointers can only be accessed as shared ref while
// the application runtime is running, and will only be dropped after the
// runtime is dropped
unsafe impl<T> Send for Ao<T> where T: Send + Sync {}
unsafe impl<T> Sync for Ao<T> where T: Send + Sync {}

impl<T> Ao<T> {
    pub fn new(original: &Box<T>) -> Ao<T> {
        Ao {
            value: original.deref(),
        }
    }
}

impl<T> Clone for Ao<T> {
    fn clone(&self) -> Self {
        Ao { value: self.value }
    }
}

impl<T> Debug for Ao<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.deref().fmt(f)
    }
}

impl<T> Deref for Ao<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: This value cannot be written to, and since it is owned by
        // the application context, it can be used by any parts of the
        // application because we make sure the application runtime is shutdown
        // before the boxes they are associated with are dropped
        unsafe { &*self.value }
    }
}

pub struct ApplicationContext<T: ViewTypes>(Arc<ApplicationContextInner<T>>);

impl<T: ViewTypes> ApplicationContext<T> {
    pub fn new(system_interop: T::SystemInterop) -> ApplicationContext<T> {
        if LOGGER_RESULT.is_err() {
            println!("Failed to set logger")
        }

        let boxed_runtime = Box::new(
            Builder::new_multi_thread()
                .worker_threads(*NUM_CPUS)
                .thread_name("ApplicationThread")
                .enable_time()
                .build()
                .unwrap_or_else(|e| {
                    error!("Failed to create tokio runtime, {:?}", e);
                    panic!("Failed to create tokio runtime");
                }),
        );

        let runtime = Ao::new(&boxed_runtime);

        let runtime_dropper = move || drop(boxed_runtime);

        let (event_bus, eb_dropper) = EventBus::new(runtime.clone());

        let boxed_system_interop = Box::new(system_interop);
        let system_interop = Ao::new(&boxed_system_interop);

        let system_interop_dropper = move || drop(boxed_system_interop);

        ApplicationContext(Arc::new(ApplicationContextInner {
            tokio_runtime: runtime,
            event_bus,
            system_interop,
            runtime_resources: RwLock::new(None),
            ao_droppers: Mutex::new(vec![
                Box::new(runtime_dropper),
                Box::new(eb_dropper),
                Box::new(system_interop_dropper),
            ]),
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
    tokio_runtime: Ao<Runtime>,
    event_bus: EventBus,
    system_interop: Ao<T::SystemInterop>,
    runtime_resources: RwLock<Option<Ao<RuntimeResources<T>>>>,

    ao_droppers: Mutex<Vec<Box<dyn FnOnce() + Send>>>,
}

impl<T: ViewTypes> ApplicationContext<T> {
    pub fn transition_to_loading_view(&self) {
        debug!("Transition to loading view");

        let self_copy = self.0.clone();

        let view: LoadingViewImpl<T> = LoadingViewImpl::new_loading_view(
            self.system_interop.create_native_view(),
        );

        (*self).tokio_runtime.spawn(LoadingPresenter::new(
            view,
            self.system_interop.clone(),
            self.event_bus.clone(),
            Box::new(move |resources| {
                self_copy.set_runtime_resources(resources);
            }),
        ));
    }

    pub fn transition_to_main_menu_view(&self, view: T::NativeView) {
        debug!("Transition to main menu view");

        (*self)
            .tokio_runtime
            .spawn(MainMenuPresenter::<T>::new(view, self.event_bus.clone()));
    }

    pub fn transition_to_game_view(&self, view: T::NativeView) {
        (*self).tokio_runtime.spawn(GamePresenter::<T>::run(
            view,
            self.event_bus.clone(),
            self.get_runtime_resources(),
            self.system_interop.clone(),
        ));
    }
}

impl<T: ViewTypes> ApplicationContextInner<T> {
    pub fn set_runtime_resources(
        &self,
        runtime_resources: RuntimeResources<T>,
    ) {
        let boxed_runtime_resources = Box::new(runtime_resources);
        let runtime_resources_ao = Ao::new(&boxed_runtime_resources);
        let runtime_resources_dropper = move || drop(boxed_runtime_resources);

        if let (Ok(mut runtime_resources_guard), Ok(mut droppers_guard)) =
            (self.runtime_resources.write(), self.ao_droppers.lock())
        {
            *runtime_resources_guard = Some(runtime_resources_ao);
            droppers_guard.push(Box::new(runtime_resources_dropper));
        } else {
            error!("Failed to unlock runtime_resources for writing");
        }
    }

    pub fn get_runtime_resources(&self) -> Ao<RuntimeResources<T>> {
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

impl<T> Drop for ApplicationContextInner<T>
where
    T: ViewTypes,
{
    fn drop(&mut self) {
        // Run the droppers in the stored order so that the runtime is dropped
        // first. This is what ensures that the whole Ao system is safe
        for ao_dropper in self.ao_droppers.lock().unwrap().drain(..) {
            ao_dropper();
        }
    }
}
