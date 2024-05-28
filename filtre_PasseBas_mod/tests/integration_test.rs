// tests/integration_test.rs
use filtre_PasseBas_mod::LowPassFilter;

#[cfg(test)]
mod tests {
    use filtre_PasseBas_mod::LowPassFilter;

    // Test pour vérifier que le filtre passe-bas atténue correctement les fréquences proches de la fréquence de coupure
    #[test]
    fn test_low_pass_filter_attenuation() {
        let sample_rate = 44100.0;
        let cutoff = 1000.0; // Fréquence de coupure à 1000 Hz
        let resonance = 0.707;
        let mut filter = LowPassFilter::new(sample_rate, cutoff, resonance);

        // Générer un signal sinusoïdal à la fréquence juste au-dessus de la fréquence de coupure
        let freq_test = 1500.0; // 1500 Hz, au-dessus de la fréquence de coupure
        let input_samples: Vec<f64> = (0..sample_rate as usize).map(|i| {
            let t = i as f64 / sample_rate;
            (2.0 * std::f64::consts::PI * freq_test * t).sin()
        }).collect();

        let mut output_samples = vec![0.0; input_samples.len()];
        filter.apply_filter(&input_samples, &mut output_samples);

        // Calculer l'atténuation en décibels
        let input_power = input_samples.iter().map(|&sample| sample * sample).sum::<f64>();
        let output_power = output_samples.iter().map(|&sample| sample * sample).sum::<f64>();
        let attenuation_db = 10.0 * ((output_power / input_power).log10());

        // On s'attend à une atténuation significative pour les fréquences au-dessus de la fréquence de coupure
        assert!(attenuation_db < -3.0, "L'atténuation pour les fréquences élevées devrait être significative.");
    }
}
