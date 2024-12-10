use crate::util::read_single_string;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Entry {
    id: Option<usize>,
    len: usize,
}

#[must_use]
pub fn d9() -> (usize, usize) {
    let input = read_single_string("input_9_sample");

    let mut entries: Vec<Entry> = Vec::new();

    let mut input_iter = input.trim().chars();
    let mut i = 0;

    while let Some(c) = input_iter.next() {
        entries.push(Entry {
            id: Some(i),
            len: c.to_digit(10).unwrap() as usize,
        });

        match input_iter.next() {
            Some(c) => {
                entries.push(Entry {
                    id: None,
                    len: c.to_digit(10).unwrap() as usize,
                });
            }
            None => break,
        }

        i += 1;
    }

    let mut compacted: Vec<Entry> = Vec::new();

    let mut f_iter = entries.iter();
    let mut b = &Entry { id: None, len: 0 };
    let mut f_left = 0;
    let mut b_used = 0;
    'l: while let Some(f) = f_iter.next() {
        if f == b {
            break;
        }

        if f.id.is_some() {
            compacted.push(*f);
            continue;
        }

        f_left = f.len;

        loop {
            if b_used >= b.len {
                b = match f_iter.next_back() {
                    Some(e) => e,
                    None => break 'l,
                };
            }

            println!("{f:?}, {b:?}, {f_left}, {b_used}");
            println!("{compacted:?}");

            if b.id.is_some() {
                match f_left.cmp(&(b.len - b_used)) {
                    std::cmp::Ordering::Less => {
                        compacted.push(Entry {
                            id: b.id,
                            len: f_left,
                        });
                        b_used += f_left;
                        break;
                    }
                    std::cmp::Ordering::Equal => {
                        compacted.push(Entry {
                            id: b.id,
                            len: f.len,
                        });
                        b_used = b.len;
                        break;
                    }
                    std::cmp::Ordering::Greater => {
                        compacted.push(Entry {
                            id: b.id,
                            len: f_left,
                        });
                        f_left -= b.len;
                        b_used = b.len;
                        continue;
                    }
                }
            } else {
                b_used = b.len
            }
        }
    }

    println!("{compacted:?}");

    let mut r1 = 0;
    let mut i = 0;

    for f in compacted {
        for _ in 0..f.len {
            r1 += i * f.id.unwrap();
            i += 1;
        }
    }

    (r1, 0)
}
