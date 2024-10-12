// IMPORTS

use std::process::Command;
use rand::Rng;
use std::io;
use rand::thread_rng;
use std::time::Duration;
use std::thread;

// FUNCTIONS

// A fn to clear the screen based on os.
fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear the screen");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear the screen");
    }
}

// The function to print the 3x3 grid.
fn print_grid(grid: &[[char; 3]; 3]) {
    println!("\n    1   2   3  \n");
    for (index, array) in grid.iter().enumerate() {
        let n: char = match index {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            _ => 'n',
        };
        println!("{} | {} | {} | {} |", n, array[0], array[1], array[2]);
        println!("");
    }
}

// The function to print header.
fn print_header() {
    clear_screen();
    println!("\nTIC TAC TOE\n------------\n");
}

// The function to print basic game instructions before starting.
fn print_intructions() {
    println!("- This is a simple tic tac toe game (3x3).");
    println!("- Good luck!");
}

// The function to get user/pc choice [x/o].
fn x_or_o_qn() -> (String, String) {
    let mut user_choice = String::new(); 
    let pc_choice: String;
    println!("\nChoose [x/o]: ");
    io::stdin().read_line(&mut user_choice).expect("Failed to read input!");
    let user_choice = user_choice.trim().to_string();
    if !(user_choice.eq_ignore_ascii_case("x") || user_choice.eq_ignore_ascii_case("o")) {
        println!("\nInput not in choices [x/o]! Bye.\n");
        std::process::exit(0);
    }
    println!("\n");
    if user_choice.eq_ignore_ascii_case("x") {
        pc_choice = "o".to_string();
    } else {
        pc_choice = "x".to_string();
    }
    (user_choice, pc_choice)
}

// The function to handle user input. 
fn user_input_handler(free_cells: &Vec<&str>) -> String {
    let mut ans: String = String::new(); 
    println!("\nYour i/p. free cells {:?} : ", free_cells);
    io::stdin().read_line(&mut ans).expect("Failed to read input!");
    let ans: String = ans.trim().to_string();
    ans
}

// The function to handle pc input selection 
fn pc_input_handler(free_cells: &Vec<&str>) -> String {
    let mut rng = thread_rng();
    let random_index: usize = rng.gen_range(0..free_cells.len());
    free_cells[random_index].to_string()
}

fn game_logic(grid: &[[char; 3]; 3], free_cells: &Vec<&str>) -> (String, bool) {

    // checking rows

    for row in grid {
        if row.iter().all(|x| *x == row[0] && *x != '_' ) {
            return  (format!("{} won!", row[0]).to_string(), false)
        }
    }

    // checking columns

    for col in 0..3 {
        if grid.iter().all(|row| row[col] == grid[0][col] && grid[0][col] != '_' ) {
            return (format!("{} won!", grid[0][col]).to_string(), false);
        }
    }

    // checking diagonals

    if grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] && grid[0][0] != '_'  {
        return (format!("{} won!", grid[0][0]).to_string(), false);
    }
    if grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] && grid[0][2] != '_' {
        return (format!("{} won!", grid[0][2]).to_string(), false);
    }

    // checking if the free_cells is empty 

    if free_cells.is_empty() {
        return ("its a draw".to_string(), false);
    }

    // for the game to continue. 
    return ("nil".to_string(), true);
}

// The function to handle cell marking.
fn handle_cells(grid: &mut [[char; 3]; 3], free_cells: &mut Vec<&str>, cell: &String, choice: &String) -> bool {
    if free_cells.contains(&cell.as_str()) {
        let row: &mut [char; 3];
        let column: usize = match cell.chars().nth(1).and_then(|c| c.to_digit(10)) {
            Some(digit) => digit as usize - 1, // Adjust for 0-based index
            None => return false, // or handle it appropriately
        };
        match cell.chars().next() {
            Some('a') => { row = &mut grid[0] },
            Some('b') => { row = &mut grid[1] },
            Some('c') => { row = &mut grid[2] },
            _ => return false,
        };

        match choice.chars().next() {
            Some(s) => { row[column] = s; },
            _ => return false,
        };

        free_cells.retain(|&s| s != cell.as_str());
        return true;
    } else {
        println!("\nErr: Wrong input, Try again.\n");
        return false;
    }
}

//  The mainloop fn.
fn main_loop() {
    // Declare and initiate 3x3 grid.
    let mut grid: [[char;3]; 3] = 
        [
            ['_', '_', '_'],
            ['_', '_', '_'],
            ['_', '_', '_']
        ];

    // Declare and initiate free_cells to track unmarked cells.
    let mut free_cells: Vec<&str> = vec![
        "a1", "a2", "a3",
        "b1", "b2", "b3",
        "c1", "c2", "c3"
    ];

    // get user, pc choice [x/o]
    let (user_choice, pc_choice) = x_or_o_qn(); 

    loop {
        // printing header.
        print_header();

        // printing the grid.
        print_grid(&grid);

        // user input and marking.
        let user_cell: String = user_input_handler(&free_cells);
        if !handle_cells(&mut grid, &mut free_cells, &user_cell, &user_choice) {
            thread::sleep(Duration::from_secs(1));
            continue;
        }
        println!("You chose: {}", user_cell);

        // computer input and marking.
        if !free_cells.is_empty() {
            let pc_cell: String = pc_input_handler(&free_cells);
            handle_cells(&mut grid, &mut free_cells, &pc_cell, &pc_choice);
            println!("Computer chose: {}", pc_cell);
        }

        // Game logic
        let (status, t_or_f) = game_logic(&grid, &free_cells);
        if !t_or_f {
            thread::sleep(Duration::from_secs(1));
            print_header();
            print_grid(&grid);
            println!("\n{}\n", status);
            break;
        }

        // 2 sec sleep before next iteration.
        thread::sleep(Duration::from_secs(2));
    }
}

// MAIN

fn main() {

    // print header.
    print_header();

    // print instructions.
    print_intructions();

    // Execute the mainloop.
    main_loop();

}

// END
