use std::io;

fn temperature_converter(){
    println!("Temperature converter");
    println!("Please, select an option:");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    let option: u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid option");
            return;
        }
    };

    match option{
        1 => {
            println!("Please, enter the temperature in Celsius: ");
            let mut celsius = String::new();
            io::stdin()
                .read_line(&mut celsius)
                .expect("Failed to read line");

            let celsius: f64 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid temperature");
                    return;
                }
            };

            let fahrenheit = (celsius * 9.0/5.0) + 32.0;
            println!("The temperature in Fahrenheit is: {}", fahrenheit);
        },
        2 => {
            println!("Please, enter the temperature in Fahrenheit: ");
            let mut fahrenheit = String::new();
            io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Failed to read line");

            let fahrenheit: f64 = match fahrenheit.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid temperature");
                    return;
                }
            };

            let celsius = (fahrenheit - 32.0) * 5.0/9.0;
            println!("The temperature in Celsius is: {}", celsius);
        },
        _ => println!("Invalid option, please try again"),
    }        
}

fn fibonacci_sequence(){
    println!("Fibonacci sequence");
    println!("Please, enter the number of elements of the sequence: ");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    let mut a = 0;
    let mut b = 1;
    let mut c;

    for _ in 0..n {
        print!("{} ", a);
        c = a + b;
        a = b;
        b = c;
    }
    println!();
}

fn fib(u: u32) -> u32 {
    if u <= 0{
        return 0;
    } else if u == 1{
        return 1;
    } else {
        fib(u-1) + fib(u-2)
    }

}


fn fibonacci_sequence_recursive(){
    println!("Fibonacci sequence");
    println!("Please, enter the number of elements of the sequence: ");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    for i in 0..n {
        print!("{} ", fib(i));
    }
    println!();
}

fn christmas_carol(){
    println!("The Twelve Days of Christmas");
    // The Twelve Days of Christmas using an array of strings
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a partridge in a pear tree", "two turtle doves", "three French hens", "four calling birds", "five golden rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];

    for i in 0..12 {
        println!("On the {} day of Christmas my true love sent to me", days[i]);
        for j in (0..i+1).rev() {
            if j == 0 && i != 0 {
                print!("And ");
            }
            println!("{}", gifts[j]);
        }
        println!();
    }
}

fn main() {
    println!("Welcome to the randomexs project!");
    println!("This project contains some exercises of the book 'The Rust Programming Language'");
    
    loop {

        println!("Menu:");
        println!("1. Temperature converter");
        println!("2. Fibonacci sequence");
        println!("3. Fibonacci but recursive!");
        println!("4. Christams carol");
        println!("5. Exit");
        println!("Please, select an option: ");

        let mut option = String::new();
    
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid option");
                return;
            }
        };

        match option {
            1 => temperature_converter(),
            2 => fibonacci_sequence(),
            3 => fibonacci_sequence_recursive(),
            4 => christmas_carol(),
            5 => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid option, please try again"),
        }
    }
}


