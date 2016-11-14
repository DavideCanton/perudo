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
use std::io;

fn read_int() -> i32 {
    let mut input_text = String::new();

    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    trimmed.parse::<i32>().expect("Invalid integer!")
}

fn main() {
    println!("Enter players' other dices count:");

    let dice = read_int();
    let my_dice = vec![Die::new(5)];
    println!("My dice: {:?}", my_dice);

    let p = Puntata::new(4, 5);

    let all_probs = get_probs_of(dice, &my_dice, false, &p);

    let mut sorted_all_probs: Vec<_> = all_probs.into_iter().collect();

    sorted_all_probs.sort_by(|&(_, v1), &(_, v2)| v1.partial_cmp(&v2).unwrap().reverse());

    let prob_of_p = {
        let ref t = sorted_all_probs.iter().find(|&&(p1, _)| p1 == p).unwrap();
        t.1
    };

    sorted_all_probs.retain(|&(px, _)| px != p);

    println!("Probability of {}: {1:.2}%", p, prob_of_p * 100.0);

    if prob_of_p <= 0.2 {
        println!("I would dubitate...");
    } else {
        println!("I would play {}, with chance {1:.2}%",
                 sorted_all_probs[0].0,
                 sorted_all_probs[0].1 * 100.0);
    }
}