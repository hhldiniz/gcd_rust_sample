use std::{io};

fn str_to_int(str_value: &String) -> i32 {
    str_value.trim().parse::<i32>().unwrap()
}

fn find_gcd(n1: i32, n2: i32) -> i32 {
    let mut gcd = 1;
    let mut divisor = 1;
    let mut rest_of_division_1: i32 = get_div_mod(divisor, n1);
    let mut rest_of_division_2: i32 = get_div_mod(divisor, n2);
    let mut result_of_division_1: i32 = get_div_result(divisor, n1);
    let mut result_of_division_2: i32 = get_div_result(divisor, n2);

    while result_of_division_1 > 1 && result_of_division_2 > 1 {
        if rest_of_division_1 == 0 && rest_of_division_2 == 0 {
            gcd *= divisor;
        }
        if rest_of_division_1 != 0 && rest_of_division_2 != 0 {
            divisor += get_next_prime_number(divisor).expect("Error while getting next prime number");
        }
        if rest_of_division_1 == 0 {
            result_of_division_1 = get_div_result(divisor, n1);
        }
        if rest_of_division_2 == 0 {
            result_of_division_2 = get_div_result(divisor, n2);
        }
        rest_of_division_1 = get_div_mod(divisor, result_of_division_1);
        rest_of_division_2 = get_div_mod(divisor, result_of_division_2);
        
        println!("Result of division 1 = {}", result_of_division_1);
        println!("Result of division 2 = {}", result_of_division_2);
        if divisor == 1 {
            divisor += 1;
        }
    }
    gcd
}

fn get_div_mod(divisor: i32, value: i32) -> i32 {
    value % divisor
}

fn get_div_result(divisor: i32, value: i32) -> i32 {
    value / divisor
}

fn get_next_prime_number(current_prime: i32) -> Result<i32, & 'static str> {
    let mut prime_lookup = current_prime + 1;
    let mut found_prime = false;
    while !found_prime {
        found_prime = is_prime(prime_lookup);
        if found_prime {
            return Ok(prime_lookup);
        } else {
            prime_lookup += 1;
        }
    }
    Err("There's something wrong, could find a prime!")
}

fn is_prime(number: i32) -> bool {
    for i in 2..number - 1 {
        if get_div_mod(i, number) == 0 {
            return false
        }
    }
    true
}

fn main() {
    let mut n1_input = String::new();
    let mut n2_input = String::new();
    println!("Insert the first number");
    io::stdin()
        .read_line(&mut n1_input)
        .expect("Error while reading first number");
    println!("Insert the second number");
    io::stdin()
        .read_line(&mut n2_input)
        .expect("Error while reading second number");

    let n1 = str_to_int(&n1_input);
    let n2 = str_to_int(&n2_input);

    println!(
        "The Greatest Common Divisor between {} and {} is {}",
        n1,
        n2,
        find_gcd(n1, n2)
    )
}
