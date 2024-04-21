use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::Sample;
use crossbeam::channel::{bounded, Receiver, Sender};
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use oscillateur_lib::Oscillateur; // Assurez-vous que le chemin est correct

fn main() -> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or(anyhow::Error::msg("No output device found"))?;
    let config = device.default_output_config()?.config();

    let sample_rate = config.sample_rate.0 as f64;
    let frequency = read_frequency()?;
    let osc = Arc::new(Mutex::new(Oscillateur::new(frequency, 0.0, sample_rate, 1024)));

    let (stop_sender, stop_receiver) = bounded::<()>(1);
    let (echantillons_sender, echantillons_receiver): (Sender<f64>, Receiver<f64>) = bounded(1024);

    let osc_for_audio = Arc::clone(&osc);
    let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut osc = osc_for_audio.lock().unwrap();
            for sample in data.iter_mut() {
                let gen_sample = osc.generate_and_get_next_sample();
                *sample = gen_sample.to_sample::<f32>();
            }
        },
        err_fn,
        None,
    )?;

    stream.play()?;

    let collect_samples_thread = thread::spawn(move || {
        let mut osc = osc.lock().unwrap();
        for _ in 0..1000 {
            let sample = osc.generate_and_get_next_sample();
            echantillons_sender.send(sample).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
        drop(echantillons_sender); // Close the sender when done
    });

    println!("Press Enter to stop playback...");
    let _ = read_stop_signal();
    stop_sender.send(()).unwrap();

    drop(stream); // Explicitly stop the stream

    let echantillons: Vec<f64> = echantillons_receiver.iter().collect();
    println!("[{}]", echantillons.iter().map(|&x| format!("{{\"x\": {}, \"y\": {}}}", x, x)).collect::<Vec<_>>().join(", "));

    collect_samples_thread.join().unwrap();

    Ok(())
}

fn read_stop_signal() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

fn read_frequency() -> Result<f64, io::Error> {
    println!("Enter the frequency (Hz):");
    let mut freq_input = String::new();
    io::stdin().read_line(&mut freq_input)?;
    let freq_hz: f64 = freq_input.trim().parse().unwrap_or(440.0); // Default to 440 Hz if parse fails
    Ok(freq_hz)
}
