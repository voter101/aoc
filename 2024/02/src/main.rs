use std::fs;

fn is_safe(report: &Vec<u32>, allow_bad_levels: bool) -> bool {
    let mut inc: Option<bool> = None;
    let mut defect_detected = false;
    for i in 1..report.len() {
        let a = report[i - 1];
        let b = report[i];

        if inc == None {
            inc = if a < b { Some(false) } else { Some(true) };
        }

        let diff = ((a as i64) - (b as i64)).abs();

        if diff > 3 || (inc == Some(true) && a <= b) || (inc == Some(false) && a >= b) {
            defect_detected = true
        }
    }

    // Not happy with the solution, but I was off by 3 in a "smarter" approach when iterating by pairs
    if defect_detected {
        if allow_bad_levels {
            for i in 0..report.len() {
                let mut dup = report.clone();
                dup.remove(i);
                if is_safe(&dup, false) {
                    return true;
                }
            }
        }
        return false;
    }

    true
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("File not loaded");
    let reports: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|input| input.parse::<u32>().unwrap())
                .collect::<_>()
        })
        .collect::<_>();

    let safe_reports = reports.iter().filter(|l| is_safe(l, false)).count();
    println!("{}", safe_reports);

    let dampened_safe_reports = reports.iter().filter(|l| is_safe(l, true)).count();
    println!("{}", dampened_safe_reports);
}
