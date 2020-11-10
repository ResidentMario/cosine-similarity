// use cosine_similarity::get_tropes;
use cosine_similarity::similarity_score;

fn main() {
    println!("{}", similarity_score("BurnNotice", "Series", "Naruto", "Series"));
}
