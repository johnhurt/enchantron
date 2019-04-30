extern crate crossbeam_channel;
extern crate futures;
extern crate parallel_event_emitter;
extern crate rayon;
extern crate trace_error;

use event::FourFoursEvent;
use event::{EventListener, ListenerRegistration};

use self::crossbeam_channel::{Receiver, Sender};
use self::futures::future::Future;
use self::parallel_event_emitter::{EventError, ParallelEventEmitter};
use self::rayon::ThreadPoolBuilder;
use self::trace_error::Trace;

use std::sync::{Arc, Mutex};

type ConsumableFuture =
    Box<Future<Item = usize, Error = Trace<EventError>> + Send>;

pub struct EventBus {
    sink: Sender<ConsumableFuture>,
    emitter: Arc<Mutex<ParallelEventEmitter<FourFoursEvent>>>,
}

impl EventBus {
    pub fn new() -> Arc<EventBus> {
        let (sink, source): (
            Sender<ConsumableFuture>,
            Receiver<ConsumableFuture>,
        ) = crossbeam_channel::unbounded();

        let worker_count = 8;
        let pool = ThreadPoolBuilder::new()
            .num_threads(worker_count)
            .build()
            .unwrap();

        for _ in 0..worker_count {
            let copied_source = source.clone();

            pool.spawn(move || loop {
                match copied_source.recv() {
                    Ok(future) => {
                        let _ = future.wait();
                    }
                    Err(e) => warn!("Receive from channel failed: {:?}", e),
                };
            })
        }

        Arc::new(EventBus {
            sink: sink,
            emitter: Arc::new(Mutex::new(ParallelEventEmitter::default())),
        })
    }

    pub fn register<E, H>(
        &self,
        event_type: FourFoursEvent,
        handler: &Arc<H>,
    ) -> ListenerRegistration
    where
        E: Send + Clone + Into<FourFoursEvent> + 'static,
        H: EventListener<E>,
    {
        self.register_disambiguous(event_type, handler, None)
    }

    pub fn register_disambiguous<E, H>(
        &self,
        event_type: FourFoursEvent,
        handler: &Arc<H>,
        _: Option<E>,
    ) -> ListenerRegistration
    where
        E: Send + Clone + Into<FourFoursEvent> + 'static,
        H: EventListener<E>,
    {
        let weak_handler = Arc::downgrade(handler);

        let mut listener_id = 0u64;
        let borrowed_event_type = event_type.clone();
        let copied_emitter = Arc::downgrade(&self.emitter);

        let mut locked_emitter = self.emitter.lock().unwrap();

        if let Ok(real_listener_id) = locked_emitter.add_listener_value(
            event_type,
            move |arg_opt: Option<E>| {
                if let Some(handler) = weak_handler.upgrade() {
                    if let Some(arg) = arg_opt {
                        handler.on_event(&arg);
                    }
                }
                Ok(())
            },
        ) {
            listener_id = real_listener_id + 1;
        }

        ListenerRegistration::new(Box::new(move || {
            println!("Deregistering {}", listener_id);
            let copied_event_type = borrowed_event_type.clone();
            if listener_id > 0 {
                if let Some(emitter) = copied_emitter.upgrade() {
                    let mut locked_emitter = emitter.lock().unwrap();
                    let _ = locked_emitter
                        .remove_listener(copied_event_type, listener_id - 1);
                }
            }
        }))
    }

    pub fn post<E>(&self, e: E)
    where
        E: Send + Clone + Into<FourFoursEvent> + 'static,
    {
        let mut locked_emitter = self.emitter.lock().unwrap();
        let event_type = e.clone().into();
        let f = locked_emitter.emit_value(event_type, e.clone());

        self.sink.send(Box::new(f));
    }
}
