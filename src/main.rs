pub use compute::{mean, mode, median};

fn main() {
    let list1 = vec![6, 11, 7];
    println!("list1: {:?}", list1);

    let mean = mean::get_mean(&list1);
    match mean {
        Some(result) => println!("mean: {}", result),
        None => println!("There is no mean for this sequence"),
    };

    let mut list2 = vec![3, 13, 7, 5, 21, 23, 39, 23, 40, 23, 14, 12, 56, 23, 29];
    println!("list2: {:?}", list2);

    let median = median::get_median(&mut list2);
    match median {
        Some(result) => println!("median: {}", result),
        None => println!("There is no median for this sequence"),
    }

    let mut list3 = vec![3, 13, 7, 5, 21, 23, 23, 40, 23, 14, 12, 56, 23, 29];
    println!("list3: {:?}", list3);
    let median2 = median::get_median(&mut list3);
    match median2 {
        Some(result) => println!("median2: {}", result),
        None => println!("There is no median2 for this sequence"),
    }

    let list4 = vec![19, 8, 29, 35, 19, 28, 15];
    println!("list4: {:?}", list4);
    let mode1 = mode::get_mode(&list4);
    match mode1 {
        Some(result) => println!("mode1: {}", result),
        None => println!("There is no mode1 for this sequence"),
    }
}
