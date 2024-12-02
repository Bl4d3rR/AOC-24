use std::env;

mod day_02;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        run_every_day();
        return;
    }

    let day: i64 = match args[1].parse::<i64>() {
        Ok(num) => num,
        Err(num) => {
            println!("{}", num);
            0
        }
    };

    match day {
        2 => day_2(),
        _ => run_every_day(),
    }
}

fn run_every_day() {
    println!("running every day...\n");

    day_2();

    println!("finished executing every day!")
}

fn day_2() {
    println!("------------------");
    println!("Day two: ");

    day_02::run_day_02();
    println!("------------------");
}
