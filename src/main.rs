use std::io::stdin;

fn main() {
    println!("Hello, please enter your weekly work hours.");

    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You entered: {}", input);
        }
        Err(error) => println!("Error: {}", error),
    };
    let workh: f64 = input.trim().replace(",", ".").parse().unwrap_or(0.0);
    input.clear();

    println!("Your weekly work hours are {}", workh);
    println!("Please enter your hours, seperated by whitespaces.");


    match stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You entered: {}", input);
        }
        Err(error) => println!("Error: {}", error),
    }
    let hours: f64 = input
        .split_whitespace()
        .map(|x|
            x.trim()
                .replace(",", ".")
                .parse::<f64>()
                .unwrap_or(0.0)
                .le(&24.0)
                .then(|| x.replace(",", ".").parse::<f64>().unwrap_or(0.0))
                .ok_or_else(|| println!("are you sure you worked more than 24h a day?"))
                .unwrap_or(0.0)
        )
        .sum();
    input.clear();

    let result = workh - hours;
    let mut result_time = String::new();
    result_time.push_str(result.floor().to_string().as_str());
    result_time.push_str(":");
    result_time.push_str(((result - result.floor()) * 60.0)
        .floor()
        .to_string()
        .eq("0")
        .then(|| "00")
        .unwrap_or(((result - result.floor()) * 60.0).floor().to_string().as_str())
    );

    println!("you still need to work {:.2} hours or {}h", result, result_time);

    stdin().read_line(&mut String::new()).unwrap();
}
