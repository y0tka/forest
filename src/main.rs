use ansi_term::Colour;
use rand::Rng;
use std::iter::repeat_with;
use std::thread;
use std::time::Duration;

const FIELD_SIZE: usize = 10;
const FIELD_AREA: usize = FIELD_SIZE * FIELD_SIZE;
const GRASS_COUNT: usize = 50;
const TREE_COUNT: usize = 20;
const FRAME_DELAY: u64 = 250;
const SIMULATION_STEPS: usize = 10;

fn main() {
    let mut field: Vec<Cell> = repeat_with(|| Cell::new()).take(FIELD_AREA).collect();

    // GRASS FILL
    rnd_fill_empty(&mut field, GRASS_COUNT, State::Grass);
    rnd_fill_empty(&mut field, TREE_COUNT, State::Tree);

    // RANDOM AGE
    for cell in field.iter_mut() {
        cell.age = rand::thread_rng().gen_range(0..10);
    }

    // SIMULATION
    for _ in 0..SIMULATION_STEPS {
        clear();
        for c in field.iter_mut() {
            *c = c.step();
        }
        print_field(&field);
        thread::sleep(Duration::from_millis(FRAME_DELAY));
    }
}

fn clear() {
    print!("{}[2J", 27 as char);
}

fn coords_to_linear(x: usize, y: usize) -> Result<usize, &'static str> {
    let res = x * FIELD_SIZE + y;
    if res < FIELD_AREA {
        return Ok(res);
    } else {
        return Err("Out of Bounds");
    }
}

fn rnd_fill_empty(field: &mut Vec<Cell>, count: usize, cell_type: State) {
    let mut to_fill: usize = count;
    while to_fill > 0 {
        let x = rand::thread_rng().gen_range(0..10);
        let y = rand::thread_rng().gen_range(0..10);
        match coords_to_linear(x, y) {
            Ok(coord) => match field[coord].cell_type {
                State::Empty => {
                    field[coord].cell_type = cell_type.clone();
                    to_fill -= 1;
                }
                _ => (),
            },
            Err(_) => panic!("OH SHIT *dies from cringe*"),
        }
    }
}

fn print_grass(age: u8) {
    print!(
        "{} ",
        Colour::Green.paint(match age {
            0 => "▁",
            1 => "▂",
            2 => "▃",
            3 => "▄",
            4 => "▅",
            5 => "▆",
            6 => "▇",
            _ => "󱔐",
        })
    );
}

fn print_tree(age: u8) {
    print!(
        "{} ",
        Colour::Green.paint(match age {
            0 => "▁",
            1 => "▂",
            2 => "▃",
            3 => "▄",
            4 => "▅",
            5 => "▆",
            6 => "▇",
            _ => "",
        })
    )
}

fn print_field(field: &Vec<Cell>) {
    print!("{}", "┌");
    print!("{}", "─".repeat(FIELD_SIZE * 2));
    println!("{}", "┐");
    for row in 0..FIELD_SIZE {
        print!("│");
        for col in 0..FIELD_SIZE {
            match coords_to_linear(row, col) {
                Ok(coord) => {
                    let elem = &field[coord];
                    match elem.cell_type {
                        State::Empty => print!("  "),
                        State::Grass => print_grass(elem.age),
                        State::Tree => print_tree(elem.age),
                    }
                }
                Err(_) => todo!(),
            }
        }
        print!("│");
        println!();
    }
    print!("{}", "└");
    print!("{}", "─".repeat(FIELD_SIZE * 2));
    println!("{}", "┘");
}

fn flame_propagation(field: &mut Vec<Cell>) {
    todo!()
}

#[derive(Debug)]
struct Cell {
    pub age: u8,
    pub cell_type: State,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            age: 0,
            cell_type: State::Empty,
        }
    }
}

impl Cell {
    fn step(&mut self) -> Self {
        Self {
            age: self.age + 1,
            cell_type: self.cell_type.clone(),
        }
    }
    fn new() -> Cell {
        Cell {
            age: 0,
            cell_type: State::Empty,
        }
    }
}

#[derive(Debug, Clone)]
enum State {
    Empty,
    Grass,
    Tree,
}
