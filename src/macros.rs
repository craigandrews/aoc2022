macro_rules! run_parts {
    ($mod_name:ident) => {
        let day = stringify!($mod_name);
        let input = format!("src/{}/input", day);
        println!("{}::part1 {}", day, $mod_name::part1(&input));
        println!("{}::part2 {}", day, $mod_name::part2(&input));
    };
}
