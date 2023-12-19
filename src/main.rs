// Ex1: Convert temperatures between Fahrenheit and Celsius.
fn fahrenheit2celsius(num: f64) -> f64 {
    (num - 32.0) * 5.0 / 9.0
}

fn celsius2fahrenheit(num: f64) -> f64 {
    9.0 / 5.0 * num + 32.0
}

// Ex2: Generate the nth Fibonacci number.
fn fibonacci(num: u32) -> u32 {
    if num == 0 {
        0
    } else if num == 1 {
        1
    } else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}

/*
 * Ex3: Print the lyrics to the Christmas carol
 * “The Twelve Days of Christmas,” taking advantage of the
 * repetition in the song.
 */
fn twelve_days_of_christmas() {
    let chorus: [&str; 12] = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "a partridge in a pear tree.",
    ];

    let order_numbers: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh",
        "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    for str_num in order_numbers {
        println!(
            "\
            On the {str_num} day of Christmas,\n\
            my true love gave to me
            "
        );
        for i in (0..11).rev() {
            println!("{}", chorus[i]);
        }
    }
}
fn main() {
    println!("\n\n---Welcome to Common Concepts Practice!---\n\n");

    // Ex1
    let numb = 100.0;
    println!(
        "\
        Ex1: Convert temperatures between Fahrenheit and Celsius\n\
        The temperature is {}°F is {}°C\n\
        The temperature is {}°C is {}°F\
        ",
        numb,
        fahrenheit2celsius(numb),
        numb,
        celsius2fahrenheit(numb)
    );

    // Ex2
    let numb = 25;
    println!(
        "\
        Ex2: Generate the nth Fibonacci number\n\
        The {numb}th Fibonacci number is {}\
        ",
        fibonacci(numb),
    );

    // Ex3
    println!("Ex3: Print the lyrics to the Christmas carol\n");
    twelve_days_of_christmas();
}
