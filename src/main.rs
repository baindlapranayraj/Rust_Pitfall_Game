use rand::prelude::*;
use std::io::stdin;
#[allow(unused_variables)]

#[allow(dead_code)]
const OBSTACLE_DISPLAY: &str = "🔥";
#[allow(dead_code)]
const FREE_SPACE_DISPLAY: &str = "_";

const COURSE_LEN: usize = 32;

fn main() {
    let course = make_course(&COURSE_LEN);
    let mut place = 0;
    let mut first_roll = true;

    println!("🎮: {:?}",make_course_display(&course,&course.len()));

    'game: loop {
        // 1) First we have to let player to roll the dice
        // 2) Then we need to find the place where the player is
        // 3) And then we need to move the player to that particular place
        // 4) Then finallyyy  printout the current board

        let roll: (usize, usize) = roll_the_dice(1..=6, 1..=6);
        let roll = [roll.0, roll.1].iter().sum(); // Sum of tuples values

        let turn = PlayerTurn {
            roll,
            current_place: place,
        };

        println!("⏭️ {:?}",turn);

        let next_place: usize = match find_place(&turn,&COURSE_LEN,first_roll) {
            NextPlace::GameWon => {
                println!("🏆 You Won 🥳");
                break 'game;
            },
            NextPlace::Place(p)=>p
        };
        place = move_player(next_place,&course);
        
        let new_board = make_course_display(&course, &place);
        println!("🎯 {:?}",new_board);
        first_roll = false;
        println!("=================================");

    }
}

//For Creating the Skeleton layout for game to play
fn make_course(course_len: &usize) -> Vec<Space> {
    let mut board: Vec<Space> = vec![];
    let mut i = 0;
    while i < *course_len {
        let rand_num = thread_rng().gen_range(0..=1);
        match rand_num {
            0 => {
                let penalty = thread_rng().gen_range(2..=6);
                board.push(Space::Obstacle(penalty));
            }
            _ => board.push(Space::FreeSpace),
        }
        i += 1;
    }
    board
}

// For Adding emojis and place of the player based on postion
fn make_course_display(course: &Vec<Space>, place: &usize) -> Vec<String> {
    let mut board = vec![];
    for (i, spot) in course.iter().enumerate() {
        if i == *place {
            board.push(format!("+{}🦀",i));
        } else {
            match spot {
                Space::Obstacle(_) => board.push(OBSTACLE_DISPLAY.to_string()),
                Space::FreeSpace => board.push(FREE_SPACE_DISPLAY.to_string()),
            }
        }
    }
    board
}

// This is for player to let the roll the dice
fn roll_the_dice(
    roll_1: std::ops::RangeInclusive<usize>,
    roll_2: std::ops::RangeInclusive<usize>
) -> (usize, usize) {
    println!("🎲🎲 Roll The Dices...");
    stdin().read_line(&mut String::new()).expect("Not Valid");
    // Run this after player enters
    (thread_rng().gen_range(roll_1), thread_rng().gen_range(roll_2))
}

fn find_place(turn:&PlayerTurn,course_len:&usize,first_roll:bool) -> NextPlace {
    let next_place = turn.current_place + turn.roll;
    if next_place >= *course_len {
        NextPlace::GameWon
    }else if first_roll {
        NextPlace::Place(turn.roll-1)
    }else {
        NextPlace::Place(next_place)
    }
}


fn move_player(current_place:usize,course: &Vec<Space>)->usize{
    let updated_place: Option<&Space> = course.get(current_place);
    match updated_place {
        Some(Space::FreeSpace)=>{
            println!("✅ Stepped on Free Space");
            return  current_place;
        },
        Some(Space::Obstacle(v)) => {
            println!("🔥 Oops Stepped on Obstacle");
            return  hit_obstacle_next_place(&current_place,v);
        },
        None=> current_place
    }
}

fn hit_obstacle_next_place(temp_place:&usize,penality:&usize)->usize{
    let new_place = *temp_place - *penality;
    #[allow(unused_comparisons)]
    if new_place < 0 {
        return 0;
    }else {
        return new_place;
    }
}

#[derive(Debug)]
enum Space {
    Obstacle(usize),
    FreeSpace,
}

enum NextPlace{
    GameWon,
    Place(usize)
}

#[derive(Debug)]
struct PlayerTurn {
    roll: usize,
    current_place: usize,
}



// =================================== Learnings ===================================

// 1) iter() gives you references, there's no need to clone or copy the elements, which can
//  be more efficient for larger collections or complex data types.

// 2) Hear Enumerate function returns the tuple like above it return
//  -(0,Obstacle),(1,FreeSpace)....... so on.
