use rand::{distributions::Alphanumeric, Rng};

pub fn generate_secret() -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(40)
        .map(char::from)
        .collect();
}
