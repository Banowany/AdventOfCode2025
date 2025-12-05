use crate::IdRange;

pub fn calc(ranges: &Vec<IdRange>, ids: &Vec<usize>) -> usize {
    let mut res: usize = 0;
    for &id in ids {
        for range in ranges {
            if range.is_contains(id) {
                res += 1;
                break;
            }
        }
    }
    return res;
}
