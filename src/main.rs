use std::io;

fn str_to_int(str_value: & String) -> i32 {
    str_value.trim().parse::<i32>().unwrap()
}

fn find_gcd(n1: i32, n2: i32) -> i32 {
    let mut gcd = 1;
    let mut rest_of_division_1: i32;
    let mut rest_of_division_2: i32;
    let mut divisor = 1;
    let mut result_of_division_1: i32 = get_div_result(divisor, n1);
    let mut result_of_division_2: i32 = get_div_result(divisor, n2);

    while result_of_division_1 > 1 && result_of_division_2 > 1 {
        rest_of_division_1 = get_div_mod(divisor, n1);
        rest_of_division_2 = get_div_mod(divisor, n2);
        result_of_division_1 = get_div_result(divisor, n1);
        result_of_division_2 = get_div_result(divisor, n2);

        if rest_of_division_1 == 0 && rest_of_division_2 == 0 {
            gcd = gcd * divisor;
        }
        divisor += 1;
    }
    gcd
}

fn get_div_mod(divisor: i32, value: i32) -> i32 {
    value %divisor
}

fn get_div_result(divisor: i32, value: i32) -> i32 {
    value / divisor
}

fn main() {
    let mut n1_input= String::new();
    let mut n2_input= String::new();
    println!("Insert the first number");
    io::stdin().read_line(&mut n1_input).expect("Error while reading first number");
    println!("Insert the second number");
    io::stdin().read_line(&mut n2_input).expect("Error while reading second number");

    let n1 = str_to_int(&n1_input);
    let n2 = str_to_int(&n2_input);

    println!("The Greatest Common Divisor between {} and {} is {}", n1, n2, find_gcd(n1, n2))
}
