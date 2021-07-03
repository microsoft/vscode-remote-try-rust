use rand::Rng;

pub fn generate_secret_num() -> i32 {
    let secret_num = rand::thread_rng().gen_range(1..101);
    secret_num
}
