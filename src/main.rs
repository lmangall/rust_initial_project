fn main() {

    let sum = add(3, 5);
    println!("The sum is: {}", sum);

    let day_of_week = "Sunday";
    if day_of_week == "Sunday" {
        println!("Race day");
    } else if day_of_week == "Saturday" {
        println!("Qualifying day");
    } else {
        println!("Rest day");
    }

    let mut counter = 0;

    while counter <= 5 {
        println!("Counter value is {}", counter);
        counter += 1;
    }

    for number in 1..=5{
        println!{"Number is {}", number};
    }

    counter = 0;
    loop {
        println!("Counter value is {}", counter);
        counter +=1;
        if counter == 6 {
            break;
        }
    }

    let num = 5;
    match num {
        1 => {
            println!("It's one");
            println!{"First stuff"};
        }
        2 => println!("It's two"),
        3 => println!("It's three"),
        _ => println!("It's something else"),
    }

    let result = match num {
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "Something else",
    };

    println!("Result is {}", result);

    }


fn add(x: i32, y: i32) -> i32 {
    let result = x + y;
    return result;
}

fn no_param() -> i32 {
    println!("It works!");
    1
}
 