use std::collections::HashMap;
use std::fs;
use std::env;

fn main() {
    let contents = fs::read_to_string("resources/input.txt").expect("input.txt not found");
    let lines = contents.lines();
    
    let mut first = Vec::new();
    let mut second = Vec::new();
    let mut counter = HashMap::new();
    
    for line in lines {
        let mut s = line.split("   ");
        let a: u32 = s.next().expect("Should be a number").parse().expect("Should be a valid number");
        let b: u32 = s.next().expect("Should be a number").parse().expect("Should be a valid number");
        
        first.push(a);
        second.push(b);
        
        counter.entry(b).and_modify(|val| *val += 1).or_insert(1);
    }
    
    first.sort_unstable();
    second.sort_unstable();
    
    let mut sum = 0;
    let mut similarity = 0;
    for i in 0..first.len(){
        let a = first[i];
        let b = second[i];
        
        let d = a.abs_diff(b);
        sum += d;
        
        similarity += a * counter.get(&a).unwrap_or(&0);
    }
    println!("{}", sum);
    println!("{}", similarity);
}
