use Oscilator_Mod_square::OscillateurSquare;

#[test]
fn test_new_oscillateur_square() {
    // Crée une nouvelle instance d'OscillateurSquare
    let osc = OscillateurSquare::new(440.0, 0.0, 44100.0, 10);

    // Vérifie que les champs sont initialisés correctement
    assert_eq!(osc.frequency, 440.0);
    assert_eq!(osc.phase, 0.0);
    assert_eq!(osc.sample_rate, 44100.0);
    assert_eq!(osc.current_sample, 0);
    assert_eq!(osc.buffer.len(), 10);
    assert!(osc.buffer.iter().all(|&x| x == 0.0));
}

#[test]
fn test_generate_and_get_next_sample_square() {
    let mut osc = OscillateurSquare::new(440.0, 0.0, 44100.0, 10);

    // Génère un échantillon et vérifie qu'il est -1.0 ou 1.0
    let sample = osc.generate_and_get_next_sample();
    assert!(sample == 1.0 || sample == -1.0);

    // Vérifie que le compteur d'échantillons est incrémenté
    assert_eq!(osc.current_sample, 1);
}

#[test]
fn test_generate_multiple_samples_square() {
    let mut osc = OscillateurSquare::new(440.0, 0.0, 44100.0, 10);

    // Génère plusieurs échantillons
    let mut samples = vec![];
    for _ in 0..10 {
        samples.push(osc.generate_and_get_next_sample());
    }

    // Vérifie que le bon nombre d'échantillons est généré
    assert_eq!(samples.len(), 10);
    assert!(samples.iter().all(|&s| s == 1.0 || s == -1.0));

    // Vérifie que le compteur d'échantillons revient à zéro après avoir parcouru le buffer
    assert_eq!(osc.current_sample, 0);
}
