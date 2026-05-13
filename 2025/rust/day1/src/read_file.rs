pub fn parse_file() -> Vec<i32> {
    std::fs::read_to_string("../data/day1/data.dat")
        .unwrap()
        .lines()
        .map(|line| {
            let operator_sign = line.chars().next().unwrap();
            let number: i32 = line[1..].parse().unwrap();
            match operator_sign {
                'L' => -number,
                'R' => number,
                _ => panic!("bad data"),
            }
        })
        .collect()
}
