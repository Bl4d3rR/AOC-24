use std::env;

mod day_01;
mod day_02;
mod day_03;

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
                        println!("\nDay {}:", $day_num);
                        $mod_name::$func_name();
                    },
                )*
                _ => run_every_day(),
            }
        }

        fn run_every_day() {
            println!("running every day...");
            $(
                println!("\nDay {}:", $day_num);
                $mod_name::$func_name();
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
    3 => day_03::run_day_03
);
