use ansi_term::Colour;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::iter::repeat_with;
use std::thread;
use std::time::Duration;

const FIELD_SIZE: usize = 10;
const FIELD_AREA: usize = FIELD_SIZE * FIELD_SIZE;
const GRASS_COUNT: usize = 0;
const TREE_COUNT: usize = 0;
const FLAME_COUNT: usize = 0;
const FRAME_DELAY: u64 = 250;
// const SIMULATION_STEPS: usize = 20;

pub mod types;
use crate::types::*;

fn main() {
    let mut field: Vec<Cell> = repeat_with(|| Cell::new()).take(FIELD_AREA).collect();

    // FIELD FILL
    rnd_fill_empty(&mut field, GRASS_COUNT, CellType::Grass);
    rnd_fill_empty(&mut field, TREE_COUNT, CellType::Tree);
    rnd_fill_empty(&mut field, FLAME_COUNT, CellType::Flame);

    // field[45].cell_type = CellType::Tree;
    // field[45].age = 1;
    // field[45].propagation = 1;

    // SIMULATION
    loop {
        clear();
        for c in field.iter_mut() {
            *c = c.step();
        }
        field = propagate(&field);
        print_field(&field);
        if field.iter().all(|ele| ele.age > 10 || ele.age == 0) {
            println!("{:?}", field_stats(&field));
            break;
        }
        // println!("{:?}", &field);
        thread::sleep(Duration::from_millis(FRAME_DELAY));
    }
}

fn clear() {
    print!("{}[2J", 27 as char);
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

fn field_stats(field: &Vec<Cell>) -> HashMap<CellType, u8> {
    let mut res: HashMap<CellType, u8> = HashMap::new();
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

fn rnd_fill_empty(field: &mut Vec<Cell>, count: usize, cell_type: CellType) {
    let mut to_fill: usize = count;
    while to_fill > 0 {
        let x = rand::thread_rng().gen_range(0..FIELD_SIZE);
        let y = rand::thread_rng().gen_range(0..FIELD_SIZE);
        match cartesian_to_linear(x, y, field) {
            Ok(coord) => match field[coord].cell_type {
                CellType::Empty => {
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
                if y_p > x_p {
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
