use std::io;
use std::io::Write;

use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    // task_1()
    let selections = &[
        "Task 1 - Enough Easter Eggs?",
        "Task 2 - Basketful of Eggs!",
        "Task 3",
        "Task 4",
        "Task 5",
        "Task 6",
        "Task 7",
        "Task 8",
        "Task 9",
        "Task 10",
        "Quit :("
    ];

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
                println!("Thats not a task!")
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
    let mut collected_eggs  = 0_u8;
    
    while input_day != 6 {

        //
        // Print the prompt

        print!("How many egges did you collect on day {input_day}? ");
        let flush_res = io::stdout().flush();

        // Make sure the last operation was successfull
        match flush_res {
            Ok(_) => {}     // Continue in the current iteration
            Err(error) => {
                println!("error: {error} \nTry again...");
                continue    // Go to the next iteration
            },
        }


        //
        // Read input into a buffer

        let mut inp_buffer = String::new();                     // New "buffer" to store the string
        let inp_res = io::stdin().read_line(&mut inp_buffer);   // Read stdin into the "buffer"

        // Make sure the last operation was successfull
        match inp_res {
            Ok(_) => {}     // Continue
            Err(error) => {
                println!("error: {error} \nTry again...");
                continue    // Go to the next iteration
            },
        }

        //
        // Cast from string to u8

        let input = inp_buffer.trim().parse::<u8>();
        match input {
            Ok(_) => {}     // Continue
            Err(error) => {
                println!("error: {error} \nTry again...");
                continue    // Go to the next iteration
            }
        }

        //
        // Add to the total collected, and move onto the next day

        collected_eggs += input.unwrap(); // Still a Result type, so unwrap it.
        input_day += 1; // Next day
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
    let mut num_baskets = 0_u8;

    let mut num_choco   = 0_u8;
    let mut num_gold    = 0_u8;  
    let mut num_silver  = 0_u8;

    let mut collected = 0;
    while collected != 3 {

        let mut egg_type = "";
        let mut egg_var = &mut num_choco;

        match collected {
            0 => {
                egg_type = "chocolate";
                egg_var = &mut num_choco;
            }
            1 => {
                egg_type = "gold";
                egg_var = &mut num_gold;
            }        
            2 => {
                egg_type = "silver";
                egg_var = &mut num_silver;
            }
            _ => {
                panic!("You're not meant to see this!!!")
            }
        }

        print!("How many {egg_type} eggs per basket? ");

        //
        // Flush stdout
        let flush_res = io::stdout().flush();
        match flush_res {
            Ok(_) => {}
            Err(error) => {
                println!("error: {error} \nTry again...");
                continue    // Go to the next iteration
            },
        }
        
        //
        // Read the users input into a buffer
        let mut inp_buffer = String::new();
        let inp_res =  io::stdin().read_line(&mut inp_buffer);
        match inp_res {
            Ok(_) => {}
            Err(error) => {
                println!("error: {error} \nTry again...");
                continue    // Go to the next iteration
            }
        }
        
        //
        // Cast from string to u8
        let input = inp_buffer.trim().parse::<u8>();
        match input {
            Ok(_) => {}     // Continue
            Err(error) => {
                println!("error: {error} \nTry again...");
                continue    // Go to the next iteration
            }
        }

        *egg_var = input.unwrap();
        collected += 1;
    }

    loop {
        print!("How many baskets would you like? ");

        //
        // Flush stdout
        let flush_res = io::stdout().flush();
        match flush_res {
            Ok(_) => {}
            Err(error) => {
                println!("error: {error} \nTry again...");
                continue    // Go to the next iteration
            },
        }
        
        //
        // Read the users input into a buffer
        let mut inp_buffer = String::new();
        let inp_res =  io::stdin().read_line(&mut inp_buffer);
        match inp_res {
            Ok(_) => {}
            Err(error) => {
                println!("error: {error} \nTry again...");
                continue    // Go to the next iteration
            }
        }
        
        //
        // Cast from string to u8
        let input = inp_buffer.trim().parse::<u8>();
        match input {
            Ok(_) => {}     // Continue
            Err(error) => {
                println!("error: {error} \nTry again...");
                continue    // Go to the next iteration
            }
        }

        num_baskets = input.unwrap();
        break;
    }

    let total_choco = num_choco * num_baskets;
    let total_gold = num_gold * num_baskets;
    let total_silver = num_silver * num_baskets;

    let total_total = total_choco + total_gold + total_silver;

    println!("
    Total chocolate eggs required: {total_choco}
    Total gold eggs required: {total_gold}
    Total silver eggs required: {total_silver}

    Total eggs required: {total_total}
    ")

}

fn task_3() {
    todo!()
}

fn task_4() {
    todo!()
}

fn task_5() {
    todo!()
}

fn task_6() {
    todo!()
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