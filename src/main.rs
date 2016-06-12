extern crate probability;
extern crate random;
extern crate rand;

#[allow(dead_code)]
mod die;
#[allow(dead_code)]
mod probs;
#[allow(dead_code)]
mod puntata;

use die::Die;
use puntata::Puntata;
use probs::get_probs_of;

fn main() {
    let dices = 25;
    let my_dices = vec![Die::new(5)];
    let p = Puntata::new(1, 5);

    let all_probs = get_probs_of(dices, &my_dices, true, &p);

    let mut sorted_all_probs: Vec<_> = all_probs.into_iter().collect();

    sorted_all_probs.sort_by(|&(_, v1), &(_, v2)| v1.partial_cmp(&v2).unwrap().reverse());
    let sorted_all_probs: Vec<_> = sorted_all_probs.into_iter().take(10).collect();

    for &(p, f) in &sorted_all_probs {
        println!("{} -> {}", p, f);
    }
}