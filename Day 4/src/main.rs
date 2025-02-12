

fn main() {
    let contents = std::fs::read_to_string("resources/input.txt").expect("input.txt not found");
    let lines:Vec<&str> = contents.lines().collect();

    let right_substring = "MAS";
    let left_substring = "SAM";
    let mut sum = 0;
    // for line in &lines {
    //     let mut line_copy = line.get(0..line.len()).expect("Should be a valid line");

    //     loop {
    //         let final_index = line_copy.len();
    //         let right_index = line_copy.find(right_substring).unwrap_or(final_index);
    //         let left_index = line_copy.find(left_substring).unwrap_or(final_index);
            
    //         match right_index.cmp(&left_index) {
    //             std::cmp::Ordering::Less => {
    //                 sum += 1;
    //                 line_copy = &line_copy[right_index+1..];
    //                 continue;
    //             },
    //             std::cmp::Ordering::Greater => {
    //                 sum += 1;
    //                 line_copy = &line_copy[left_index+1..];
    //                 continue;
    //             },
    //             std::cmp::Ordering::Equal => {
    //                 break;
    //             }
    //         }
    //     }
    // }
    // for column in 0..lines[0].len() {
    //     let mut buffer = String::new();
    //     for line in &lines {
    //         buffer.push_str(&line[column..column+1]);
    //     }
    //     loop {
    //         let final_index = buffer.len();
    //         let right_index = buffer.find(right_substring).unwrap_or(final_index);
    //         let left_index = buffer.find(left_substring).unwrap_or(final_index);
            
    //         match right_index.cmp(&left_index) {
    //             std::cmp::Ordering::Less => {
    //                 sum += 1;
    //                 buffer = buffer[right_index+1..].into();
    //                 continue;
    //             },
    //             std::cmp::Ordering::Greater => {
    //                 sum += 1;
    //                 buffer = buffer[left_index+1..].into();
    //                 continue;
    //             },
    //             std::cmp::Ordering::Equal => {
    //                 break;
    //             }
    //         }
    //     }
    // }
    
    for y in 0..lines.len()-2 {
        for x in 0..lines.len()-2 {
            let mut diagonal1 = String::new();
            let mut diagonal2 = String::new();
            let mut i = 0;
            loop {
                if i == 3 {
                    break;
                }
                diagonal1.push(lines[y+i].chars().nth(x+i).unwrap());
                diagonal2.push(lines[y+i].chars().nth(x+2-i).unwrap());
                i += 1;
            }
            if (diagonal1 == right_substring 
                || diagonal1 == left_substring) 
                && (diagonal2 == right_substring 
                ||diagonal2 == left_substring) {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}