use std::env;

mod year_2023;
mod year_2024;

fn main() {
    let mut args = env::args();

    args.next();
    
    let year = args.next().unwrap();
    let day = args.next().unwrap();

    let args = args.collect::<Vec<_>>();

    match &year[..] {
        "2023" => match &day[..] {
            "1" => year_2023::day1::run(args),
            "2" => year_2023::day2::run(args),
            "3" => year_2023::day3::run(args),
            "4" => year_2023::day4::run(args),
            "5" => year_2023::day5::run(args),
            "6" => year_2023::day6::run(args),
            "7" => year_2023::day7::run(args),
            _ => panic!("Invalid day for year {}", 2023)
        },
        "2024" => match &day[..] {
            "1" => year_2024::day1::run(args),
            "2" => year_2024::day2::run(args),
            "3" => year_2024::day3::run(args),
            _ => panic!("Invalid day for year {}", 2024)
        }
        _ => panic!("Inavlid year"),
    }
}