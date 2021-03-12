extern crate rand;
extern crate termion;
extern crate failure;
use std::{fs::File, io::Write};
use std::io::{BufRead, BufReader};
use std::env;
use termion::{clear, color};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut world = [[0u8; 75]; 75];

    if args.len() < 3 {
        for i in 0..74 {
            for j in 0..74 {
                if rand::random() {
                    world[i][j] = 1;
                } else {
                    world[i][j] = 0;
                }
            }
        }
    } else {
        let filename = env::args().nth(2).unwrap();
        world = populate_from_file(filename).expect("File is corrupted.");
    }

    let mut current_generation = 0;
    let max_generation = match env::args().nth(1) {
        Some(v) => v.parse::<i32>().expect("Second argument must be a valid number."),
        None => 100
    };

    for _gens in 0..max_generation {
        let temp = generation(world);
        world = temp;
        current_generation += 1;
        println!("{}", clear::All);
        display_world(world);
        println!("{blue}Population at generation {g} is {c}", blue = color::Fg(color::Blue), g = current_generation, c = census(world));
    }

    world_to_file(world, "final_world.txt").expect("Unable to create file.");
}

fn census(world: [[u8; 75]; 75]) -> u16 {
    let mut count = 0;

    for i in 0..74 {
        for j in 0..74 {
            if world[i][j] == 1 {
                count += 1;
            }
        }
    }

    count
}

fn generation(world: [[u8; 75]; 75]) -> [[u8; 75]; 75] {
    let mut new_world = [[0u8; 75]; 75];

    for i in 0..74 {
        for j in 0..74 {
            let mut count = 0;
            if i > 0 {
                count = count + world[i - 1][j];
            }
            if i > 0 && j > 0 {
                count = count + world[i - 1][j - 1];
            }
            if i > 0 && j < 74 {
                count = count + world[i - 1][j + 1];
            }
            if i < 74 && j > 0 {
                count = count + world[i + 1][j - 1]
            }
            if i < 74 {
                count = count + world[i + 1][j];
            }
            if i < 74 && j < 74 {
                count = count + world[i + 1][j + 1];
            }
            if j > 0 {
                count = count + world[i][j - 1];
            }
            if j < 74 {
                count = count + world[i][j + 1];
            }

            new_world[i][j] = 0;

            if (count < 2) && (world[i][j] == 1) {
                new_world[i][j] = 0;
            }
            if world[i][j] == 1 && (count == 2 || count == 3) {
                new_world[i][j] = 1;
            }
            if (world[i][j] == 0) && (count == 3) {
                new_world[i][j] = 1;
            }
        }
    }

    new_world
}

fn populate_from_file(filename: String) -> Result<[[u8; 75]; 75], failure::Error> {
    let mut new_world = [[0u8; 75]; 75];
    let file = File::open(filename).expect("Unable to read from file.");
    let reader = BufReader::new(file);
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let l = line?;
        let mut words = l.split_whitespace();
        let left = words.next().unwrap();
        let right = words.next().unwrap();
        pairs.push((left.parse::<usize>()?, right.parse::<usize>()?))
    }

    for i in 0..74 {
        for j in 0..74 {
            new_world[i][j] = 0;
        }
    }

    for (x, y) in pairs {
        new_world[x][y] = 1;
    }

    Ok(new_world)
}

fn display_world(world: [[u8; 75]; 75]) {
    for i in 0..74 {
        for j in 0..74 {
            if world[i][j] == 1 {
                print!("{red}*", red = color::Fg(color::Red));
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn world_to_file(world: [[u8; 75]; 75], filename: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    let mut content = String::from("");

    for i in 0..74 {
        for j in 0..74 {
            if world[i][j] == 1 {
                content.push_str("*");
            } else {
                content.push_str(" ");
            }
        }
        content.push_str("\n");
    }

    file.write_all(content.as_bytes())
}