#[cfg(test)]
mod tests {
    use Oscilator_Mod_saw::OscillateurSaw;


    #[test]
    fn test_new_oscillator() {
        let osc = OscillateurSaw::new(440.0, 0.0, 44100.0, 1024);
        assert_eq!(osc.frequency, 440.0);
        assert_eq!(osc.phase, 0.0);
        assert_eq!(osc.sample_rate, 44100.0);
        assert_eq!(osc.buffer.len(), 1024);
        assert_eq!(osc.current_sample, 0);
    }

    #[test]
    fn test_generate_and_get_next_sample() {
        let mut osc = OscillateurSaw::new(440.0, 0.0, 44100.0, 1024);
        let first_sample = osc.generate_and_get_next_sample();
        // Teste si la première sortie est raisonnable; cela nécessiterait de connaître la sortie attendue.
        // Pour un test plus précis, considérez des valeurs connues ou calculez à la main les attentes.
        assert!(first_sample >= -1.0 && first_sample <= 1.0);

        // Vérifiez si `current_sample` est incrémenté correctement
        assert_eq!(osc.current_sample, 1);

        // Vérifiez le comportement après plusieurs échantillons
        for _ in 1..1024 {
            osc.generate_and_get_next_sample();
        }
        // Après remplissage du buffer, `current_sample` devrait revenir à 0
        assert_eq!(osc.current_sample, 0);
    }
}
