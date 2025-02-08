//Can be optimized to break the loop once it

fn main() {
    let contents = std::fs::read_to_string("resources/input.txt").expect("input.txt not found");
    let lines = contents.lines();
    
    let mut linenum = 1;
    let mut sum = 0;
    for line in lines {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|w| w.parse().expect("Should be a valid number"))
            .collect();
        let ascending = numbers[0] < numbers[numbers.len()-1];
        let mut safe = true;
        println!("{}", linenum);
        println!("{:?}", numbers);
        linenum += 1;
        let mut numbers_removed: Vec<Vec<i32>> = Vec::new();
        
        for i in 0..numbers.len() {
            let mut numbers_copy = numbers.clone();
            numbers_copy.remove(i);
            numbers_removed.push(numbers_copy);
        }

        let mut i = 1;
        loop {
            if i > numbers.len()-1  {
                break;
            }
            let current = numbers[i];
            let previous = numbers[i-1];
            if (ascending && previous > current)
                || (!ascending && previous < current)
                || (current.abs_diff(previous) > 3 || current.abs_diff(previous) == 0) {
                safe = false;
                break;
            }
            i += 1;
        }
        
        if !safe {
            for j in 0..numbers_removed.len() {
                safe = true;
                i = 1;
                loop {
                    if i > numbers_removed.len() -2 {
                        break;
                    }
                    let current = numbers_removed[j][i];
                    let previous = numbers_removed[j][i-1];
                    if (ascending && previous > current)
                        || (!ascending && previous < current)
                        || (current.abs_diff(previous) > 3 || current.abs_diff(previous) == 0) {
                        safe = false;
                        break;
                    }
                    i += 1;
                }
                if safe {
                    break;
                }
            }
        }
        if safe {
            sum += 1;
        }
        println!("{}", sum);
        println!();
    }
    println!("{}", sum);
}