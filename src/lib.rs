// The solving logic
// Gets a String equation moves it to vec of items and solves

// for solving without brackets
fn simple_solve(mut e: Vec<String>) -> f64 {
    while let Some(index) = e.iter().position(|x| x == "^") {
        let a: f64 = e[index - 1].parse().expect("Not a number!");
        let b: f64 = e[index + 1].parse().expect("Not a number!");
        let ans = a.powf(b).to_string();

        e.splice((index - 1)..=(index + 1), vec![ans]);
    }

    while let Some(index) = e.iter().position(|x| x == "/") {
        let a: f64 = e[index - 1].parse().expect("Not a number!");
        let b: f64 = e[index + 1].parse().expect("Not a number!");
        let ans = (a / b).to_string();

        e.splice((index - 1)..=(index + 1), vec![ans]);
    }

    while let Some(index) = e.iter().position(|x| x == "*") {
        let a: f64 = e[index - 1].parse().expect("Not a number!");
        let b: f64 = e[index + 1].parse().expect("Not a number!");
        let ans = (a * b).to_string();

        e.splice((index - 1)..=(index + 1), vec![ans]);
    }
    while let Some(index) = e.iter().position(|x| x == "-") {
        let a: f64 = e[index - 1].parse().expect("Not a number!");
        let b: f64 = e[index + 1].parse().expect("Not a number!");
        let ans = a - b;
        e.splice((index - 1)..=(index + 1), vec![ans.to_string()]);
    }

    while let Some(index) = e.iter().position(|x| x == "+") {
        let a: f64 = e[index - 1].parse().expect("Not a number!");
        let b: f64 = e[index + 1].parse().expect("Not a number!");
        let ans = a + b;

        e.splice((index - 1)..=(index + 1), vec![ans.to_string()]);
    }

    let mut final_number: f64 = 0.0;
    for last in e {
        //println!("{last}");
        let last_number: f64 = last
            .parse()
            .expect("unexpected error. Couldn't parse number");
        final_number += last_number;
    }
    return final_number;
}
pub fn solve(input: String) -> Result<f64, ()> {
    let input: Vec<char> = input.chars().filter(|ch| !ch.is_whitespace()).collect();
    let mut equation: Vec<String> = Vec::with_capacity(input.len());

    let mut item: String = String::new();
    for ch in input {
        if ch.is_numeric() {
            item.push(ch);
        }
        if !ch.is_numeric() {
            if !item.is_empty() {
                equation.push(item.clone());
                item.clear();
            }
            if ch == '-' || ch == '.' {
                item.push(ch);
            } else {
                equation.push(ch.to_string());
            }
        }
    }
    if !item.is_empty() {
        equation.push(item);
    }
    while equation.contains(&String::from("(")) {
        let mut brackets: Vec<(i32, i32, i32)> = Vec::new();
        let mut open_stack = Vec::new();
        let mut depth: i32 = 0;

        for (index, elem) in equation.iter().enumerate() {
            if *elem == "(" {
                open_stack.push(index as i32);
                depth += 1;
            } else if *elem == ")" {
                let open_id = open_stack.pop().unwrap();
                let record: (i32, i32, i32) = (open_id, index as i32, depth);
                brackets.push(record);
                depth -= 1;
            }
        }
        if let Some(&(start, end, depth)) = brackets.iter().max_by_key(|x| x.2) {
            //println!("{start}, {end}, {depth}");
            let slice = &equation[(start + 1) as usize..end as usize].to_vec();
            let solved_slice = simple_solve(slice.clone());

            equation.splice(
                start as usize..=end as usize,
                vec![solved_slice.to_string()],
            );
        }
        //println!("{equation:?}")
    }
    Ok(simple_solve(equation))
}
