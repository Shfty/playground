use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

fn timer_loop(start_ts: Instant, interval: Duration, mut body: impl FnMut()) {
    body();

    let mut prev_ts = start_ts;
    loop {
        let ts = Instant::now();
        let delta = ts - prev_ts;
        if delta >= interval {
            body();
            prev_ts = ts;
        }
    }
}

pub trait Interpolate {
    type Factor;
    type Result;
    fn interp(self, target: Self, fac: Self::Factor) -> Self::Result;
}

impl Interpolate for f32 {
    type Factor = Self;
    type Result = Self;

    fn interp(self, target: Self, fac: Self::Factor) -> Self::Result {
        let delta = target - self;
        self + delta * fac
    }
}

impl Interpolate for f64 {
    type Factor = Self;
    type Result = Self;

    fn interp(self, target: Self, fac: Self::Factor) -> Self {
        let delta = target - self;
        self + delta * fac
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameState {
    timestamp: Instant,
    iteration: usize,
}

impl GameState {
    pub fn new(iteration: usize) -> Self {
        GameState {
            timestamp: Instant::now(),
            iteration,
        }
    }
}

impl Interpolate for GameState {
    type Factor = f64;
    type Result = f64;

    fn interp(self, target: Self, fac: Self::Factor) -> Self::Result {
        let d0 = self.iteration as f64;
        let d1 = target.iteration as f64;
        d0.interp(d1, fac)
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            timestamp: Instant::now(),
            iteration: 0,
        }
    }
}

pub struct SharedState<T>
where
    T: Copy,
{
    double_buffer: Mutex<[T; 2]>,
    count: AtomicUsize,
    dirty: AtomicBool,
}

impl<T> Default for SharedState<T>
where
    T: Copy + Default,
{
    fn default() -> Self {
        SharedState {
            double_buffer: Mutex::new([T::default(); 2]),
            count: AtomicUsize::new(0),
            dirty: AtomicBool::new(false),
        }
    }
}

fn spawn_game_thread(
    game_state: Arc<SharedState<GameState>>,
    start_ts: Instant,
    fixed_timestep: Duration,
) {
    const BLOCK_ON_RENDER: bool = true;

    std::thread::spawn(move || {
        let mut idx = 0;
        timer_loop(start_ts, fixed_timestep, move || {
            let new_state = GameState::new(idx);
            if BLOCK_ON_RENDER {
                while game_state.dirty.load(Ordering::SeqCst) {}
            }
            let mut double_buffer = game_state.double_buffer.lock().unwrap();
            double_buffer[0] = double_buffer[1];
            double_buffer[1] = new_state;
            drop(double_buffer);
            if game_state.count.load(Ordering::SeqCst) < 2 {
                game_state.count.fetch_add(1, Ordering::SeqCst);
            }
            game_state.dirty.store(true, Ordering::SeqCst);
            println!("Game\t{:?}", idx);
            idx += 1;
        });
    });
}

fn run_render_thread(
    game_state: Arc<SharedState<GameState>>,
    start_ts: Instant,
    render_timestep: Duration,
) {
    // Enter render loop
    let mut buf = [GameState::default(); 2];
    timer_loop(start_ts, render_timestep, move || {
        if game_state.dirty.load(Ordering::SeqCst) {
            buf = *game_state.double_buffer.lock().unwrap();
            game_state.dirty.store(false, Ordering::SeqCst);
        }

        // Interpolate based on the duration since the most recent frame and render
        if game_state.count.load(Ordering::SeqCst) == 2 {
            let d0 = buf[0];
            let d1 = buf[1];

            let render_delta = Instant::now().duration_since(d1.timestamp);
            let frame_delta = d1.timestamp.duration_since(d0.timestamp);
            let fac = (render_delta.as_secs_f64() / frame_delta.as_secs_f64()).min(1.0);

            let val = Interpolate::interp(d0, d1, fac);

            println!(
                "Render\t{:?}, Fac: {:?}, Interp: {:?}",
                (buf[0].iteration, buf[1].iteration),
                fac,
                val
            );
        }
    })
}

fn main() {
    let start_ts = Instant::now();

    let fixed_timestep = Duration::from_secs_f64(1.0 / 30.0);
    let render_timestep = Duration::from_secs_f64(1.0 / 120.0);

    let game_state = Arc::new(SharedState::default());
    spawn_game_thread(game_state.clone(), start_ts, fixed_timestep);
    run_render_thread(game_state, start_ts, render_timestep);
}
