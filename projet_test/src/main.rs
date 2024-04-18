
use oscillateur_lib::Oscillateur;
fn main() {
    let mut osc = Oscillateur::new(440.0, 0.0, 44100.0, 1024); // Crée un oscillateur avec un buffer de taille 1024
    let mut x = 0; // Compteur pour simuler l'axe x

    // Boucle infinie pour générer et afficher les échantillons
    loop {
        let sample = osc.generate_and_get_next_sample();
        println!("Sample {}: {}", x, sample);

        x += 1;  // Incrémentation de l'index après chaque échantillon

        // Pause pour simuler un taux d'échantillonnage en temps réel et éviter de saturer la sortie
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
