use std::collections::HashMap;

fn get_max_key(map: HashMap<&i32, i32>) -> Option<i32> {
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