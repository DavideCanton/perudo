use std::fmt;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Puntata {
    value: i32,
    count: i32,
}

impl Puntata {
    pub fn new(count: i32, value: i32) -> Self {
        Puntata {
            value: value,
            count: count,
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_count(&self) -> i32 {
        self.count
    }

    pub fn with_count(&self, count: i32) -> Self {
        Puntata {
            value: self.value,
            count: count,
        }
    }
}

impl fmt::Display for Puntata {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            1 => write!(fmt, "Puntata di {} Lama", self.count),
            n => write!(fmt, "Puntata di {} {}", self.count, n),
        }

    }
}

pub fn least_gt_puntate(p: &Puntata, is_palifico: bool) -> Vec<Puntata> {
    let mut v = vec![];

    if p.value != 1 {
        for i in (p.value + 1)..7 {
            v.push(Puntata::new(p.count, i));
        }

        for i in 2..7 {
            v.push(Puntata::new(p.count + 1, i));
        }

        v.push(Puntata::new((p.count + 1) / 2, 1));
    } else {
        v.push(Puntata::new(p.count + 1, 1));

        for i in 2..7 {
            v.push(Puntata::new(p.count * 2 + 1, i));
        }
    }

    if is_palifico {
        v.retain(|pv| pv.value == p.value)
    }

    v
}

pub fn all_gt_puntate(total_dices: i32, p: &Puntata, is_palifico: bool) -> Vec<Puntata> {
    let mut v = (p.count..total_dices)
                    .flat_map(|v| {
                        let px = p.with_count(v);
                        least_gt_puntate(&px, is_palifico)
                    })
                    .collect::<HashSet<_>>();

    if !is_palifico || p.get_value() == 1 {
        for i in p.count..total_dices + 1 {
            v.insert(Puntata::new(i, 1));
        }
    }

    v.remove(p);

    v.into_iter().collect()
}
