use std::sync::{Arc, Mutex, RwLock};

use std::sync::atomic::AtomicIsize;

use std::thread;

use std::time::{Duration, Instant};

use crate::event::{
    Evaluate, EventBus, EventListener, FourFoursEvent, Layout,
    ListenerRegistration,
};

use crate::model::{GameDisplayState, GameState, Point, Rect, Size};

use crate::native::{HasIntSize, RuntimeResources, SystemView};

use crate::ui::{
    DragHandler, GameView, HandlerRegistration, HasMutableVisibility,
    LayoutHandler,
};

lazy_static! {
    static ref MIN_EVAL_SEPARATION: Duration = Duration::from_millis(500);
}

const BOUNDARY_FRACTION: f64 = 0.04;
const SPACING_FRACTION: f64 = 0.04;
const MAX_CARD_WIDTH_FRAC: f64 = 0.2;
const MAX_CARD_HEIGHT_FRAC: f64 = 0.35;

const MIN_SUPPLY_CARD_WIDTH_PTS: f64 = 45.0;
const MIN_PLAY_CARD_WIDTH_PTS: f64 = 35.0;

const TEX_AREA_HEIGHT_FRAC: f64 = 0.3;

pub struct GamePresenter<V, S>
where
    S: SystemView,
    V: GameView<T = S::T>,
{
    view: V,
    event_bus: Arc<EventBus>,
    runtime_resources: Arc<RuntimeResources<S>>,
    listener_registrations: Mutex<Vec<ListenerRegistration>>,
    handler_registrations: Mutex<Vec<Box<HandlerRegistration>>>,

    display_state: RwLock<GameDisplayState<V::S>>,
}

impl<V, S> EventListener<Layout> for GamePresenter<V, S>
where
    S: SystemView,
    V: GameView<T = S::T>,
{
    fn on_event(&self, event: &Layout) {
        info!("Game view resized to : {}, {}", event.width, event.height);

        let mut display_state = self
            .display_state
            .write()
            .expect("Failed to lock display state for reading");
    }
}

impl<V, S> GamePresenter<V, S>
where
    S: SystemView,
    V: GameView<T = S::T>,
{
    fn add_listener_registration(&self, lr: ListenerRegistration) {
        if let Ok(mut locked_list) = self.listener_registrations.lock() {
            locked_list.push(lr);
        }
    }

    fn add_handler_registration(&self, hr: Box<HandlerRegistration>) {
        if let Ok(mut locked_list) = self.handler_registrations.lock() {
            locked_list.push(hr);
        }
    }

    /// Initialize the display state with the initial game state
    fn initialize_game_state(
        this: Arc<GamePresenter<V, S>>,
        game_state: GameState,
    ) {
        let mut new_display_state = GameDisplayState::default();

        *this
            .display_state
            .write()
            .expect("Failed to get write lock on display state") =
            new_display_state;
    }

    fn bind(self) -> Arc<GamePresenter<V, S>> {
        let copied_event_bus = self.event_bus.clone();

        self.add_handler_registration(Box::new(self.view.add_layout_handler(
            create_layout_handler!(|w, h| {
                copied_event_bus.post(Layout {
                    width: w,
                    height: h,
                })
            }),
        )));

        let result = Arc::new(self);

        result.add_listener_registration(
            result.event_bus.register_disambiguous(
                FourFoursEvent::Layout,
                &result,
                Some(Layout {
                    width: 0,
                    height: 0,
                }),
            ),
        );

        result
    }

    pub fn new(
        view: V,
        event_bus: Arc<EventBus>,
        runtime_resources: Arc<RuntimeResources<S>>,
    ) -> Arc<GamePresenter<V, S>> {
        let result = GamePresenter {
            view: view,
            event_bus: event_bus,
            runtime_resources: runtime_resources,
            listener_registrations: Mutex::new(Vec::new()),
            handler_registrations: Mutex::new(Vec::new()),

            display_state: RwLock::new(GameDisplayState::default()),
        };

        let game_state = GameState::default();

        let arc_result = result.bind();

        GamePresenter::initialize_game_state(arc_result.clone(), game_state);

        arc_result
    }
}

impl<V, S> Drop for GamePresenter<V, S>
where
    S: SystemView,
    V: GameView<T = S::T>,
{
    fn drop(&mut self) {
        info!("Dropping Game Presenter")
    }
}
