use filtre_PasseBas_mod::LowPassFilter;

fn main() {
    // Création d'un tableau d'entrée avec des valeurs d'exemple
    let input_samples: Vec<f64> = vec![/* valeurs de votre oscillateur ou autres valeurs de test */];
    let mut output_samples: Vec<f64> = vec![0.0; input_samples.len()];

    // Création de l'instance du filtre
    let mut filter = LowPassFilter::new(44100.0, 220.0, 0.707);

    // Application du filtre
    filter.apply_filter(&input_samples, &mut output_samples);

    // Affichage des échantillons filtrés
    println!("Filtered samples: {:?}", output_samples);
}
