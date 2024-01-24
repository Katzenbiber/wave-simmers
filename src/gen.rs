pub fn get_sin() -> [u8; 10000] {
    let mut data = [0; 100 * 100];
    
    for (n, pixel) in data.iter_mut().enumerate() {
        let row = n / 100;
        let phase = row as f64 / 100.0 * 2.0 * std::f64::consts::PI;
        let value = ((phase.sin() + 1.0) * 127.0) as u8;
        *pixel = value;
    }

    return data;
}