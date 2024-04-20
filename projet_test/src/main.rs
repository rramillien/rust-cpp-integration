use oscillator_module::Oscillator;

fn main() {
    let mut buffer = [0.0; 255];
    let mut obj = Oscillator::new(440.0, 0.0, 44100.0, &mut buffer);
    for i in 0..9 {
        let buffer = obj.process();
        for j in 0..buffer.len() {
            println!("Loop {}: subloop: {}: sample {}", i, j, buffer[j]);
        }
    }
}