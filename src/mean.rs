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
