use ansi_term::Colour;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::iter::repeat_with;
use std::thread;
use std::time::Duration;
use strum::IntoEnumIterator;

const FIELD_SIZE: usize = 10;

const GRASS_COUNT: usize = 8;
const TREE_COUNT: usize = 8;
const FLAME_COUNT: usize = 2;

const FRAME_DELAY: u64 = 300;
const SIMULATION_STEPS: usize = 100;

pub mod types;
use crate::types::*;

fn main() {
    let mut field: Vec<Cell> = get_new_field(FIELD_SIZE);

    field = rnd_fill_empty(&mut field, GRASS_COUNT, CellType::Grass);
    field = rnd_fill_empty(&mut field, TREE_COUNT, CellType::Tree);
    field = rnd_fill_empty(&mut field, FLAME_COUNT, CellType::Flame);

    for _ in 0..SIMULATION_STEPS {
        clear();
        field = get_field_step(&field);
        print_field(&field);

        thread::sleep(Duration::from_millis(FRAME_DELAY));
    }

    println!("{:?}", field_stats(&field));
}

fn clear() {
    print!("{}[2J", 27 as char);
}

fn get_field_step(field: &Vec<Cell>) -> Vec<Cell> {
    let mut return_field = field.clone().to_vec();
    for cell in return_field.iter_mut() {
        *cell = cell.step();
    }
    return_field = propagate(&return_field);
    return return_field;
}

fn get_new_field(size: usize) -> Vec<Cell> {
    let field: Vec<Cell> = repeat_with(|| Cell::new()).take(size * size).collect();
    return field;
}

fn cartesian_to_linear(y: usize, x: usize, field: &Vec<Cell>) -> Result<usize, &'static str> {
    let area = field.len();
    let side = f32::sqrt(area as f32).floor() as usize;
    if y >= side || x >= side {
        return Err("Out of Bounds (C2L)");
    }
    let res = x * side + y;
    return Ok(res);
}

fn linear_to_cartesian(position: usize, field: &Vec<Cell>) -> Result<(usize, usize), &'static str> {
    let side = f32::sqrt(field.len() as f32).floor() as usize;
    if position >= field.len() {
        return Err("Out of Bounds (L2C)");
    }
    return Ok((position % side, position / side));
}

fn field_stats(field: &Vec<Cell>) -> HashMap<CellType, u32> {
    let mut res: HashMap<CellType, u32> = HashMap::new();
    for c in CellType::iter() {
        res.insert(c, 0);
    }
    for ele in field.iter() {
        res.insert(
            ele.cell_type.clone(),
            res.get(&ele.cell_type).copied().unwrap() + 1,
        );
    }
    return res;
}

fn rnd_fill_empty(field: &Vec<Cell>, count: usize, cell_type: CellType) -> Vec<Cell> {
    let mut return_field = field.clone().to_vec();
    let side = f32::sqrt(return_field.len() as f32).floor() as usize;
    let mut to_fill: usize = count;
    while to_fill > 0 {
        let x = thread_rng().gen_range(0..side);
        let y = thread_rng().gen_range(0..side);
        match cartesian_to_linear(x, y, &return_field) {
            Ok(coord) => match return_field[coord].cell_type {
                CellType::Empty => {
                    return_field[coord].cell_type = cell_type.clone();
                    return_field[coord].age = rand::thread_rng().gen_range(0..10);
                    to_fill -= 1;
                }
                _ => (),
            },
            Err(_) => panic!("OH SHIT *dies from cringe*"),
        }
    }
    return return_field;
}

fn print_field(field: &Vec<Cell>) {
    let side = f32::sqrt(field.len() as f32).floor() as usize;
    print!("{}", "┌");
    print!("{}", "─".repeat(side * 2));
    println!("{}", "┐");
    for row in 0..side {
        print!("│");
        for col in 0..side {
            match cartesian_to_linear(row, col, field) {
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
    print!("{}", "─".repeat(side * 2));
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
                if y_p > x_p {
                    offset = [0, *vec![-1, 1].choose(&mut thread_rng()).unwrap()];
                } else {
                    offset = [*vec![-1, 1].choose(&mut thread_rng()).unwrap(), 0];
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
                match cartesian_to_linear(x, y, field) {
                    Ok(coord) => match ele.cell_type {
                        CellType::Grass | CellType::Tree => match return_field[coord].cell_type {
                            CellType::Empty => {
                                return_field[coord].cell_type = ele.cell_type.clone();
                                return_field[coord].age = 0;
                                return_field[coord].propagation = 1;
                            }
                            _ => (),
                        },
                        CellType::Flame => match return_field[coord].cell_type {
                            CellType::Grass | CellType::Tree => {
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
