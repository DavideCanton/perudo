use std::cmp::max;
use die::Die;
use std::collections::HashMap;
use probability::distribution::{Categorical, Binomial, Sample, Discrete};
use random::{default, Default};
use rand::{thread_rng, Rng};
use puntata::{Puntata, all_gt_puntate};

pub struct DieGenerator {
    dist: Categorical,
    gen: Default,
}

impl DieGenerator {
    pub fn new() -> Self {
        let v = vec![1.0 / 6.0; 6];
        let mut rng = thread_rng();

        DieGenerator {
            dist: Categorical::new(v.as_slice()),
            gen: default().seed([rng.gen(), rng.gen()]),
        }
    }

    pub fn random_die(&mut self) -> Die {
        let n = self.dist.sample(&mut self.gen);
        Die::new(n as i32 + 1)
    }
}

fn my_dices_matching(my_dices: &[Die], value: i32, is_palifico: bool) -> i32 {
    my_dices.iter()
        .filter(|d| d.matches_value(value, is_palifico))
        .count() as i32
}

pub fn prob_of(other_dices: i32, my_dices: &[Die], is_palifico: bool, p: &Puntata) -> f64 {
    let valid_my_dices = my_dices_matching(my_dices, p.get_value(), is_palifico);

    let prob = if p.is_lama() || is_palifico {
        // considera solo il valore della puntata nel calcolo
        1.0 / 6.0
    } else {
        1.0 / 3.0
    };

    let dist = Binomial::new(other_dices as usize, prob);
    let start = max(p.get_count() - valid_my_dices, 0);

    if start == 0 {
        1.0 // avoid floating point errors when 1 is sure
    } else {
        (start..other_dices + 1)
            .map(|v| dist.mass(v as usize))
            .sum()
    }
}

pub fn get_probs_of(other_dices: i32,
                    my_dices: &[Die],
                    is_palifico: bool,
                    least_puntata: &Puntata)
                    -> HashMap<Puntata, f64> {
    let total_dices = other_dices + my_dices.len() as i32;
    let all_puntate = all_gt_puntate(total_dices, least_puntata, is_palifico);

    all_puntate.iter()
        .map(|&p| {
            let prob = prob_of(other_dices, my_dices, is_palifico, &p);
            (p, prob)
        })
        .collect::<HashMap<_, _>>()
}


#[cfg(test)]
mod tests {
    use die::Die;
    use puntata::Puntata;
    use probs::{get_probs_of, my_dices_matching};

    #[test]
    fn test_get_probs_of() {
        let other = 5;
        let my_dices: Vec<Die> = vec![2, 2, 2, 4, 5].into_iter().map(Die::new).collect();
        let puntata = Puntata::new(2, 2);

        let p_map = get_probs_of(other, &my_dices, false, &puntata);

        let v = *p_map.get(&puntata.with_count(3)).unwrap();

        assert!(v >= 0.99);
    }

    #[test]
    fn test_my_dices_matching() {
        let mut my_dices: Vec<Die> = vec![2, 2, 4, 5].into_iter().map(Die::new).collect();

        my_dices.push(Die::new_lama());

        let c = my_dices_matching(&my_dices, 2, false);

        assert_eq!(c, 3);
    }

    #[test]
    fn test_my_dices_matching_palifico() {
        let mut my_dices: Vec<Die> = vec![2, 2, 4, 5].into_iter().map(Die::new).collect();

        my_dices.push(Die::new_lama());

        let c = my_dices_matching(&my_dices, 2, true);

        assert_eq!(c, 2);
    }
}