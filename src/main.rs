#[allow(dead_code)]
mod die;
#[allow(dead_code)]
mod probs;
#[allow(dead_code)]
mod puntata;

use crate::{die::Die, probs::get_probs_of, puntata::Puntata};
use std::io;

fn read_int() -> u8 {
    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    trimmed.parse().expect("Invalid integer!")
}

fn main() {
    println!("Enter players' other dices count:");

    let other_dice_cnt = read_int();

    let my_dice = (0..other_dice_cnt)
        .map(|_| Die::new(read_int()).expect("Invalid value for die"))
        .collect::<Vec<_>>();

    println!("My dice: {:?}", my_dice);

    let puntata = Puntata::new(4, 5).unwrap();

    let all_probs = get_probs_of(other_dice_cnt, &my_dice, false, puntata);

    let mut sorted_all_probs: Vec<_> = all_probs.into_iter().collect();

    sorted_all_probs.sort_by(|&(_, v1), &(_, v2)| v1.partial_cmp(&v2).unwrap().reverse());

    let t = sorted_all_probs
        .iter()
        .find(|&&(p1, _)| p1 == puntata)
        .unwrap();
    let prob_of_p = t.1;

    sorted_all_probs.retain(|&(px, _)| px != puntata);

    println!("Probability of {}: {1:.2}%", puntata, prob_of_p * 100.0);

    if prob_of_p <= 0.2 {
        println!("I would dubitate...");
    } else {
        println!(
            "I would play {}, with chance {1:.2}%",
            sorted_all_probs[0].0,
            sorted_all_probs[0].1 * 100.0
        );
    }

    println!();
    println!("List of all probs:");

    for prob in sorted_all_probs.iter().take(40).filter(|&(_, x)| *x > 0.0) {
        println!("{}, with chance {1:.2}%", prob.0, prob.1 * 100.0);
    }
}
