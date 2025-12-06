fn calculate_stack(stack: &Vec<usize>, operation: &str) -> usize {
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

pub fn calc(elements: &Vec<Vec<usize>>, operations: &Vec<&str>) -> usize {
    let n = elements.len();
    let mut res: usize = 0;

    for i in 0..n {
        let single_result = calculate_stack(&elements[i], operations[i]);
        res += single_result;
    }
    return res;
}