use std::collections::HashMap;
use std::{io};
use std::io::Write;

use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    // task_1()
    let selections = &[
        "Task 1 - Enough Easter Eggs?",
        "Task 2 - Basketful of Eggs!",
        "Task 3 - Long Enough?",
        "Task 4",
        "Task 5 - Efficient Easter",
        "Task 6 - Who got the most eggs?",
        "Task 7",
        "Task 8",
        "Task 9",
        "Task 10",
        "Quit :("
    ];
    loop {
        let selection_index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select the task you would like to run")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        let selected_index = selection_index + 1;

        match selected_index {
                1 => { task_1()  }  
                2 => { task_2()  }  
                3 => { task_3()  }  
                4 => { task_4()  }  
                5 => { task_5()  }  
                6 => { task_6()  }  
                7 => { task_7()  }  
                8 => { task_8()  }  
                9 => { task_9()  }  
                10 =>{ task_10() }  
                
                11 => {
                    // Quit :(
                    println!("Goodbye!");
                    return 
                }
                _ => {
                    // Its none of the above?
                    println!("Wait a minuite...\n You're not meant to be here!")
                }

        }

        print!("\n    Press enter to return back to the main menu...");
        io::stdout().flush().unwrap_or({});
        io::stdin().read_line(&mut String::new()).unwrap_or(0);
    }

}

fn input(prompt: &String) -> String {

    loop {
        print!("{prompt}");

        // Flush stdout, since its a new line buffer, but we are not printing a new line
        if let Err(error) = io::stdout().flush() {
            // Print the error message and go to the next iteration
            println!("error: {error}", );
            println!("Try again...\n");
            continue
        }

        // Clear the buffer
        let mut inp_buffer = String::new();

        // Read stdin into the "buffer"
        match io::stdin().read_line(&mut inp_buffer) {
            Ok(_) => {
                return inp_buffer;
            }
            Err(error) => {
                // Print the error message and go to the next iteration
                println!("error: {error}", );
                println!("Try again...\n");
                continue
            }
        }
    }
}

fn int_input(prompt: &String) -> usize {
    loop {
        let str_input = input(&prompt);

        let int_input = str_input.trim().parse::<usize>();
        match int_input {
            Ok(int) => {
                return int;
            }
            Err(error) => {
                println!("error: {error} \nTry again...");
                continue    // Try again
            }
        }

    }
}

fn task_1() {
    println!(
        "
        *** Task 1 ***
        Write a program that calculates the total number of Easter eggs collected 
        over 5 days. The program must accept 5 separate inputs (storing the inputs 
        as integers) for the number of eggs collected each day. If the total is 50 
        or more, print \"Great job! You are an Easter egg superstar!\". Otherwise, 
        print \"Good effort, but lets try to find more eggs next time.\".
        "
    );

    let mut input_day       = 1_u8;
    let mut collected_eggs  = 0_usize;
    
    while input_day != 6 {

        let eggs = int_input(&format!("How many egges did you collect on day {input_day}? "));

        collected_eggs += eggs;
        input_day +=1;
    }

    if collected_eggs >= 50 {
        println!("Great job! You are an Easter egg superstar!")
    } else {
        println!("Good effort, but lets try to find more eggs next time.")
    }

}

