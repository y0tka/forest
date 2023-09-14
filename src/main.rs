use ansi_term::Colour;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::iter::repeat_with;
use std::thread;
use std::time::Duration;

const FIELD_SIZE: usize = 10;
const FIELD_AREA: usize = FIELD_SIZE * FIELD_SIZE;
const GRASS_COUNT: usize = 5;
const TREE_COUNT: usize = 5;
const FRAME_DELAY: u64 = 250;
// const SIMULATION_STEPS: usize = 20;

fn main() {
    let mut field: Vec<Cell> = repeat_with(|| Cell::new()).take(FIELD_AREA).collect();

    // GRASS FILL
    rnd_fill_empty(&mut field, GRASS_COUNT, State::Grass);
    rnd_fill_empty(&mut field, TREE_COUNT, State::Tree);

    // SIMULATION
    loop {
        clear();
        for c in field.iter_mut() {
            *c = c.step();
        }
        field = propagate(&field);
        print_field(&field);
        if field.iter().all(|ele| ele.age > 10) {
            field_stats(&field);
            break;
        }
        // println!("{:?}", &field);
        thread::sleep(Duration::from_millis(FRAME_DELAY));
    }
}

fn clear() {
    print!("{}[2J", 27 as char);
}

fn cartesian_to_linear(y: usize, x: usize, field_size: usize) -> Result<usize, &'static str> {
    let res = x * field_size + y;
    if res < field_size * field_size {
        return Ok(res);
    } else {
        return Err("Out of Bounds (C2L)");
    }
}

fn linear_to_cartesian(position: usize, field: &Vec<Cell>) -> Result<(usize, usize), &'static str> {
    if position >= field.len() {
        return Err("Out of Bounds (L2C)");
    }
    return Ok((position % field.len(), position / field.len()));
}

fn field_stats(field: &Vec<Cell>) -> HashMap<State, u8> {
    let mut res: HashMap<State, u8> = HashMap::new();
    for ele in field.iter() {
        if res.contains_key(&ele.cell_type) {
            res.insert(
                ele.cell_type.clone(),
                res.get(&ele.cell_type).copied().unwrap() + 1,
            );
        } else {
            res.insert(ele.cell_type.clone(), 1);
        }
    }
    return res;
}

fn rnd_fill_empty(field: &mut Vec<Cell>, count: usize, cell_type: State) {
    let mut to_fill: usize = count;
    while to_fill > 0 {
        let x = rand::thread_rng().gen_range(0..FIELD_SIZE);
        let y = rand::thread_rng().gen_range(0..FIELD_SIZE);
        match cartesian_to_linear(x, y, FIELD_SIZE) {
            Ok(coord) => match field[coord].cell_type {
                State::Empty => {
                    field[coord].cell_type = cell_type.clone();
                    field[coord].age = rand::thread_rng().gen_range(0..10);
                    to_fill -= 1;
                }
                _ => (),
            },
            Err(_) => panic!("OH SHIT *dies from cringe*"),
        }
    }
}

fn print_field(field: &Vec<Cell>) {
    print!("{}", "┌");
    print!("{}", "─".repeat(FIELD_SIZE * 2));
    println!("{}", "┐");
    for row in 0..FIELD_SIZE {
        print!("│");
        for col in 0..FIELD_SIZE {
            match cartesian_to_linear(row, col, FIELD_SIZE) {
                Ok(coord) => {
                    field[coord].print();
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

fn propagate(field: &Vec<Cell>) -> Vec<Cell> {
    let mut return_field: Vec<Cell> = field.clone().to_vec();
    for (index, ele) in field.iter().enumerate() {
        if ele.age < 8 {
            continue;
        }
        match ele.propagation {
            1 => {
                let x_p: f32 = thread_rng().gen();
                let y_p: f32 = thread_rng().gen();
                let offset: [isize; 2];
                if x_p > y_p {
                    offset = [0, *vec![-1, 1].choose(&mut rand::thread_rng()).unwrap()];
                } else {
                    offset = [*vec![-1, 1].choose(&mut rand::thread_rng()).unwrap(), 0];
                }
                let (mut x, mut y) = linear_to_cartesian(index, field).unwrap();
                match x.checked_add_signed(offset[0]) {
                    Some(res) => x = res,
                    None => continue,
                }
                match y.checked_add_signed(offset[1]) {
                    Some(res) => y = res,
                    None => continue,
                }
                match cartesian_to_linear(x, y, FIELD_SIZE) {
                    Ok(coord) => match ele.cell_type {
                        State::Grass | State::Tree => match return_field[coord].cell_type {
                            State::Empty => {
                                return_field[coord].cell_type = ele.cell_type.clone();
                                return_field[coord].age = 0;
                                return_field[coord].propagation = 1;
                            }
                            _ => (),
                        },
                        _ => (),
                    },
                    Err(_) => continue,
                }
            }
            _ => (),
        }
    }
    return return_field;
}

#[derive(Debug, Clone)]
struct Cell {
    pub age: u8,
    pub cell_type: State,
    pub propagation: u8,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            age: 0,
            cell_type: State::Empty,
            propagation: 1,
        }
    }
}

impl Cell {
    fn new() -> Cell {
        Cell {
            age: 0,
            cell_type: State::Empty,
            propagation: 1,
        }
    }

    #[allow(unreachable_patterns)]
    fn print(&self) {
        match self.cell_type {
            State::Empty => print!("  "),
            State::Grass => {
                print!(
                    "{} ",
                    Colour::Green.paint(match self.age {
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
            State::Tree => {
                print!(
                    "{} ",
                    Colour::Green.paint(match self.age {
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
            _ => print!("? "),
        }
    }

    fn step(&mut self) -> Self {
        Self {
            age: self.age + 1,
            cell_type: self.cell_type.clone(),
            propagation: self.propagation,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum State {
    Empty,
    Grass,
    Tree,
}
