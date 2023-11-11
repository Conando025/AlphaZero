pub struct AlphaZeroConfig {
    pub actor_count: usize,
    pub ucb_parameter: UCBParameters,
    pub noise_parameters: NoiseParameters,
    pub simulation_parameters: SimulationParameters,
    pub training_parameters: TrainingParameters,
}

impl Default for AlphaZeroConfig {
    fn default() -> Self {
        AlphaZeroConfig {
            actor_count: 5000,
            ucb_parameter: UCBParameters {
                c_base: 19652.0,
                c_init: 1.25,
            },
            noise_parameters: NoiseParameters {
                dirichlet_alpha: 0.3,
                exploration_fraction: 0.25,
            },
            simulation_parameters: SimulationParameters {
                number_of_sampling_moves: 30,
                maximum_number_of_moves: 512,
                number_of_simulations: 800,
            },
            training_parameters: TrainingParameters {
                steps: 700_000,
                checkpoint_interval: 1_000,
                window_size: 1_000_000,
                batch_size: 4096,
                weight_decay: 1e-4,
                momentum: 0.9,
                learning_rate_schedule: vec![
                    (000_000, 2e-1),
                    (100_000, 2e-2),
                    (300_000, 2e-3),
                    (500_000, 2e-4),
                ],
            },
        }
    }
}

pub struct UCBParameters {
    pub c_base: f64,
    pub c_init: f64,
}

pub struct NoiseParameters {
    pub dirichlet_alpha: f64,
    pub exploration_fraction: f64,
}

pub struct SimulationParameters {
    pub number_of_sampling_moves: usize,
    pub maximum_number_of_moves: usize,
    pub number_of_simulations: usize,
}

pub struct TrainingParameters {
    pub steps: usize,
    pub checkpoint_interval: usize,
    pub window_size: usize,
    pub batch_size: usize,

    pub weight_decay: f64,
    pub momentum: f64,
    pub learning_rate_schedule: Vec<(usize, f64)>,
}