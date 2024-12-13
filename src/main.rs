use std::env;
use std::time::Instant;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_11;
mod day_12;
mod day_13;

// Define a macro to create functions for each day
macro_rules! define_days {
    ( $( $day_num:expr => $mod_name:ident::$func_name:ident ),* ) => {
        fn main() {
            let args: Vec<String> = env::args().collect();

            if args.len() <= 1 {
                run_every_day();
                return;
            }

            let day: i64 = match args[1].parse::<i64>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid day. Running all days.");
                    0
                }
            };

            match day {
                $(
                    $day_num => {
                        let start = Instant::now();
                        println!("\nDay {}:", $day_num);
                        $mod_name::$func_name();
                        println!("took {}ms to solve both parts", start.elapsed().as_millis());
                    },
                )*
                _ => run_every_day(),
            }
        }

        fn run_every_day() {
            println!("running every day...");
            $(
                let start = Instant::now();
                println!("\nDay {}:", $day_num);
                $mod_name::$func_name();
                println!("took {}ms to solve both parts", start.elapsed().as_millis());
                println!("-------------");
            )*
            println!("\nfinished executing every day!");
        }
    };
}

// Use the macro to define functions for each day
define_days!(
    1 => day_01::run_day_01,
    2 => day_02::run_day_02,
    3 => day_03::run_day_03,
    4 => day_04::run_day_04,
    5 => day_05::run_day_05,
    6 => day_06::run_day_06,
    7 => day_07::run_day_07,
    11 => day_11::run_day_11,
    12 => day_12::run_day_12,
    13 => day_13::run_day_13

);
