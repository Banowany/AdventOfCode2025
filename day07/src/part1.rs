use std::collections::{HashMap, HashSet};

fn step(splitters: &HashMap<usize, Vec<usize>>, curr: &HashSet<(usize, usize)>, visited: &mut HashSet<(usize, usize)>) -> (HashSet<(usize, usize)>, usize) {
    let mut res: usize = 0;
    let mut next_curr: HashSet<(usize, usize)> = HashSet::new();
    for &(row, col) in curr {
        let maybe_splitters_for_col = splitters.get(&col);
        match maybe_splitters_for_col {
            Some(splitters_for_col) => {
                let maybe_splitter: Option<usize> = splitters_for_col
                    .iter()
                    .filter(|&&x| x > row)
                    .next()
                    .map(|&t| t);

                match maybe_splitter {
                    Some(splitter) => {
                        next_curr.insert((splitter, col-1));
                        next_curr.insert((splitter, col+1));
                        let splitter_point = (splitter, col);
                        if (!visited.contains(&splitter_point)) {
                            res+=1;
                            visited.insert(splitter_point);
                        }
                        // println!("({}, {})", splitter, col);
                    }
                    None => {
                        next_curr.insert((row, col));
                    }
                }
            }
            None => {
                next_curr.insert((row, col));
            }
        }
    }
    // println!();
    return (next_curr, res);
}
pub fn calc(splitters: &HashMap<usize, Vec<usize>>, start: (usize, usize)) -> usize {
    let mut res: usize = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut curr: HashSet<(usize, usize)> = [start].iter().cloned().collect();
    let output = step(splitters, &curr, &mut visited);
    let mut next_curr = output.0;
    res += output.1;

    while curr != next_curr {
        curr = next_curr;
        // println!("{:?}", curr);
        let output = step(splitters, &curr, &mut visited);
        next_curr = output.0;
        res += output.1;
    }

    return res;

    // znajdz splitery dla obecnych promieni czyli:
    // 1. wyciagnij splitery dla kolumny promienia
    // 2. wyfiltruj wzgledem wiersza
    // 3. dla tego o najmniejszym wierszu po filtracji daj promienie z lewej i prawej
}
