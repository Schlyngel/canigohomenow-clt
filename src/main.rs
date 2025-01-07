use serde::{Deserialize, Serialize};
use std::fs;
use std::io::stdin;

#[derive(Debug, Serialize, Deserialize)]
struct Save {
    workh: f64,
    pause: Vec<f64>,
}

impl Save {
    fn new(workh: f64, pause: Vec<f64>) -> Self {
        Save { workh, pause }
    }
}

fn main() {
    let mut input = String::new();
    let mut workh: f64;
    let mut pause: Vec<f64> = vec![];
    if load_data().is_some() {
        Save { workh, pause } = load_data().unwrap();
        println!("Hello, your weekly work hours are {}", workh);
        println!("Change weekly work hours? [y/N]");
        stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "y" {
            clear_terminal();
            println!("Please enter your weekly work hours.");
            input.clear();
            workh = user_input_work_hours(&mut input);
            save_data(Save::new(workh, vec![]));
        }
    } else {
        println!("Hello, please enter your weekly work hours.");
        workh = user_input_work_hours(&mut input);
        clear_terminal();
        println!("Your weekly work hours are {}", workh);
        println!("Do you want to save your work hours for the future? [Y/n]");
        stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "n" {
            save_data(Save::new(workh, vec![]));
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
    let start_time = get_start_time(&mut input);
    clear_terminal();
    if !pause.is_empty() {
        println!(
            "Your breaktime is {}h for <6h, {}h for 6-9h and {}h for >9h.",
            pause[0], pause[1], pause[2]
        );
        println!("Do you want to change your breaktime? [y/N]");
        stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "y" {
            input.clear();
            clear_terminal();
            println!("Please enter your breaktime for <6h, 6-9h and >9h.");
            pause = get_pause_time(&mut input);
            save_data(Save::new(workh, pause.clone()));
        }
    } else {
        println!("Please enter your breaktime for <6h, 6-9h and >9h.");
        pause = get_pause_time(&mut input);
        clear_terminal();
        println!(
            "Your breaktime is {}h for <6h, {}h for 6-9h and {}h for >9h.",
            pause[0], pause[1], pause[2]
        );
        println!("Do you want to save your breaktime for the future? [Y/n].");
        stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            save_data(Save::new(workh, pause.clone()));
        }
        input.clear();
    }
    clear_terminal();
    println!(
        "You can go home at {}",
        calc_end_time(start_time, pause, result)
    );

    println!("press any key to quit...");

    // so the console doesnt close
    stdin().read_line(&mut String::new()).unwrap();
}

fn get_pause_time(input: &mut String) -> Vec<f64> {
    match stdin().read_line(input) {
        Ok(_) => {}
        Err(error) => println!("Error: {}", error),
    }
    let result: Vec<f64> = input_to_vec(input)
        .iter()
        .map(|x| if x > &5.0 { x / 60.0 } else { *x })
        .collect();
    input.clear();
    result
}

fn get_start_time(input: &mut String) -> f64 {
    match stdin().read_line(input) {
        Ok(_) => {}
        Err(error) => println!("Error: {}", error),
    }
    let result: f64 = input_to_vec(input).iter().sum::<f64>();
    input.clear();
    result
}

fn calc_end_time(start_time: f64, pause: Vec<f64>, remaining: f64) -> String {
    if remaining.lt(&6.0) {
        result_as_date(start_time + pause[0] + remaining)
    } else if remaining.gt(&9.0) {
        result_as_date(start_time + pause[2] + remaining)
    } else {
        result_as_date(start_time + pause[1] + remaining)
    }
}

fn load_data() -> Option<Save> {
    let buffer = fs::read_to_string("cighn_savefile").unwrap_or("".to_string());
    if !buffer.is_empty() {
        Some(serde_json::from_str(&buffer).unwrap())
    } else {
        None
    }
}

fn save_data(save: Save) {
    fs::write("cighn_savefile", serde_json::to_string(&save).unwrap())
        .expect("Unable to write file");
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

#[allow(clippy::obfuscated_if_else)]
fn result_as_date(result: f64) -> String {
    let mut result_time = String::new();
    result_time.push_str(result.floor().to_string().as_str());
    result_time.push(':');
    result_time.push_str(
        ((result - result.floor()) * 60.0)
            .floor()
            .to_string()
            .eq("0")
            .then_some("00")
            .unwrap_or(
                ((result - result.floor()) * 60.0)
                    .floor()
                    .to_string()
                    .as_str(),
            ),
    );
    result_time
}

fn user_input_hours(input: &mut String) -> (f64, Option<String>) {
    match stdin().read_line(input) {
        Ok(_) => {}
        Err(error) => println!("Error: {}", error),
    }
    let input_vec = input_to_vec(input);
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

fn input_to_vec(input: &mut str) -> Vec<f64> {
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
                    .map(|x| x.0.parse::<f64>().unwrap() + (x.1.parse::<f64>().unwrap() / 60.0))
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
