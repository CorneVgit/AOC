use crate::util::read_single_string;

#[derive(Default, Copy, Clone)]
struct Entry {
    id: Option<usize>,
    len: usize,
}

fn get_input(input: String) -> Vec<Entry> {
    let mut filesystem: Vec<Entry> = Vec::new();

    let mut input_iter = input.trim().chars();
    let mut i = 0;

    while let Some(c) = input_iter.next() {
        filesystem.push(Entry {
            id: Some(i),
            len: c.to_digit(10).unwrap() as usize,
        });

        match input_iter.next() {
            Some(c) => {
                filesystem.push(Entry {
                    id: None,
                    len: c.to_digit(10).unwrap() as usize,
                });
            }
            None => break,
        }

        i += 1;
    }

    filesystem
}

#[must_use]
pub fn d9() -> (usize, usize) {
    let input = read_single_string("input_9");

    let filesystem = get_input(input);

    (
        calc_checksum(&compact_filesystem_fragmented(&filesystem)),
        calc_checksum(&compact_filesystem_whole(&filesystem)),
    )
}

fn compact_filesystem_fragmented(filesystem: &[Entry]) -> Vec<Entry> {
    let mut compacted = Vec::new();

    let mut src = &Entry::default();
    let mut dest_free_space_left;
    let mut src_file_parts_left = src.len;

    let mut filesystem_iter = filesystem.iter();
    while let Some(entry) = filesystem_iter.next() {
        if entry.id.is_some() {
            // Entry is a file
            compacted.push(*entry);

            continue;
        }

        // Entry is free space
        dest_free_space_left = entry.len;
        while dest_free_space_left > 0 {
            if src.id.is_none() || src_file_parts_left == 0 {
                // Find next file to move
                src = match filesystem_iter.next_back() {
                    Some(e) => e,
                    None => {
                        break;
                    }
                };
                src_file_parts_left = src.len;

                continue;
            }

            match dest_free_space_left.cmp(&src_file_parts_left) {
                std::cmp::Ordering::Less => {
                    compacted.push(Entry {
                        id: src.id,
                        len: dest_free_space_left,
                    });

                    // Moved part of the file to fill the whole empty space
                    src_file_parts_left -= dest_free_space_left;

                    break;
                }
                std::cmp::Ordering::Equal => {
                    compacted.push(Entry {
                        id: src.id,
                        len: dest_free_space_left,
                    });

                    // Moved the whole file to fill the whole empty space
                    src_file_parts_left = 0;

                    break;
                }
                std::cmp::Ordering::Greater => {
                    compacted.push(Entry {
                        id: src.id,
                        len: src_file_parts_left,
                    });

                    // Moved the whole file to fill part of the empty space
                    dest_free_space_left -= src_file_parts_left;
                    src_file_parts_left = 0;
                }
            }
        }
    }

    if let Some(e) = compacted.last_mut() {
        if src_file_parts_left > 0 && e.id == src.id {
            e.len += src_file_parts_left;
        }
    }

    compacted
}

fn compact_filesystem_whole(filesystem: &[Entry]) -> Vec<Entry> {
    let mut compacted = filesystem.to_vec();

    for entry in filesystem.iter().rev().filter(|e| e.id.is_some()) {
        if let Some(dest_i) = compacted
            .iter()
            .position(|e| e.id.is_none() && e.len >= entry.len)
        {
            match compacted.iter().position(|e| e.id == entry.id) {
                Some(src_i) if dest_i > src_i => continue,
                Some(src_i) => compacted[src_i].id = None,
                None => break,
            }

            if compacted[dest_i].len > entry.len {
                compacted.insert(
                    dest_i + 1,
                    Entry {
                        id: None,
                        len: compacted[dest_i].len - entry.len,
                    },
                );
            }

            compacted[dest_i] = *entry;
        }
    }

    compacted
}

fn calc_checksum(compacted: &[Entry]) -> usize {
    let mut r = 0;
    let mut i = 0;

    for f in compacted {
        match f.id {
            Some(id) => {
                for _ in 0..f.len {
                    r += i * id;
                    i += 1;
                }
            }
            None => {
                i += f.len;
            }
        };
    }

    r
}
