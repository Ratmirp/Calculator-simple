use std::io::{self};
// Cclculator can't work with negative numbers for now
fn solve_without_bracket(mut e: Vec<String>) -> i32 {
    // must get a clear arr with 15 as 1 item not 2
    // find ^ first solve change str, move oni
    // TODO add recurtion while true
    while let Some(index) = e.iter().position(|x| x == "^") {
        let a: i32 = e[index - 1].parse().expect("Not a number!");
        let b: u32 = e[index + 1].parse().expect("Not a number!");
        let ans = a.pow(b).to_string();

        e.splice((index - 1)..=(index + 1), vec![ans]);
    }

    while let Some(index) = e.iter().position(|x| x == "*") {
        let a: i32 = e[index - 1].parse().expect("Not a number!");
        let b: i32 = e[index + 1].parse().expect("Not a number!");
        let ans = (a * b).to_string();

        e.splice((index - 1)..=(index + 1), vec![ans]);
    }

    while let Some(index) = e.iter().position(|x| x == "+" || x == "-") {
        let a: i32 = e[index - 1].parse().expect("Not a number!");
        let b: i32 = e[index + 1].parse().expect("Not a number!");

        let ans = if e[index] == "+" { a + b } else { a - b };

        e.splice((index - 1)..=(index + 1), vec![ans.to_string()]);
    }
    let final_number: i32 = e[0].parse().expect("Final result is not a valid number");
    return final_number;
}

fn main() {
    println!("Please write an equation");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("You typed: {input}");

    let equation: Vec<char> = input.chars().filter(|ch| !ch.is_whitespace()).collect();

    let mut clear_number: Vec<String> = Vec::with_capacity(equation.len());
    // STEP 0
    // connets long numbers together
    let mut number: String = "".to_string();
    for ch in equation {
        if ch.is_numeric() {
            number.push(ch);
        }
        if !ch.is_numeric() {
            if !number.is_empty() {
                clear_number.push(number.clone());
                number.clear();
            }
            clear_number.push(ch.to_string());
        }
    }
    if !number.is_empty() {
        clear_number.push(number);
    }
    // works till it has () inside
    while clear_number.contains(&String::from("(")) {
        let mut deepness: i32 = 0;
        let mut open_stack = Vec::new();
        let mut braket_info = Vec::new(); // contains (index of "(", index of ")", deepness)

        for (index, elem) in clear_number.iter().enumerate() {
            if *elem == "(" {
                open_stack.push(index);
                deepness += 1;
            }
            if *elem == ")" {
                let open_idx = open_stack.pop().unwrap();
                braket_info.push((open_idx, index, deepness));
                deepness -= 1;
            }
        }
        if let Some(&(start, end, depth)) = braket_info.iter().max_by_key(|x| x.2) {
            let par_result = solve_without_bracket(clear_number[(start + 1)..end].to_vec());
            let par_string = par_result.to_string();
            clear_number.splice(start..=end, vec![par_string]);
        }
    }
    let final_answer = solve_without_bracket(clear_number);
    println!("{final_answer}");
}
