use std::io;

fn main() {
    println!("Fibonnaci nth Number Generator\n1.Generate nth Number\n2.Generate Entire Sequence: ");

    loop{
        println!("Choose Operation (1-2): ");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 || choice == 2 {
            if choice == 1
            {
                println!("nth Number\nPlease Enter A Number: ");
                operation(choice);
                //println!("{}",fibonacci_test(10, choice));
                break;
            }
            else {
                println!("Sequence Generation\nPlease Enter A Number: ");
                operation(choice);
                //println!("{}",fibonacci_test(10, choice));
                break;
            }
        }
        else {
            println!("please enter a valid number");
        }
    }
}

fn operation(execute:i32){
    let mut number= String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number.trim().parse().expect("Please type a number!");

    if execute == 1 {
        println!("Fibonacci({number}) = {}", fibonacci_nth(number+1));

        println!("Would You Like To See The Previous Numbers? (y/n)");

        loop{
            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let choice: char = choice.trim().parse().expect("please enter a character");

            if choice == 'y' || choice == 'n'{
                if choice == 'y'{
                    fibonacci(number);
                    break;
                }
                else{
                    break;
                }
            }
            else {
                println!("Please enter y or n: ");
            }
        }
    }
    else if execute == 2 {
        fibonacci(number + 1);
    }
    else {
        println!("Error, the calculation does not exist");
    }
}

fn fibonacci(num:i32){
    let mut previous_number = 0;
    let mut current_number = 1;

    for nth in 0..num {
        let temp = previous_number + current_number;

        if nth <= 0 {
            println!("Fibonacci({nth}) = {}", 0);
        }
        else if nth == 1 {
            println!("Fibonacci({nth}) = {}",1);
        }
        else {
            println!("Fibonacci({nth}) = {temp}");

            previous_number = current_number;
            current_number = temp;
        }
    }
}

fn fibonacci_nth(num:i32) -> i32 {
    let mut previous_number = 0;
    let mut current_number = 1;

    for nth in 0..num {
        let temp = previous_number + current_number;

        if nth <= 0 {
            continue;
        }
        else if nth == 1 {
            continue;
        }
        else {
            previous_number = current_number;
            current_number = temp;
        }
    }

    return current_number;
}

/*fn fibonacci_test(num: i32, choice: i32) -> i32 { //use arrays to print the whole sequence
    let mut previous_number = 0;
    let mut current_number = 1;
    let mut sequence: [i32,num];

    for nth in 0..num {
        let temp = previous_number + current_number;

        if choice == 1 && nth <= num{
            if nth <= 0 {
                continue;
            }
            else if nth == 1 {
                continue;
            }
            else {
                previous_number = current_number;
                current_number = temp;
            }
        }
        else {
            if nth <= 0 {
                sequence.push(nth);
            }
            else if nth == 1 {
                sequence.push(nth);
            }
            else {
                sequence.push(temp);
                previous_number = current_number;
                current_number = temp;
            }
        }
    }

    return current_number;
}*/
