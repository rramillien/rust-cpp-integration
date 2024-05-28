pub struct LowPassFilter {
    pub sample_rate: f64,
    pub cutoff: f64,
    pub resonance: f64,
    pub omega: f64,
    pub sin_omega: f64,
    pub alpha: f64,
    pub cos_omega: f64,
    a1: f64,
    a2: f64,
    b0: f64,
    b1: f64,
    b2: f64,
    pub input_modified: bool,
    input_prev: [f64; 2],
    output_prev: [f64; 2],
}

impl LowPassFilter {
    pub fn new(sample_rate: f64, cutoff: f64, resonance: f64) -> Self {
        let mut filter = LowPassFilter {
            sample_rate,
            cutoff,
            resonance,
            omega: 0.0,
            sin_omega: 0.0,
            alpha: 0.0,
            cos_omega: 0.0,
            a1: 0.0,
            a2: 0.0,
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            input_modified: true,
            input_prev: [0.0, 0.0],
            output_prev: [0.0, 0.0],
        };
        filter.update_coefficients();
        filter
    }

    pub fn update_coefficients(&mut self) {
        if self.input_modified {
            self.omega = 2.0 * std::f64::consts::PI * self.cutoff / self.sample_rate;
            self.sin_omega = self.omega.sin();
            self.alpha = self.sin_omega / (2.0 * self.resonance);
            self.cos_omega = self.omega.cos();

            let a0 = 1.0 + self.alpha;

            self.a1 = (-2.0 * self.cos_omega) / a0;
            self.a2 = (1.0 - self.alpha) / a0;
            self.b0 = ((1.0 - self.cos_omega) / 2.0) / a0;
            self.b1 = -(1.0 - self.cos_omega) / a0;
            self.b2 = self.b0;

            self.input_modified = false;
        }
    }

    pub fn apply_filter(&mut self, input_array: &[f64], output_array: &mut [f64]) {
        for (i, &input) in input_array.iter().enumerate() {
            let output = self.b0 * input + self.b1 * self.input_prev[0] + self.b2 * self.input_prev[1]
                - self.a1 * self.output_prev[0] - self.a2 * self.output_prev[1];

            // Update the input and output histories
            self.input_prev[1] = self.input_prev[0];
            self.input_prev[0] = input;
            self.output_prev[1] = self.output_prev[0];
            self.output_prev[0] = output;

            if i < output_array.len() {
                output_array[i] = output;
            }
        }
    }
}
