// use cosine_similarity::get_tropes;
use cosine_similarity::similarity_score;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("ERROR: This program expects 2 arguments, but {} were given.", args.len() - 1);
        return;
    }
    // let (m1, m2) = (&args[1], &args[2]);
    let m1_tup: Vec<&str> = args[1].split("/").collect::<Vec<&str>>();
    if m1_tup.len() != 2 {
        println!("ERROR: Input {} was not valid, must be in the format TYPE/NAME.", args[1]);
        return;
    }
    let m1_name = m1_tup[1];
    let m1_type = m1_tup[0];

    let m2_tup: Vec<&str> = args[2].split("/").collect::<Vec<&str>>();
    if m2_tup.len() != 2 {
        println!("ERROR: Input {} was not valid, must be in the format TYPE/NAME.", args[2]);
        return;
    }
    let m2_name = m2_tup[1];
    let m2_type = m2_tup[0];

    println!("{}", similarity_score(m1_name, m1_type, m2_name, m2_type));
}