fn task_2() {
    println!(
        "
        *** Task 2 ***
        The Easter Bunny is preparing baskets. For each basket, he needs 3 types of 
        eggs: chocolate, gold, and silver. The user inputs the number of each type 
        of egg per basket and the number of baskets. Calculate and print the total 
        number of eggs needed. 
        "
    );

    let mut num_choco   = 0_usize;
    let mut num_gold    = 0_usize;  
    let mut num_silver  = 0_usize;

    let mut collected = 0;
    while collected != 3 {

        let (egg_type, egg_var) = match collected {
            0 => {( "chocolate", &mut num_choco  )}
            1 => {( "gold",      &mut num_gold   )}        
            2 => {( "silver",    &mut num_silver )}
            _ => {
                panic!("You're not meant to see this!!!")
            }
        };

        let eggs = int_input(&format!("How many {egg_type} eggs per basket? "));

        *egg_var += eggs;
        collected += 1
        
    }

    let num_baskets = int_input(&String::from("How many baskets would you like?"));

    let total_choco = num_choco * num_baskets;
    let total_gold  = num_gold * num_baskets;
    let total_silver= num_silver * num_baskets;

    let total_total = total_choco + total_gold + total_silver;

    println!("
    Total chocolate eggs required: {total_choco}
    Total gold eggs required: {total_gold}
    Total silver eggs required: {total_silver}

    Total eggs required: {total_total}
    ")

}

fn task_3() {
    println!(
        "
        *** Task 3 ***
        Write a program that calculates the total time spent on an Easter egg hunt. The
        user inputs the time (in minutes) spent each day over 3 days. Print out the
        total number of hours. If the total time is more than 120 minutes, print \"Wow,
        that was a long hunt!\". Otherwise, print \"Efficient hunting!\". 
        "
    );

    let mut total_mins = 0_usize;
    let mut collected_day = 1;

    while collected_day != 4 {

        let day_str = match collected_day {
            1 => { "first" }
            2 => { "second"}        
            3 => { "third" }
            _ => {
                panic!("You're not meant to see this!!!")
            }
        };

        let mins = int_input(&format!("How many minuites did you spend on the {day_str} day? "));

        total_mins += mins;
        collected_day += 1
    }
    println!("You spend a total of {total_mins} mins");

    if total_mins > 4320 {
        println!("Woah...You beat time itself!!!")
    } else if total_mins > 120 {
        println!("Wow, that was a long hunt!")
    } else {
        println!("Efficient hunting!")
    }


}

fn task_4() {
    println!(
        "
        *** Task 4 ***
        An Easter event needs to distribute eggs evenly across 5 baskets. The user 
        inputs the total number of eggs and the program calculates how many eggs go 
        into each basket. If there are leftovers, it should also print the number of 
        leftover eggs. The output should be in the form of \"X eggs per basket with Y 
        leftover\". 
        "
    );

    let num_of_eggs_per_basket = int_input(&String::from("How many eggs do you have? "));
    let baskets = 5_usize;
    
    let remaining_eggs = num_of_eggs_per_basket % baskets;
    let eggs_per_basket = (num_of_eggs_per_basket-remaining_eggs) / baskets;

    println!("> {eggs_per_basket} eggs per basket with {remaining_eggs} left over")

}

fn task_5() {
    println!(
        "
        *** Task 5 ***
        Calculate the efficiency of 5 Easter Bunny's helpers. The user inputs the 
        number of eggs each helper can prepare in one day. The program calculates 
        and prints the average number of eggs prepared by each helper. Calculate 
        the total eggs prepared over the 4 days of Easter weekend (Good Friday 
        to Easter Monday) 
        "
    );

    let mut helpers_eggs_per_day = [0;5];
    
    for x in 0..5 {
        let bunny_num = x+1;
        let eggs = int_input(&String::from(format!("How many eggs can bunny {bunny_num} prepare in 1 day? ")));
        helpers_eggs_per_day[x] = eggs;
    }

    let max_eggs_per_day: usize = helpers_eggs_per_day.iter().sum();
    let max_eggs_per_week = max_eggs_per_day * 4;

    let average_helper_eggs = max_eggs_per_day / 5; 
    println!("> Each helper can prepare an average of {average_helper_eggs} per day");
    println!("> {max_eggs_per_week} eggs can be prepared over the 4 days of Easter weekend")
}

fn task_6() {
    println!(
        "
        *** Task 5 ***
        The Easter bunny visits 4 houses in a row, dropping off eggs. Write a program
        that asks for the number of eggs dropped at each house and prints the house 
        that got the most eggs. If two or more houses got the same highest number of 
        eggs, print \"Tie\". }}
        "
    );

    let mut eggs_in_houses = HashMap::new();

    for house_num in 1..5 {
        let eggs = int_input(&String::from(format!("How many eggs got dropped at house {house_num}? ")));
        eggs_in_houses.insert(house_num as usize, eggs);
    }

    let max_eggs = eggs_in_houses.values().max().unwrap();

    let mut max_occurences = HashMap::new();

    for (house_num, eggs) in eggs_in_houses.iter() {
        if eggs == max_eggs {
            max_occurences.insert(house_num.clone(), eggs);
        }
    }

    if max_occurences.len() == 1 { 
        let max_house_num = max_occurences.keys().max().unwrap();
        println!("> House {max_house_num} got the most eggs with {max_eggs} eggs!")
    } else {
        println!("> The following houses tied with {max_eggs} eggs!");

        for (house_num, _) in max_occurences {
            println!("> - House {house_num}");
        }

    }

}


fn task_7() {
    todo!()
}

fn task_8() {
    todo!()
}

fn task_9() {
    todo!()
}

fn task_10() {
    todo!()
}