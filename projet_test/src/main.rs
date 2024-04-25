use Oscilator_Mod_saw::OscillateurSaw; // Remplacez par le nom correct du module où OscillateurSaw est défini
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io;

fn main() {
    let osc = Arc::new(Mutex::new(OscillateurSaw::new(440.0, 0.0, 44100.0, 1024)));
    let echantillons = Arc::new(Mutex::new(Vec::new()));
    let running = Arc::new(Mutex::new(true));

    let osc_clone = Arc::clone(&osc);
    let samples_clone = Arc::clone(&echantillons);
    let running_clone = Arc::clone(&running);

    // Thread pour générer les échantillons
    let generator = thread::spawn(move || {
        while *running_clone.lock().unwrap() {
            let mut osc = osc_clone.lock().unwrap();
            let sample = osc.generate_and_get_next_sample();
            samples_clone.lock().unwrap().push(sample);
            thread::sleep(Duration::from_millis(10));
        }
    });

    println!("Press Enter to stop playback...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    *running.lock().unwrap() = false;
    generator.join().unwrap();

    // Afficher les échantillons générés
    let samples = echantillons.lock().unwrap();
    println!("[{}]", samples.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(", "));
}
