pub struct Oscillateur {
    frequency: f64,        // Fréquence en Hz
    phase: f64,            // Phase initiale de l'onde en radians
    sample_rate: f64,      // Nombre d'échantillons par seconde en Hz
    current_sample: usize, // Compteur pour l'échantillon actuel
    buffer: Vec<f64>,      // Buffer pour stocker les échantillons précalculés
}

impl Oscillateur {
    pub fn new(frequency: f64, phase: f64, sample_rate: f64, buffer_size: usize) -> Self {
        let buffer = vec![0.0; buffer_size];  // Initialisation du buffer avec des zéros
        Oscillateur {
            frequency,
            phase,
            sample_rate,
            current_sample: 0,
            buffer,
        }
    }

    pub fn generate_and_get_next_sample(&mut self) -> f64 {
        if self.current_sample >= self.buffer.len() {
            self.current_sample = 0; // Réinitialiser si nous atteignons la fin du buffer
        }

        let t = (self.current_sample as f64 / self.sample_rate) + self.phase;
        let sample = (2.0 * std::f64::consts::PI * self.frequency * t).sin();

        // Mettre à jour le buffer avec le nouvel échantillon
        self.buffer[self.current_sample] = sample;
        self.current_sample += 1;

        sample
    }
}
