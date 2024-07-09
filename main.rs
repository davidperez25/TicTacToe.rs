use std::fmt;
use std::io;

fn strtoi(s: &str) -> Option<usize> {
    if let Some(first_char) = s.chars().next() {
        match first_char {
            'a'..='z' => Some(first_char as usize - 'a' as usize),
            _ => None,
        }
    } else {
        None
    }
}

struct Grid {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<usize>>,
}

impl Grid {
    fn new(rows: usize, cols: usize) -> Self {
        Grid {
            rows,
            cols,
            grid: vec![vec![2; cols]; rows],
        }
    }
    
    fn set(&mut self, coord: &str) {
        let letters: String = coord.chars()
            .filter(|c| !c.is_digit(10))
            .collect();
        
        let number = match strtoi(&letters) {
            Some(num) => num,
            None => return,
        };
        
        let nsidx = coord.chars().position(|c| c.is_digit(10));
        
        let cols: usize = match nsidx {
            Some(idx) => {
                let numstr = &coord[idx..];
                match numstr.parse::<usize>() {
                    Ok(num) => num,
                    Err(e) => {
                        println!("Error parsing number: {}", e);
                        0
                    },
                }
            },
            None => 0,
        };
        
        if number < self.rows && cols < self.cols && self.grid[number][cols] == 2 {
            self.grid[number][cols] = 1;
            self.ai();
        } else {
            println!("Cannot place there...");
        }
    }
    
    fn check(&self) -> i32 {
        let size = self.grid.len();

        for i in 0..size {
            if self.grid[i][0] == 1 && self.grid[i][1] == 1 && self.grid[i][2] == 1 {
                return 1;
            }
            if self.grid[i][0] == 0 && self.grid[i][1] == 0 && self.grid[i][2] == 0 {
                return 0;
            }
        }

        for j in 0..size {
            if self.grid[0][j] == 1 && self.grid[1][j] == 1 && self.grid[2][j] == 1 {
                return 1;
            }
            if self.grid[0][j] == 0 && self.grid[1][j] == 0 && self.grid[2][j] == 0 {
                return 0;
            }
        }

        if self.grid[0][0] == 1 && self.grid[1][1] == 1 && self.grid[2][2] == 1 ||
            self.grid[0][2] == 1 && self.grid[1][1] == 1 && self.grid[2][0] == 1 {
            return 1;
        }
        if self.grid[0][0] == 0 && self.grid[1][1] == 0 && self.grid[2][2] == 0 ||
            self.grid[0][2] == 0 && self.grid[1][1] == 0 && self.grid[2][0] == 0 {
            return 0;
        }
        
        for row in &self.grid {
            for &cell in row {
            
                if cell == 2 {
                    return 4;
                }
                
            }
        }
        0
    }
    
    fn ai(&mut self) { //algorithm not mine
        for i in 0..self.rows {
            for j in 0..self.cols {
                if j <= self.cols - 3 && self.grid[i][j] == 1 && self.grid[i][j + 1] == 1 && self.grid[i][j + 2] == 2 {
                    self.grid[i][j + 2] = 0;
                    return;
                } else if j >= 2 && self.grid[i][j] == 1 && self.grid[i][j - 1] == 1 && self.grid[i][j - 2] == 2 {
                    self.grid[i][j - 2] = 0;
                    return;
                }

                if i <= self.rows - 3 && self.grid[i][j] == 1 && self.grid[i + 1][j] == 1 && self.grid[i + 2][j] == 2 {
                    self.grid[i + 2][j] = 0;
                    return;
                } else if i >= 2 && self.grid[i][j] == 1 && self.grid[i - 1][j] == 1 && self.grid[i - 2][j] == 2 {
                    self.grid[i - 2][j] = 0;
                    return;
                }
            }
        }

        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.grid[i][j] == 2 {
                    self.grid[i][j] = 0;
                    return;
                }
            }
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for &cell in row {
                match cell {
                    2 => write!(f, "' ")?,
                    0 => write!(f, "x ")?,
                    1 => write!(f, "o ")?,
                    _ => write!(f, "? ")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn gloop(mut game: Grid) {
    println!("{}", game);
    
    match game.check() {
        0 => {
            println!("X wins!");
            return;
        }
        1 => {
            println!("O wins!");
            return;
        },
        3 => {
            println!("Draw!");
            return;
        },
        _ => {
            
        },
    }

    println!("Enter coordinates (e.g., 'a1', 'b2', etc.): ");
    
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line...");

    let input = input.trim();

    game.set(&input);
    
    gloop(game);
}

fn main() {
    let game = Grid::new(3, 3);
    gloop(game);
    
}
