use std::any::Any;
use std::ops::Add;
use good_lp::{coin_cbc, variable, variables, Expression, Solution, SolverModel};
use crate::Machine;

fn calc_for_machine(machine: &Machine) -> f64 {
    let mut res: f64 = 0.;


    variables! {
        vars:
    }

    // ?????

    let k = machine.buttons.len();
    let mut sum_expr = Expression::from(0);

    let mut var_list = Vec::new();
    for i in 0..k {
        let var_name = format!("x{}", i);
        let var = vars.add(variable().name(var_name.clone()).integer().min(0));
        var_list.push((var_name, var));

        sum_expr = sum_expr + var;
    }


    let n = machine.joltage.len();
    let mut constraints = Vec::new();
    for i in 0..n {
        // let var_name = format!("y{}", i);
        // let var = vars.add(variable().name(var_name.clone()).integer().min(0));
        // var_list.push((var_name, var));

        let buttons_id: Vec<usize> = machine.buttons.iter().enumerate().filter(|&(_, t)| {t.contains(&i)}).map(|x| {x.0}).collect();
        let mut condition = Expression::from(0);
        for j in buttons_id {
            condition += var_list[j].1
        }
        constraints.push(condition.eq(machine.joltage[i] as i32))
    }

    let mut solution = vars.minimise(sum_expr).using(coin_cbc).with_all(constraints).solve().unwrap();

    // println!("{:?}", solution) }
    //     .with(something.eq(1)).solve().unwrap();
    // println!("Minimalna suma: {:?}", solution.status());
    for (name, var) in var_list {
        println!("{name} {}", solution.value(var));
        res = res + solution.value(var);
    }

    res
}

pub fn calc(machines: &Vec<Machine>) -> f64 {
    let mut res: f64 = 0.;

    for machine in machines {
        res += calc_for_machine(machine);
    }

    res
}