use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Before hang");
    println!("{}", rng.sample(rand::distributions::Uniform::new(0.99999999997819644, 1.)));
    println!("After hang");
}
