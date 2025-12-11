use advent_of_code::*;
use good_lp::{Expression, Solution, SolverModel, variable, variables};


fn main() {
    let input = get_input(2025, 10);

    
    let mut counter = 0;

    for line in input.lines() {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let config = parts[parts.len()-1][1..parts[parts.len()-1].len()-1].to_string();
        let buttons = &parts[1..parts.len()-1];

        let buttons = buttons.iter().map(|b| {
            let inner = &b[1..b.len()-1];
            let numbers = inner.split(',').map(|x| x.parse::<int>().unwrap()).collect::<Vec<_>>();
            numbers
        }
        ).collect::<Vec<_>>();

        let config = config.split(',').map(|x| x.parse::<int>().unwrap()).collect::<Vec<_>>();

        let mut vars = vec![];
        let mut var_problem = variables!();


        for _ in 0..buttons.len() {
            let v = var_problem.add(variable().min(0).integer());
            vars.push(v);
        }

        let mut problem = var_problem.minimise(
            vars.iter().fold(0.into(), |acc : Expression, v| acc + v)
        ).using(good_lp::microlp);

        for i in 0..config.len() {
            let mut expr : Expression = 0.into();
            for (j, button) in buttons.iter().enumerate() {
                if button.contains(&(i as int)) {
                    expr = expr + vars[j];
                }
            }
            problem.add_constraint(expr.eq(config[i] as f64));
        }

        let sol = problem.solve().unwrap();

        let value = sol.eval(&vars.iter().fold(0.into(), |acc : Expression, v| acc + v));

        counter += value.round() as int;
        
    }

    println!("{}", counter);
}
