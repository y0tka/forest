use rand::prelude::*;
use rand_xorshift::XorShiftRng;

use std::iter::repeat_with;

use strum_macros::EnumIter;

#[derive(Debug, Clone)]
pub struct Cell {
    pub age: usize,
    pub cell_type: CellType,
    pub propagation: u8,
}

#[derive(Default)]
pub struct PropagationConfig {
    pub flame_longevity: u8,
    pub propagation_threshold: u8,
}

impl PropagationConfig {
    pub fn new() -> Self {
        Self {
            flame_longevity: 15,
            propagation_threshold: 8,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            age: 0,
            cell_type: CellType::Empty,
            propagation: 1,
        }
    }
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            age: 0,
            cell_type: CellType::Empty,
            propagation: 1,
        }
    }

    pub fn step(&mut self, config: &PropagationConfig) -> Self {
        if self.age > config.flame_longevity.into() && self.cell_type == CellType::Flame {
            return Self {
                age: 0,
                cell_type: CellType::Empty,
                propagation: 0,
            };
        }
        Self {
            age: self.age + 1,
            cell_type: self.cell_type,
            propagation: self.propagation,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, EnumIter, Copy)]
pub enum CellType {
    Empty,
    Grass,
    Tree,
    Flame,
}

pub fn get_field_step(field: &[Cell], config: &PropagationConfig) -> Vec<Cell> {
    let mut return_field = field.to_vec();
    for cell in return_field.iter_mut() {
        *cell = cell.step(config);
    }
    return_field = propagate(&return_field, config);
    return_field
}

pub fn get_empty_field(size: usize) -> Vec<Cell> {
    let field: Vec<Cell> = repeat_with(Cell::new).take(size * size).collect();
    field
}

pub fn get_random_field(
    size: usize,
    grass_count: usize,
    tree_count: usize,
    flame_count: usize,
) -> Vec<Cell> {
    let mut field = get_empty_field(size);
    field = rnd_fill_empty(&field, grass_count, CellType::Grass);
    field = rnd_fill_empty(&field, tree_count, CellType::Tree);
    field = rnd_fill_empty(&field, flame_count, CellType::Flame);
    field
}

pub fn cartesian_to_linear(y: usize, x: usize, field: &Vec<Cell>) -> Result<usize, &'static str> {
    let area = field.len();
    let side = f32::sqrt(area as f32).floor() as usize;
    if y >= side || x >= side {
        return Err("Out of Bounds (C2L)");
    }
    let res = x * side + y;
    Ok(res)
}

pub fn linear_to_cartesian(
    position: usize,
    field: &Vec<Cell>,
) -> Result<(usize, usize), &'static str> {
    let side = f32::sqrt(field.len() as f32).floor() as usize;
    if position >= field.len() {
        return Err("Out of Bounds (L2C)");
    }
    Ok((position % side, position / side))
}

fn rnd_fill_empty(field: &[Cell], count: usize, cell_type: CellType) -> Vec<Cell> {
    let mut rng = XorShiftRng::seed_from_u64(0);
    let mut return_field = field.to_vec();
    let side = f32::sqrt(return_field.len() as f32).floor() as usize;
    let mut to_fill: usize = count;
    while to_fill > 0 {
        let x = rng.gen_range(0..side);
        let y = rng.gen_range(0..side);
        match cartesian_to_linear(x, y, &return_field) {
            Ok(coord) => {
                if CellType::Empty == return_field[coord].cell_type {
                    return_field[coord].cell_type = cell_type;
                    return_field[coord].age = rng.gen_range(0..10);
                    to_fill -= 1;
                }
            }
            Err(_) => panic!("OH SHIT *dies from cringe*"),
        }
    }
    return_field
}

fn propagate(field: &Vec<Cell>, config: &PropagationConfig) -> Vec<Cell> {
    let mut return_field: Vec<Cell> = field.to_vec();
    let mut rng = XorShiftRng::seed_from_u64(0);
    for (index, ele) in field.iter().enumerate() {
        if ele.age < config.propagation_threshold.into() {
            continue;
        }
        if ele.propagation == 1 {
            let x_offset: isize;
            let y_offset: isize;
            if rng.gen::<f32>() >= 0.5 {
                x_offset = if rng.gen::<f32>() >= 0.5 { -1 } else { 1 };
                y_offset = 0;
            } else {
                x_offset = 0;
                y_offset = if rng.gen::<f32>() >= 0.5 { -1 } else { 1 };
            }

            let (mut x, mut y) = linear_to_cartesian(index, field).unwrap();
            match x.checked_add_signed(x_offset) {
                Some(res) => x = res,
                None => continue,
            }
            match y.checked_add_signed(y_offset) {
                Some(res) => y = res,
                None => continue,
            }
            match cartesian_to_linear(x, y, field) {
                Ok(coord) => match ele.cell_type {
                    CellType::Grass | CellType::Tree => {
                        if CellType::Empty == return_field[coord].cell_type {
                            return_field[coord].cell_type = ele.cell_type;
                            return_field[coord].age = 0;
                            return_field[coord].propagation = 1;
                        }
                    }
                    CellType::Flame => match return_field[coord].cell_type {
                        CellType::Grass | CellType::Tree => {
                            return_field[coord].cell_type = ele.cell_type;
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
    }
    return_field
}
