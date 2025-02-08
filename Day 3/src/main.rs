fn main() {
    let contents = std::fs::read_to_string("resources/input.txt").expect("input.txt not found");
    let lines = contents.lines();
    
    let mut sum = 0;
    let mut enable = true;
    for mut line in lines {
        loop {
            let opt = line.find("mul(");
            let mul_index = match opt {
                Some(index) => index,
                None => break,
            };
            let opt = line.find("do");
            let do_index = match opt {
                Some(index) => index,
                None => mul_index,
            };
            
            if do_index < mul_index {
                let split_line = line.split_at(do_index);
                let do_section = &split_line.1[..7];
                if do_section[0..4] == *"do()" {
                    enable = true;
                    line = &split_line.1[4..];
                } else if do_section == "don't()" {
                    enable = false;
                    line = &split_line.1[7..];
                }
            } else {
                let split_line = line.split_at(mul_index);
                line = &split_line.1[4..];
            }
            
            if enable {                
                let opt = line.find(")");
                let bracket_index = match opt {
                    Some(index) => index,
                    None => continue,
                };
                match line.split_at(bracket_index).0.split_once(",") {
                    Some((x,y)) => {
                        if x.len() > 3 || y.len() > 3 {
                            continue;
                        }
                        if is_numeric(x) && is_numeric(y){
                            let num1: i32 = x.parse().expect("Should be a number");
                            let num2: i32 = y.parse().expect("Should be a number");
                            sum += num1*num2;
                        }
                    },
                    None => continue,
                };
            }
        }
    }
    println!("{}", sum);
}

fn is_numeric(x: &str) -> bool {
    for c in x.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
    }
    true
}
