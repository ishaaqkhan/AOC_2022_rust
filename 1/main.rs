fn main() {
    let mut sum: Vec<i32> = Vec::new();
    for line in include_str!("input.txt").lines() {
        if line != "" {
            if let Some(last) = sum.last().cloned() {
                sum.push(last + line.parse::<i32>().unwrap());
            }
        } else {
            sum.push(0);
        }
    }
    let mut sorted_sum: Vec<i32> = sum.clone();
    sorted_sum.sort();
    sorted_sum.reverse();
    let _max_value = *sum.iter().max().unwrap();
    let _max_three_sum = sorted_sum[0] + sorted_sum[1] + sorted_sum[2];
    println!("{}", _max_three_sum);
}