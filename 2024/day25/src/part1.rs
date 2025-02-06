use std::{
    collections::{BTreeMap, HashSet},
    io::{BufRead, BufReader},
    ops::Bound,
};

#[derive(Debug, Hash, Eq, PartialEq)]
enum Factory {
    Key([u8; 5]),
    Lock([u8; 5]),
}

pub struct BufReadWrapper<B>(pub B);

impl<B: BufRead> TryFrom<BufReadWrapper<B>> for Factory {
    type Error = ();

    fn try_from(mut value: BufReadWrapper<B>) -> Result<Self, Self::Error> {
        // Your logic for constructing Factory from B goes here
        let mut line = String::new();
        let mut i = 0;
        let mut inner = [None; 5];
        let mut is_lock = false;
        while let Ok(len) = value.0.read_line(&mut line) {
            if len == 1 || len == 0 {
                if i == 0 {
                    return Err(());
                }
                break;
            }

            if (i == 0) && (line.trim() == "#####") {
                is_lock = true;
                i += 1;
                line.truncate(0);
                continue;
            }

            let comparator = if is_lock { '.' } else { '#' };
            line.trim().chars().enumerate().for_each(|(j, c)| {
                if inner[j].is_none() && c == comparator {
                    inner[j] = if is_lock {
                        Some(i - 1)
                    } else {
                        Some(5 - (i - 1))
                    };
                }
            });
            i += 1;
            line.truncate(0);
        }
        let mut finner: [u8; 5] = [0; 5];
        inner.into_iter().enumerate().for_each(|(i, v)| {
            finner[i] = v.unwrap();
        });
        match is_lock {
            true => Ok(Self::Lock(finner)),
            false => Ok(Self::Key(finner)),
        }
    }
}

pub fn run(mut buf: impl BufRead) {
    let mut pin_heights: [BTreeMap<u8, Vec<[u8; 5]>>; 5] = [
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    ];
    let mut locks = Vec::new();
    while let Ok(factory) = Factory::try_from(BufReadWrapper(&mut buf)) {
        match factory {
            Factory::Lock(inner) => {
                locks.push(inner);
            }
            Factory::Key(inner) => {
                for (i, c) in inner.iter().enumerate() {
                    pin_heights[i]
                        .entry(*c)
                        .or_insert_with(|| Vec::new())
                        .push(inner.clone());
                }
            }
        }
    }

    let mut total = 0;
    for lock in locks {
        let mut res: Option<HashSet<[u8; 5]>> = None;
        for (i, c) in lock.iter().enumerate() {
            let subset = pin_heights[i].range((Bound::Included(0), Bound::Included(5 - c)));
            let subset: HashSet<[u8; 5]> = subset
                .into_iter()
                .map(|(c, v)| v.clone())
                .flatten()
                .collect();
            if let Some(curr) = res {
                res = Some(HashSet::from_iter(curr.intersection(&subset).cloned()));
            } else {
                res = Some(subset)
            }
        }
        total += res.unwrap().len();
    }
    println!("{}", total);
}
