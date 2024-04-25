pub struct Oscillator<'a> {
    frequency: f64,        // Fréquence en Hz
    phase: f64,            // Phase initiale de l'onde en radians
    sample_rate: f64,      // Nombre d'échantillons par seconde en Hz
    current_sample: usize, // Compteur pour l'échantillon actuel
    buffer: &'a mut [f64]  // Buffer pour stocker les échantillons précalculés
}

impl<'a> Oscillator<'a> {
    pub fn new(frequency: f64, phase: f64, sample_rate: f64, buffer: &'a mut [f64]) -> Self {
        Oscillator {
            frequency,
            phase,
            sample_rate,
            current_sample: 0,
            buffer
        }
    }

    pub fn process(&mut self) -> &[f64] {
        for i in 0..self.buffer.len() {
            let t = (self.current_sample as f64 / self.sample_rate) + self.phase;
            self.buffer[i] = (2.0 * std::f64::consts::PI * self.frequency * t).sin();
            self.current_sample += 1;
        }

        self.buffer
    }
}