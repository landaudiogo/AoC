use std::io::BufRead;

#[derive(Debug, Clone)]
enum Content {
    Free(u64),
    Block(u64, u64),
}

impl Content {
    fn as_block(&mut self) -> Option<(&mut u64, &mut u64)> {
        match self {
            Self::Block(id, len) => Some((id, len)),
            Self::Free(_) => return None,
        }
    }

    fn as_free(&mut self) -> Option<&mut u64> {
        match self {
            Self::Block(_, _) => None,
            Self::Free(length) => return Some(length),
        }
    }
}

fn find_last_block(disk: &Vec<Content>, start: usize, end: usize) -> Option<usize> {
    for i in (start..end).rev() {
        if let Content::Block(_, _) = disk[i] {
            return Some(i);
        }
    }
    return None;
}

fn defrag_disk(mut disk: Vec<Content>) -> Vec<Content> {
    let mut defrag_disk = Vec::new();
    let mut end = disk.len();
    for content_idx in 0..disk.len() {
        let content = &disk[content_idx];
        match content {
            Content::Block(_, _) => {
                defrag_disk.push(content.clone());
            }
            Content::Free(length) => {
                let mut remaining = *length;
                while remaining > 0 {
                    let block_idx =
                        if let Some(block_idx) = find_last_block(&disk, content_idx, end) {
                            block_idx
                        } else {
                            return defrag_disk;
                        };
                    let block = disk[block_idx].as_block().unwrap();
                    let quantity = u64::min(*block.1, remaining);
                    remaining = remaining - quantity;
                    *block.1 = *block.1 - quantity;
                    if *block.1 == 0 {
                        end = block_idx;
                    }
                    defrag_disk.push(Content::Block(*block.0, quantity));
                }
            }
        }
    }

    defrag_disk
}

pub fn run<B: BufRead>(mut buf: B) -> u64 {
    let mut line = String::new();
    let mut disk = Vec::new();
    let mut id = 0;
    buf.read_line(&mut line).unwrap();

    for (i, c) in line.trim_end().chars().enumerate() {
        let contiguity = c.to_digit(10).unwrap() as u64;
        if i % 2 == 0 {
            disk.push(Content::Block(id, contiguity));
            if contiguity > 0 {
                id += 1;
            }
        } else {
            disk.push(Content::Free(contiguity));
        }
    }

    let disk = defrag_disk(disk);
    let mut position = 0;
    let mut checksum = 0;
    for mut content in disk {
        let block = content.as_block().unwrap();
        for _ in 0..*block.1 {
            checksum += *block.0 * position;
            position += 1;
        }
    }
    checksum
}
