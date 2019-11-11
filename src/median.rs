use crate::mean;

pub fn get_median(list: &mut Vec<i32>) -> Option<i32> {
    list.sort();

    let len = list.len();

    if len % 2 != 0 {
        return Some(list[(len / 2) + 1]);
    } else {
        let middle = len / 2;
        return mean::get_mean(&list[middle - 1..middle + 1].to_vec());
    }
}
