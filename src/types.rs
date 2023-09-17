use crate::Colour;
#[derive(Debug, Clone)]
pub struct Cell {
    pub age: u8,
    pub cell_type: CellType,
    pub propagation: u8,
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

    #[allow(unreachable_patterns)]
    pub fn print(&self) {
        match self.cell_type {
            CellType::Empty => print!("  "),
            CellType::Grass => {
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
            CellType::Tree => {
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
            CellType::Flame => {
                print!(
                    "{} ",
                    Colour::Red.paint(match self.age {
                        0 => "▁",
                        1 => "▂",
                        2 => "▃",
                        3 => "▄",
                        4 => "▅",
                        5 => "▆",
                        6 => "▇",
                        _ => "",
                    })
                )
            }
            _ => print!("? "),
        }
    }

    pub fn step(&mut self) -> Self {
        match self.cell_type {
            CellType::Flame => {
                if self.age > 15 {
                    return Self {
                        age: 0,
                        cell_type: CellType::Empty,
                        propagation: 0,
                    };
                }
                return Self {
                    age: self.age + 1,
                    cell_type: self.cell_type.clone(),
                    propagation: self.propagation,
                };
            }
            _ => Self {
                age: self.age + 1,
                cell_type: self.cell_type.clone(),
                propagation: self.propagation,
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CellType {
    Empty,
    Grass,
    Tree,
    Flame,
}
