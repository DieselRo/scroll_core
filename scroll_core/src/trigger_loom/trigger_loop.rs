// trigger_loop.rs – The Pendulum of the Archive

#[derive(Debug, Clone)]
pub enum SymbolicRhythm {
    Constant(f32), // Hz
    Dawn,
    Dusk,
    Spiral(u32), // Recursive step rhythm
    EmotionDriven,
}
/*
#[derive(Debug, Clone)]
pub struct TriggerLoopConfig {
    pub rhythm: SymbolicRhythm,
    pub max_invocations_per_tick: usize,
    pub allow_test_ticks: bool,
    pub emotional_signature: Option<EmotionSignature>,
}

pub trait PulseSensitive {
    fn should_awaken(&self, tick: u64) -> bool;
}

pub struct TriggerLoopEngine {
    config: TriggerLoopConfig,
    tick_counter: u64,
    agent_depth: HashMap<String, u32>,
}

impl TriggerLoopEngine {
    pub fn new(config: TriggerLoopConfig) -> Self {
        Self {
            config,
            tick_counter: 0,
            agent_depth: HashMap::new(),
        }
    }

    pub fn start_loop(&mut self, constructs: &mut [Box<dyn NamedConstruct>]) {
        let base_freq = self.resolve_frequency();
        let interval = Duration::from_secs_f32(1.0 / base_freq);

        loop {
            let now = Instant::now();
            self.tick_once(constructs);
            let elapsed = now.elapsed();
            if elapsed < interval {
                thread::sleep(interval - elapsed);
            }
        }
    }

    pub fn tick_once(&mut self, constructs: &mut [Box<dyn NamedConstruct>]) {
        self.tick_counter += 1;

        let mut fired_count = 0;
        for construct in constructs.iter_mut() {
            if let Some(pulse) = construct.as_pulse_sensitive() {
                if pulse.should_awaken(self.tick_counter) {
                    let cost = evaluate_construct(construct);
                    // TODO: Handle cost decision logic
                    fired_count += 1;
                }
            }
            if fired_count >= self.config.max_invocations_per_tick {
                break;
            }
        }
        // TODO: Log tick summary and possible TriggerLoopInvocation event
    }

    fn resolve_frequency(&self) -> f32 {
        match &self.config.rhythm {
            SymbolicRhythm::Constant(hz) => *hz,
            SymbolicRhythm::EmotionDriven => {
                if let Some(emotion) = &self.config.emotional_signature {
                    modulate_frequency(1.0, emotion) // base 1Hz
                } else {
                    1.0
                }
            }
            _ => 1.0, // Default fallback
        }
    }
}

pub fn modulate_frequency(base: f32, emotion: &EmotionSignature) -> f32 {
    // TODO: Replace with true emotional mapping
    match emotion.intensity.round() as i32 {
        0 => base * 0.5,
        1 => base * 0.8,
        2 => base,
        3 => base * 1.5,
        _ => base * 2.0,
    }
}
*/
