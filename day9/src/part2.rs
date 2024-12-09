use std::{
    collections::{BTreeMap, BTreeSet},
    io::BufRead,
    ops::Bound::{Excluded, Included, Unbounded},
};

pub fn run<B: BufRead>(mut buf: B) -> u64 {
    let mut line = String::new();
    let mut free_map = BTreeMap::new();
    let mut file_map = BTreeMap::new();
    let mut id = 0;
    let mut address = 0;
    buf.read_line(&mut line).unwrap();

    for (i, c) in line.trim_end().chars().enumerate() {
        let contiguity = c.to_digit(10).unwrap() as u64;
        if i % 2 == 0 {
            if contiguity > 0 {
                file_map.insert(id, (address, contiguity));
                id += 1;
                address += contiguity;
            }
        } else {
            let frees = free_map
                .entry(contiguity)
                .or_insert_with(|| BTreeSet::new());
            frees.insert(address);
            address += contiguity;
        }
    }

    for (_, (file_idx, file_size)) in file_map.iter_mut().rev() {
        let elements = free_map.range_mut((Included(*file_size), Unbounded));
        let res: Option<(u64, u64)> = elements.fold(None, |accum, (size, indices)| {
            if let Some((_, accum_index)) = accum {
                let lowest_idx = *indices.first().unwrap();
                if lowest_idx < accum_index {
                    Some((*size, lowest_idx))
                } else {
                    accum
                }
            } else {
                Some((*size, *indices.first().unwrap()))
            }
        });

        if let Some((free_size, free_idx)) = res {
            if free_idx > *file_idx {
                continue;
            }
            *file_idx = free_idx;
            let indices = free_map.get_mut(&free_size).unwrap();
            indices.remove(&free_idx);
            if indices.len() == 0 {
                free_map.remove(&free_size);
            }
            let indices = free_map
                .entry(free_size - *file_size)
                .or_insert_with(|| BTreeSet::new());
            indices.insert(free_idx + *file_size);
        }
    }

    let mut checksum = 0;
    for (file_id, (start_idx, length)) in file_map {
        for i in start_idx..(start_idx + length) {
            checksum += file_id * i;
        }
    }

    checksum
}
