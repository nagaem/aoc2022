mod utils;

fn sum_calories() -> Vec<u32> {
    let mut cals = Vec::new();
    let mut new: bool = true;
    // taken directly from `rust-by-example`
    if let Ok(lines) = utils::read_lines("./input_files/day1.txt") {
        for line in lines {
            if let Ok(l) = line {
                let parsed: Result<u32, ParseIntError> = l.parse();
                match parsed {
                    Ok(cal) => {
                        if new {
                            cals.push(cal);
                            new = false;
                        } else {
                            let size = cals.len();
                            cals[size - 1] += cal;
                        }
                    },
                    Err(_error) => {
                        new = true;
                        continue;
                    },
                };

            }
        }
    }

    return cals;
}
// keeping it simple for day 1, will refactor as necessary
pub fn day1() {
    let mut cal_vec = sum_calories();
    cal_vec.sort();
    let total_elves = cal_vec.len();

    // puzzle 1
    println!("The highest calories carried by an elf is {}.", cal_vec[total_elves - 1]);

    // puzzle 2
    let cals: u32 = cal_vec[total_elves - 3..total_elves].iter().sum();
    println!("The top three elves are carrying {} calories.", cals);
}