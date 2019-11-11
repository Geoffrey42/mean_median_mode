use std::collections::HashMap;

fn get_mean(list: &Vec<i32>) -> Option<i32> {
    let mut sum: i32 = 0;

    if list.len() == 0 {
        return None;
    }

    for element in list {
        sum += element;
    }
    Some(sum / (list.len() as i32))
}

fn get_median(list: &mut Vec<i32>) -> Option<i32> {
    list.sort();

    let len = list.len();

    if len % 2 != 0 {
        return Some(list[(len / 2) + 1]);
    } else {
        let middle = len / 2;
        return get_mean(&list[middle - 1..middle + 1].to_vec());
    }
}

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

fn get_mode(list: &Vec<i32>) -> Option<i32> {
    let mut map = HashMap::new();
    
    for element in list {
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }
    get_max_key(map)
}

fn main() {
    let list1 = vec![6, 11, 7];
    println!("list1: {:?}", list1);

    let mean = get_mean(&list1);
    match mean {
        Some(result) => println!("mean: {}", result),
        None => println!("There is no mean for this sequence"),
    };

    let mut list2 = vec![3, 13, 7, 5, 21, 23, 39, 23, 40, 23, 14, 12, 56, 23, 29];
    println!("list2: {:?}", list2);

    let median = get_median(&mut list2);
    match median {
        Some(result) => println!("median: {}", result),
        None => println!("There is no median for this sequence"),
    }

    let mut list3 = vec![3, 13, 7, 5, 21, 23, 23, 40, 23, 14, 12, 56, 23, 29];
    println!("list3: {:?}", list3);
    let median2 = get_median(&mut list3);
    match median2 {
        Some(result) => println!("median2: {}", result),
        None => println!("There is no median2 for this sequence"),
    }

    let list4 = vec![19, 8, 29, 35, 19, 28, 15];
    println!("list4: {:?}", list4);
    let mode1 = get_mode(&list4);
    match mode1 {
        Some(result) => println!("mode1: {}", result),
        None => println!("There is no mode1 for this sequence"),
    }
}
