use std::fs::File; 
use std::io::{BufRead, BufReader};
use std::collections::HashMap; 

fn main() {
    let filename =  String::from("input.txt");
    let lines : Vec<String> = get_lines_from_file(filename);
    let accum = get_sum(&lines);
    let repeated_freq = get_repeated_frequency(&lines);

    println!("sum: {}\nrepeated frequency: {}", accum, repeated_freq);
}

/* part 1 */
fn get_sum(lines : &Vec<String>) -> i32 {
    let mut accum : i32 = 0;
    for line in lines {
        accum += get_int(&line);
    }
   accum
}

/* part 2 */
fn get_repeated_frequency(lines : &Vec<String>) -> i32 {
    let mut curr_freq = 0;
    let mut freqs : HashMap<i32, i32> = HashMap::new();

    loop {
        for line in lines {
            let new_freq = curr_freq + get_int(&line);
            let count_for_freq = freqs.entry(new_freq).or_insert(0);
            *count_for_freq += 1;
            if *count_for_freq > 1 {
                return new_freq;
            }
            curr_freq = new_freq;
        }
    }
}

fn get_int(line : &String) -> i32 {
    let operator = line.as_bytes()[0];
    let mut val = 0;
    if operator == b'+' {
        val += &line[1..].parse::<i32>().unwrap();
    } else if operator == b'-' {
        val -= &line[1..].parse::<i32>().unwrap();
    } else {
        panic!("OH NOES");
    }
    val
}

fn get_lines_from_file(filename: String) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let mut ret_lines : Vec<String> = vec![];
    for line in BufReader::new(file).lines() {
        ret_lines.push(line.unwrap());
    }
    ret_lines
}
