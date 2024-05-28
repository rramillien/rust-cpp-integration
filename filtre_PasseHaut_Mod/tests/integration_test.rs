#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use filtre_PasseHaut_Mod::HighPassFilter;

    #[test]
    fn test_high_pass_filter_attenuation() {
        let sample_rate = 44100.0;
        let cutoff = 1000.0;  // Fréquence de coupure à 1000 Hz
        let resonance = 0.707; // Facteur de qualité typique pour un filtre passe-haut
        let mut filter = HighPassFilter::new(sample_rate, cutoff, resonance);

        // Générer un signal sinusoïdal à 100 Hz, bien en dessous de la fréquence de coupure
        let frequency = 100.0;
        let num_samples = sample_rate as usize;
        let input_samples: Vec<f64> = (0..num_samples).map(|i| {
            (2.0 * PI * frequency * (i as f64 / sample_rate)).sin()
        }).collect();

        let mut output_samples = vec![0.0; num_samples];
        filter.apply_filter(&input_samples, &mut output_samples);

        // Vérifiez que le signal de sortie est effectivement atténué
        assert!(output_samples.iter().all(|&x| x.abs() < 0.01), "Le filtre passe-haut ne bloque pas efficacement les fréquences basses.");
    }
}
