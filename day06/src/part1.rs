fn calculate_stack(stack: Vec<usize>, operation: &str) -> usize {
    let mut res = stack[0];

    if operation == "*" {
        for &val in stack.iter().skip(1) {
            res *= val;
        }
    }
    else {
        for &val in stack.iter().skip(1) {
            res += val;
        }
    }

    return res;
}

pub fn calc(elements: &Vec<Vec<&str>>) -> usize {
    let n = elements[0].len();
    let mut res: usize = 0;

    for i in 0..n {
        let mut stack: Vec<usize> = Vec::new();
        for line in elements {
            if line[i] != "*" && line[i] != "+" {
                let val: usize = line[i]
                    .parse()
                    .expect("Error during translating the value from column");
                stack.push(val);
            }
        }
        let single_result = calculate_stack(stack, elements.last().unwrap()[i]);
        res += single_result;
    }
    return res;
}
