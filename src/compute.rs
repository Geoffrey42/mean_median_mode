use std::collections::HashMap;

pub fn get_mean(list: &Vec<i32>) -> Option<i32> {
    let mut sum: i32 = 0;

    if list.len() == 0 {
        return None;
    }

    for element in list {
        sum += element;
    }
    Some(sum / (list.len() as i32))
}

pub fn get_median(list: &mut Vec<i32>) -> Option<i32> {
    list.sort();

    let len = list.len();

    if len % 2 != 0 {
        return Some(list[(len / 2) + 1]);
    } else {
        let middle = len / 2;
        return get_mean(&list[middle - 1..middle + 1].to_vec());
    }
}

pub fn get_max_key(map: HashMap<&i32, i32>) -> Option<i32> {
    let mut result = None;
    let mut max = 0;

    for (key, value) in map {
        if max < value {
            max = value;
            result = Some(*key);
        }
    }
    result
}

pub fn get_mode(list: &Vec<i32>) -> Option<i32> {
    let mut map = HashMap::new();
    
    for element in list {
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }
    get_max_key(map)
}