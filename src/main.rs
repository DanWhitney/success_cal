use core::num;
use std::{collections::HashMap, process::Output};

use convert_base::Convert;
use rand::Rng;


enum Outcome {
    Fail,
    Success,
    Max,
}

fn V5Outcome(number: u32) -> Outcome {
    if number == 10 {
        return Outcome::Max;
    }
    if number >= 6 {
        return Outcome::Success;
    } else {
        return Outcome::Fail;
    }
}

fn V5SuccessesNotCrit(number: u32) -> u32 {
    if number == 0 {
        return 1;
    } else if number > 5{
        return 1;
    }
    else {
        return 0;
    }
}

fn count_successes(result: Vec<Outcome>) -> u32 {
    let mut count: u32 = 0;
    let mut max_count: u32 = 0;
    for outcome in &result {
        match outcome {
            Outcome::Fail => count += 0,
            Outcome::Success => count += 1,
            Outcome::Max => {
                count += 1;
                max_count += 1;
            }
        }
    }
    count + (max_count / 2) * 2
}
fn buteforce (){
for d in 8..=10 {
    let mut dist: HashMap<u32, u32> = HashMap::new();
    for j in 0..100000000 {
        let mut results: Vec<Outcome> = Vec::new();
        for i in 0..d {
            let num = rand::thread_rng().gen_range(1..=10);
            // println!("number: {num}");
            let outcome = V5Outcome(num);
            results.push(outcome);
        }

        //        println!("");
        let num_of_successes = count_successes(results);

        let current_count = dist.entry(num_of_successes).or_insert(0);
        *current_count += 1;
    }
    let max = *dist.keys().into_iter().max().unwrap();
    print!("max: {max}");
    let mut output = String::new();
    for c in 0..=max {
        //println!("{c}");
        match dist.get(&c) {
            Some(x) => {
                output = format!("{output},{x}");
            },
            None => {
                output = format!("{output},");
            },
         }
    }
    println!("{output}");
}
}

fn to_digits_using_to_string(number: u64) -> Vec<u32> {
    let string = number.to_string();
    let digits: Vec<u32> = string.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
    return digits;
}

fn to_digits_and_pad(number: u32, base: u32, desired_length: usize) -> Vec<u32> {
    let digits = to_digits(number, base);

    let mut padded_digits = Vec::with_capacity(desired_length);
    for i in 0..desired_length {
        if i < digits.len() {
            padded_digits.push(digits[i]);
        } else {
            padded_digits.push(0);
        }
    }

    return padded_digits;
}

fn to_digits(num: u32, base:u32) -> Vec<u32> {
    /*
     * Zero is a special case because
     * it is the terminating value of the unfold below,
     * but given a 0 as input, [0] is expected as output.
     * w/out this check, the output is an empty vec.
     */
    if num == 0 {
        return vec![0];
    }

    let mut x = num;
    let mut result = std::iter::from_fn(move || {
        if x == 0 {
            None
        } else {
            let current = x % base;
            x /= base;
            Some(current)
        }
    })
    .collect::<Vec<u32>>();

    //result.reverse();
    result
}

fn reached_max (vec:& Vec<u32>, base:u32) -> bool{
    let size = vec.len();
    let count = vec.iter().filter(|&n| *n == base-1).count();
    size == count
}

fn convert_to_die_num (vec:&Vec<u32>) -> Vec<u32>{
    vec.iter().map(|&n| n + 1).collect()
}

fn apply_base_success_filter_with_target (vec:&Vec<u32>, target:u32) -> Vec<u32>{
    vec.iter().map(|&n| {
        if n >= target {
            1
        }
        else {
            0
        }
    }
    ).collect()
}



fn main() {

   for num_of_d in 1..=10 {
    //let num_of_d = 2;
    let sides:u32= 10;
    let mut counter = 0;
    let mut dist: HashMap<u32,u32> = HashMap::new();
    loop {
        
        
        let mut d = to_digits_and_pad(counter, sides, num_of_d); 

        d.reverse();
        
        let dr = convert_to_die_num(&d);

        let sr = apply_base_success_filter_with_target(&dr, 6);
        let pcsr = apply_base_success_filter_with_target(&dr, 10);

        let sr_count:u32 = sr.iter().sum();
        let pcsr_count:u32 = pcsr.iter().sum();
        let pcsr_count=  pcsr_count/2 * 2;
        let num_s =  sr_count + pcsr_count;

       // println!("{:?} | {:?} | {:?} | {:?} ", dr, sr, pcsr, num_s);

        let current_count = dist.entry(num_s).or_insert(0);
        *current_count += 1;


        if reached_max(&d,sides) {
            break;
        }
        counter+=1;
    }
  //  println!("count {:?}",counter);
  //  println!("dist {:?}",dist);

    let max = *dist.keys().into_iter().max().unwrap();
   // println!("max: {max}");
    let mut output = format!("{num_of_d}");
    for c in 0..=max {
        //println!("{c}");
        match dist.get(&c) {
            Some(x) => {
                let perc: f64 = f64::from(*x)/f64::from(counter+1);
                output = format!("{output},{perc}");
            },
            None => {
                output = format!("{output},");
            },
         }
    }
    println!("{output}");
   }
}
