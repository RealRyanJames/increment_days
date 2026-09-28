use std::{ io::stdin };
use chrono::{ DateTime, Datelike, Local };
use crate::traits::DateNumIncrement;

pub mod traits;
pub mod increment;

struct DateNow {
    date: DateTime<Local>,
}

struct StateDateMesh {
    _month: increment::INCREMENT,
    _day: increment::INCREMENT,
    _year: increment::INCREMENT,
}

trait _NextMonth {
    fn get_month() -> StateDateMesh;
    fn get_day() -> StateDateMesh;
    fn get_year() -> StateDateMesh;
}

impl Default for DateNow {
    fn default() -> Self {
        return DateNow {
            date: Local::now(),
        };
    }
}

fn sub_by(i: increment::INCREMENT) -> increment::INCREMENT {
    return i;
}

fn add_by(i: increment::INCREMENT) -> increment::INCREMENT {
    return i;
}

impl Default for StateDateMesh {
    fn default() -> Self {
        let date: DateNow = Default::default();
        return StateDateMesh {
            _day: date.date.day() as i32,
            _month: date.date.month() as i32,
            _year: date.date.year() as i32,
        };
    }
}

fn create_input_messsage(s: String) -> String {
    return String::from(s);
}

struct MessageMeshDisplay {
    _message: String,
}

pub trait MessageMesh {
    fn getmesh();
}

impl Default for MessageMeshDisplay {
    fn default() -> Self {
        let date: DateNow = Default::default();
        return MessageMeshDisplay {
            _message: String::from(
                format!("{}/{}/{}", date.date.month(), date.date.day(), date.date.year())
            ),
        };
    }
}

trait SeptNumberMesh {
    fn get_number_months();
}

struct ValueMeshMonth {
    day: i32,
    month: i32,
    year: i32,
}

impl Default for ValueMeshMonth {
    fn default() -> Self {
        return ValueMeshMonth {
            day: 30,
            month: 9,
            year: 0,
        };
    }
}

impl traits::DateNumIncrement for increment::Increment {
    fn get_next() {
        let mut str = String::new();
        println!("{}", create_input_messsage(String::from("Enter Add - Sub: ")));
        stdin().read_line(&mut str).expect("No Input Entered");
        if str.contains("Add") {
            let today_ = StateDateMesh::default();

            match today_._month as i32 {
                1 => {
                    println!(
                        "Month: {} has {} Days",
                        today_._month,
                        (31 as i32).abs() - today_._day
                    );
                }

                2 => {
                    println!(
                        "Month: {} has {} Days",
                        today_._month,
                        (28 as i32).abs() - today_._day
                    );
                }

                3 => {
                    println!(
                        "Month: {} has {} Days",
                        today_._month,
                        (31 as i32).abs() - today_._day
                    );
                }

                4 => {
                    println!(
                        "Month: {} has {} Days",
                        today_._month,
                        (30 as i32).abs() - today_._day
                    );
                }

                5 => {
                    println!(
                        "Month: {} has {} Days",
                        today_._month,
                        (31 as i32).abs() - today_._day
                    );
                }

                6 => {
                    println!(
                        "Month: {} has {} Days",
                        today_._month,
                        (30 as i32).abs() - today_._day
                    );
                }

                7 => {
                    println!(
                        "Month: {} has {} Days",
                        today_._month,
                        (31 as i32).abs() - today_._day
                    );
                }

                8 => {
                    println!(
                        "Month: {} has {} Days",
                        today_._month,
                        (31 as i32).abs() - today_._day
                    );
                }

                9 => {
                    println!(
                        "Month: {} has {} Days",
                        today_._month,
                        (30 as i32).abs() - today_._day
                    );
                }

                10 => {
                    println!(
                        "Month: {} has {} Days",
                        today_._month,
                        (30 as i32).abs() - today_._day
                    );
                }

                11 => {
                    println!(
                        "Month: {} has {} Days",
                        today_._month,
                        (31 as i32).abs() - today_._day
                    );
                }

                12 => {
                    println!(
                        "Month: {} has {} Days",
                        today_._month,
                        (30 as i32).abs() - today_._day
                    );
                }
                _ => {}
            }
        }

        if str.contains("Sub") {
            let today = StateDateMesh::default();

            match today._month as i32 {
                1 => {
                    println!("Month: {} has {} Days", today._month, (31 as i32).abs() - today._day);
                }

                2 => {
                    println!("Month: {} has {} Days", today._month, (28 as i32).abs() - today._day);
                }

                3 => {
                    println!("Month: {} has {} Days", today._month, (31 as i32).abs() - today._day);
                }

                4 => {
                    println!("Month: {} has {} Days", today._month, (30 as i32).abs() - today._day);
                }

                5 => {
                    println!("Month: {} has {} Days", today._month, (31 as i32).abs() - today._day);
                }

                6 => {
                    println!("Month: {} has {} Days", today._month, (30 as i32).abs() - today._day);
                }

                7 => {
                    println!("Month: {} has {} Days", today._month, (31 as i32).abs() - today._day);
                }

                8 => {
                    println!("Month: {} has {} Days", today._month, (31 as i32).abs() - today._day);
                }

                9 => {
                    println!("Month: {} has {} Days", today._month, (30 as i32).abs() - today._day);
                }

                10 => {
                    println!("Month: {} has {} Days", today._month, (30 as i32).abs() - today._day);
                }

                11 => {
                    println!("Month: {} has {} Days", today._month, (31 as i32).abs() - today._day);
                }

                12 => {
                    println!("Month: {} has {} Days", today._month, (30 as i32).abs() - today._day);
                }
                _ => {}
            }
        }

        // if (today._day > 29)
    }
}

fn main() {
    println!("Today is: {}", MessageMeshDisplay::default()._message);

    loop {
        increment::Increment::get_next();
    }
}
