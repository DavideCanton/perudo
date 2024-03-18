use crate::die::Die;
use std::{collections::HashSet, fmt};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Puntata {
    value: Die,
    count: u8,
}

impl Puntata {
    pub fn new(count: u8, value: u8) -> Option<Self> {
        Die::new(value).map(|d| Puntata { value: d, count })
    }

    pub fn new_lama(count: u8) -> Self {
        Puntata {
            value: Die::new_lama(),
            count,
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value.get_value()
    }

    pub fn get_count(&self) -> u8 {
        self.count
    }

    pub fn is_lama(&self) -> bool {
        self.value.is_lama()
    }

    pub fn with_count(self, count: u8) -> Self {
        Puntata {
            value: self.value,
            count,
        }
    }
}

impl fmt::Display for Puntata {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.is_lama() {
            write!(fmt, "Puntata di {} Lama", self.count)
        } else {
            write!(fmt, "Puntata di {} {}", self.count, self.value.get_value())
        }
    }
}

pub fn least_gt_puntate(p: Puntata, is_palifico: bool) -> Vec<Puntata> {
    let mut puntate = vec![];

    if !p.is_lama() {
        for i in (p.value.get_value() + 1)..=6 {
            puntate.push(Puntata::new(p.count, i).unwrap());
        }

        for i in 2..=6 {
            puntate.push(Puntata::new(p.count + 1, i).unwrap());
        }

        puntate.push(Puntata::new_lama((p.count + 1) / 2));
    } else {
        puntate.push(Puntata::new_lama(p.count + 1));

        for i in 2..=6 {
            puntate.push(Puntata::new(p.count * 2 + 1, i).unwrap());
        }
    }

    if is_palifico {
        puntate.retain(|pv| pv.value == p.value)
    }

    puntate
}

pub fn all_gt_puntate(total_dices: u8, p: Puntata, is_palifico: bool) -> Vec<Puntata> {
    let mut v = (p.count..total_dices)
        .flat_map(|v| {
            let px = p.with_count(v);
            least_gt_puntate(px, is_palifico)
        })
        .collect::<HashSet<_>>();

    if !is_palifico || p.is_lama() {
        for i in p.count..=total_dices {
            v.insert(Puntata::new_lama(i));
        }
    }

    v.remove(&p);
    v.insert(p);

    v.into_iter().collect()
}
