use serde::{Deserialize, Serialize};

use crate::error::GridCspError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Constraint {
    Add(u64),
    Div(u64),
    Equal(u64),
    Mul(u64),
    Sub(u64),
    Unique,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CellGroup {
    Column(usize),
    Row(usize),
    Square {
        x: usize,
        y: usize,
        height: usize,
        width: usize,
    },
    List(Vec<Cell>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConstrainedGroup {
    pub constraint: Constraint,
    pub group: CellGroup,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Problem {
    pub grid_size: usize,
    pub constraints: Vec<ConstrainedGroup>,
}

impl Constraint {
    pub fn over(self, group: CellGroup) -> ConstrainedGroup {
        ConstrainedGroup::new(self, group)
    }
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Self {
        Cell { x, y }
    }

    fn validate(&self, grid_size: usize) -> Result<(), GridCspError> {
        if self.x >= grid_size || self.y >= grid_size {
            Err(GridCspError::CellOutOfBound(self.x, self.y))
        } else {
            Ok(())
        }
    }
}

impl CellGroup {
    pub fn to_cells(&self, grid_size: usize) -> Vec<Cell> {
        match self {
            CellGroup::Column(x) => (0..grid_size).map(|y| Cell::new(*x, y)).collect(),
            CellGroup::Row(y) => (0..grid_size).map(|x| Cell::new(x, *y)).collect(),
            CellGroup::Square {
                x,
                y,
                height,
                width,
            } => (*x..x + width)
                .flat_map(|x| (*y..y + height).map(move |y| Cell::new(x, y)))
                .collect(),
            CellGroup::List(cells) => cells.clone(),
        }
    }

    pub fn constrainted_by(self, constrain: Constraint) -> ConstrainedGroup {
        ConstrainedGroup::new(constrain, self)
    }

    pub fn size(&self, grid_size: usize) -> usize {
        match self {
            CellGroup::Column(_) => grid_size,
            CellGroup::Row(_) => grid_size,
            CellGroup::Square { height, width, .. } => height * width,
            CellGroup::List(cells) => cells.len(),
        }
    }

    fn validate(&self, grid_size: usize) -> Result<(), GridCspError> {
        match self {
            CellGroup::Column(x) => {
                if *x >= grid_size {
                    return Err(GridCspError::ColumnOutOfBound(*x));
                }
            }
            CellGroup::Row(y) => {
                if *y >= grid_size {
                    return Err(GridCspError::RowOutOfBound(*y));
                }
            }
            CellGroup::Square {
                x,
                y,
                height,
                width,
            } => {
                if *x >= grid_size || *y >= grid_size {
                    return Err(GridCspError::SquareOutOfBound(*x, *y));
                } else if x + width > grid_size || y + height > grid_size {
                    return Err(GridCspError::SquareOutOfBound(x + width, y + height));
                }
            }
            CellGroup::List(cells) => {
                cells
                    .iter()
                    .fold(Ok(()), |acc, c| acc.and(c.validate(grid_size)))?;
            }
        }
        Ok(())
    }
}

impl ConstrainedGroup {
    pub fn new(constraint: Constraint, group: CellGroup) -> Self {
        ConstrainedGroup { constraint, group }
    }

    fn validate(&self, grid_size: usize) -> Result<(), GridCspError> {
        self.group.validate(grid_size)?;
        match self.constraint {
            Constraint::Add(_) | Constraint::Div(_) | Constraint::Mul(_) | Constraint::Sub(_) => {
                if self.group.size(grid_size) < 2 {
                    return Err(GridCspError::ConstrainedGroupTooSmall);
                }
            }
            Constraint::Equal(_) => {}
            Constraint::Unique => {
                if self.group.size(grid_size) > grid_size {
                    return Err(GridCspError::ConstrainedGroupTooBig);
                }
            }
        }
        Ok(())
    }
}

impl Problem {
    pub fn new(grid_size: usize) -> Problem {
        Problem {
            grid_size,
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(&mut self, constraint: ConstrainedGroup) {
        self.constraints.push(constraint)
    }

    pub fn validate(&self) -> Result<(), GridCspError> {
        self.constraints
            .iter()
            .fold(Ok(()), |acc, c| acc.and(c.validate(self.grid_size)))
    }
}
