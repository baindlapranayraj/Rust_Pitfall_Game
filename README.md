# Pitfall Game in Rust

This project is a simple command-line game inspired by the classic **Pitfall**. Players roll dice to move along a randomly generated course filled with obstacles and free spaces. The goal is to reach the end without falling behind due to obstacles.

## Features

- Roll dice to advance along the course.
- Randomly generated obstacles with penalties.
- Simple and interactive command-line gameplay.

## How to Play

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/pitfall-game.git
   cd pitfall-game
   cargo build
   cargo run

 ## Technologies Used

 -Rust for game logic and execution.<br/>
 -rand crate for random number generation.

 ## Example Output

   ```bash
   🎲🎲 Roll The Dices...
⏭️ PlayerTurn { roll: 7, current_place: 0 }
✅ Stepped on Free Space
🎯 ["+0🦀", "_", "_", "🔥", "_", "_", "🔥", "_", "🔥", "_", "_", "🔥", "_"]
=================================

  
