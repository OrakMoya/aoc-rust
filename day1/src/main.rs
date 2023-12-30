use std::io;

fn main() {
    let mut line = String::new();
    let mut sum = 0;
    let mut vec: Vec<i32> = Vec::new();
    while io::stdin().read_line(&mut line).unwrap() != 0 {
        if line.trim().is_empty() {
            vec.push(sum);
            sum = 0;
            continue;
        }

        sum += line.trim().parse::<i32>().unwrap();
        line = String::new();
    }
    vec.push(sum);
    vec.sort();

    sum = 0;
    for i in 0..3 {
        println!("i: {}", i);
        sum += vec.pop().unwrap();
    }
    println!("{}", sum);
}
