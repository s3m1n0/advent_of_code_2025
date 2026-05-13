use crate::read_file::parse_file;

pub fn part_1() {
    let operation_numbers = parse_file();
    count_zero_hits(&operation_numbers);
    // println!("{:?}" , numbers);
}

fn count_zero_hits(operations: &Vec<i32>) {
    let mut solution = 0;

    let mut password = 50;

    for i in operations {
        password = (password + i).rem_euclid(100);

        if password == 0 {
            solution += 1;
        }
    }

    println!("{}", solution)
}

