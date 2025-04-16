use rand::{rngs::ThreadRng, Rng};

pub mod sensor;

// generate noise
fn noise(value: f64, rng: &mut ThreadRng) -> f64 {
    let noise = rng.gen_range(0.95..=1.05);

    value * noise
}

// generate anomalies
pub fn generate_anomalies(value: f64, rng: &mut ThreadRng) -> f64 {
    let value_with_noise = noise(value, rng);
    let final_value = if rng.gen_bool(0.05) {
        let anomaly_factor = if rng.gen_bool(0.5) {
            rng.gen_range(1.15..=1.2)
        } else {
            rng.gen_range(0.8..=0.85)
        };
        value_with_noise * anomaly_factor
    } else {
        value_with_noise
    };

    final_value
}

pub fn generate_rand_value_u8(rng: &mut ThreadRng) -> u8 {
    rng.gen_range(1..=10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_rand_value_u8() {
        let mut rng = rand::thread_rng();

        let actual = generate_rand_value_u8(&mut rng);
        assert!(actual >= 1 && 10 >= actual)
    }
}
