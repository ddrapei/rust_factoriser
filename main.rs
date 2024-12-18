use std::io;

fn get_prime_factors(mut number: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    // deviding a number by 2, as long as we can
    while number % 2 == 0 {
        factors.push(2);
        number = number / 2;
    }

    // deviding by unpaired factors
    let mut i = 3;
    while i * i <= number {
        while number % i == 0 {
            factors.push(i);
            number = number / i;
        }
        i += 2;
    }

    // add the number the is left, ignores 1
    if number > 2 {
        factors.push(number);
    }

    factors
}

fn main() {
    println!("Input a number to factorise:");

    loop {
        let mut input = String::new();

        //getting input from a user
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let number: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("The input is not a number");
                continue;
            }
        };

        if number == 0 {
            println!("Exit...");
            break;
        }

        // Отримуємо та виводимо результат
        let factors = get_prime_factors(number);
        println!("The number {} has those factors: {:?}", number, factors);
        println!("\nEnter the second number, ot type (0 Enter)121 to exit");
    }
}
