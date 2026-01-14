use rand::prelude::*;

pub fn generate_id() -> u32 {
    let mut rng = rand::rng();
    rng.random_range(1000..9999)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let id = generate_id();
        assert!(id >= 1000 && id < 9999);
    }
}
