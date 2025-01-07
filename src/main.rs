use std::fs;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    let mut workh: f64;
    if load_workh().is_normal() {
        workh = load_workh();
        println!("Hello, your weekly work hours are {}", workh);
        println!("Change weekly work hours? [y/N]");
        stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "y" {
            clear_terminal();
            println!("Please enter your weekly work hours.");
            input.clear();
            workh = user_input_work_hours(&mut input);
            save_workh(workh);
        }
    } else {
        println!("Hello, please enter your weekly work hours.");
        workh = user_input_work_hours(&mut input);
        clear_terminal();
        println!("Your weekly work hours are {}", workh);
        println!("Do you want to save your work hours for the future? [Y/n]");
        stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "n" {
            save_workh(workh);
        }
        input.clear();
    }

    clear_terminal();
    println!("Please enter your hours, seperated by whitespaces.");

    let hours: f64;
    loop {
        let (h, e) = user_input_hours(&mut input);
        match e {
            None => {
                hours = h;
                break;
            }
            Some(e) => {
                clear_terminal();
                println!("{}", e);
                println!("Please enter your daily hours, seperated by whitespaces.");
                continue;
            }
        }
    }

    let result = workh - hours;
    let result_time = result_as_date(result);
    clear_terminal();

    println!(
        "you still need to work {:.2} hours or {}h",
        result, result_time
    );
    println!("Please enter you start time.");
    let end_time = calc_end_time(&mut input, result);
    clear_terminal();
    println!("You can go home at {}", end_time);

    println!("press any key to quit...");

    // so the console doesnt close
    stdin().read_line(&mut String::new()).unwrap();
}

fn calc_end_time(mut input: &mut String, hours: f64) -> String {
    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => println!("Error: {}", error),
    }
    let result = result_as_date(input_to_vec(&mut input).iter().sum::<f64>() + hours);
    input.clear();
    result
}

fn load_workh() -> f64 {
    fs::read_to_string("cighn_savefile")
        .unwrap_or("0".to_string())
        .parse::<f64>()
        .unwrap()
}

fn save_workh(workh: f64) {
    fs::write("cighn_savefile", workh.to_string()).expect("Unable to write file");
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn result_as_date(result: f64) -> String {
    let mut result_time = String::new();
    result_time.push_str(result.floor().to_string().as_str());
    result_time.push_str(":");
    result_time.push_str(
        ((result - result.floor()) * 60.0)
            .floor()
            .to_string()
            .eq("0")
            .then(|| "00")
            .unwrap_or(
                ((result - result.floor()) * 60.0)
                    .floor()
                    .to_string()
                    .as_str(),
            ),
    );
    result_time
}

fn user_input_hours(mut input: &mut String) -> (f64, Option<String>) {
    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => println!("Error: {}", error),
    }
    let input_vec = input_to_vec(&mut input);
    let mut hours_err = None;
    for x in input_vec.iter() {
        if x > &24.0 {
            hours_err = Some(format!("Are you sure you worked {} hours a day?", x));
        }
    }
    let hours = input_vec.iter().sum::<f64>();
    input.clear();

    (hours, hours_err)
}

fn input_to_vec(input: &mut &mut String) -> Vec<f64> {
    let input_vec = input
        .split_whitespace()
        .map(|x| x.trim())
        .collect::<Vec<&str>>();
    let mut output_vec = Vec::new();
    for x in input_vec {
        if x.contains(":") {
            output_vec.push(
                x.trim()
                    .split_once(':')
                    .map(
                        |x| x.0.parse::<f64>().unwrap() + (x.1.parse::<f64>().unwrap() / 60.0)
                    )
                    .unwrap_or(0.0),
            )
        } else {
            output_vec.push(x.replace(",", ".").parse::<f64>().unwrap());
        }
    }
    output_vec
}

fn user_input_work_hours(input: &mut String) -> f64 {
    match stdin().read_line(input) {
        Ok(_) => {}
        Err(error) => println!("Error: {}", error),
    };
    let workh: f64 = input.trim().replace(",", ".").parse().unwrap_or(0.0);
    input.clear();
    workh
}
