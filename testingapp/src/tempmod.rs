use rand::{thread_rng, Rng};

pub fn print_random_number(){
    println!("{}", thread_rng().gen_range(0..100))
}