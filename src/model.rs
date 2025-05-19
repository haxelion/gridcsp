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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridDimensions {
    pub width: usize,
    pub height: usize,
    pub number_max: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GenericProblem {
    pub grid: GridDimensions,
    pub constraints: Vec<ConstrainedGroup>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SudokuProblem {
    pub grid_size: usize,
    pub constraints: Vec<ConstrainedGroup>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KenKenProblem {
    pub grid_size: usize,
    pub constraints: Vec<ConstrainedGroup>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MagicSquareProblem {
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

    fn validate(&self, grid: GridDimensions) -> Result<(), GridCspError> {
        if self.x >= grid.width || self.y >= grid.height {
            Err(GridCspError::CellOutOfBound(self.x, self.y))
        } else {
            Ok(())
        }
    }
}

impl CellGroup {
    pub fn to_cells(&self, grid: GridDimensions) -> Vec<Cell> {
        match self {
            CellGroup::Column(x) => (0..grid.height).map(|y| Cell::new(*x, y)).collect(),
            CellGroup::Row(y) => (0..grid.width).map(|x| Cell::new(x, *y)).collect(),
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

    pub fn size(&self, grid: GridDimensions) -> usize {
        match self {
            CellGroup::Column(_) => grid.height,
            CellGroup::Row(_) => grid.width,
            CellGroup::Square { height, width, .. } => height * width,
            CellGroup::List(cells) => cells.len(),
        }
    }

    fn validate(&self, grid: GridDimensions) -> Result<(), GridCspError> {
        match self {
            CellGroup::Column(x) => {
                if *x >= grid.width {
                    return Err(GridCspError::ColumnOutOfBound(*x));
                }
            }
            CellGroup::Row(y) => {
                if *y >= grid.height {
                    return Err(GridCspError::RowOutOfBound(*y));
                }
            }
            CellGroup::Square {
                x,
                y,
                height,
                width,
            } => {
                if *x >= grid.width || *y >= grid.height {
                    return Err(GridCspError::SquareOutOfBound(*x, *y));
                } else if x + width > grid.width || y + height > grid.height {
                    return Err(GridCspError::SquareOutOfBound(x + width, y + height));
                }
            }
            CellGroup::List(cells) => {
                cells
                    .iter()
                    .fold(Ok(()), |acc, c| acc.and(c.validate(grid)))?;
            }
        }
        Ok(())
    }
}

impl ConstrainedGroup {
    pub fn new(constraint: Constraint, group: CellGroup) -> Self {
        ConstrainedGroup { constraint, group }
    }

    fn validate(&self, grid: GridDimensions) -> Result<(), GridCspError> {
        self.group.validate(grid)?;
        match self.constraint {
            Constraint::Add(_) | Constraint::Div(_) | Constraint::Mul(_) | Constraint::Sub(_) => {
                if self.group.size(grid) < 2 {
                    return Err(GridCspError::ConstrainedGroupTooSmall);
                }
            }
            Constraint::Equal(_) => {}
            Constraint::Unique => {
                if self.group.size(grid) as u64 > grid.number_max {
                    return Err(GridCspError::ConstrainedGroupTooBig);
                }
            }
        }
        Ok(())
    }
}

impl GridDimensions {
    pub fn new(width: usize, height: usize, number_max: u64) -> Self {
        GridDimensions {
            width,
            height,
            number_max,
        }
    }
}

impl GenericProblem {
    pub fn new(grid: GridDimensions) -> Self {
        GenericProblem {
            grid,
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(&mut self, constraint: ConstrainedGroup) {
        self.constraints.push(constraint)
    }

    pub fn validate(&self) -> Result<(), GridCspError> {
        self.constraints
            .iter()
            .fold(Ok(()), |acc, c| acc.and(c.validate(self.grid)))
    }
}

impl SudokuProblem {
    pub fn new(grid_size: usize) -> Self {
        SudokuProblem {
            grid_size,
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(&mut self, constraint: ConstrainedGroup) {
        self.constraints.push(constraint)
    }
}

impl TryFrom<SudokuProblem> for GenericProblem {
    type Error = GridCspError;

    fn try_from(mut problem: SudokuProblem) -> Result<Self, Self::Error> {
        let root = problem.grid_size.isqrt();
        if root * root != problem.grid_size {
            return Err(GridCspError::UnsupportedSudokuSize);
        }

        let mut generic = GenericProblem::new(GridDimensions::new(
            problem.grid_size,
            problem.grid_size,
            problem.grid_size as u64,
        ));
        for i in 0..problem.grid_size {
            generic.add_constraint(Constraint::Unique.over(CellGroup::Row(i)));
            generic.add_constraint(Constraint::Unique.over(CellGroup::Column(i)));
        }
        for x in (0..problem.grid_size).step_by(root) {
            for y in (0..problem.grid_size).step_by(root) {
                generic.add_constraint(Constraint::Unique.over(CellGroup::Square {
                    x,
                    y,
                    height: root,
                    width: root,
                }));
            }
        }
        problem
            .constraints
            .drain(..)
            .for_each(|cst| generic.add_constraint(cst));

        Ok(generic)
    }
}

impl KenKenProblem {
    pub fn new(grid_size: usize) -> Self {
        KenKenProblem {
            grid_size,
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(&mut self, constraint: ConstrainedGroup) {
        self.constraints.push(constraint)
    }
}

impl From<KenKenProblem> for GenericProblem {
    fn from(mut problem: KenKenProblem) -> Self {
        let mut generic = GenericProblem::new(GridDimensions::new(
            problem.grid_size,
            problem.grid_size,
            problem.grid_size as u64,
        ));

        for i in 0..problem.grid_size {
            generic.add_constraint(Constraint::Unique.over(CellGroup::Row(i)));
            generic.add_constraint(Constraint::Unique.over(CellGroup::Column(i)));
        }
        problem
            .constraints
            .drain(..)
            .for_each(|cst| generic.add_constraint(cst));

        generic
    }
}

impl MagicSquareProblem {
    pub fn new(grid_size: usize) -> Self {
        MagicSquareProblem {
            grid_size,
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(&mut self, constraint: ConstrainedGroup) {
        self.constraints.push(constraint)
    }
}

impl From<MagicSquareProblem> for GenericProblem {
    fn from(mut problem: MagicSquareProblem) -> Self {
        let number_max = problem.grid_size as u64 * problem.grid_size as u64;
        let magic = problem.grid_size as u64 * (number_max + 1) / 2;
        let mut generic = GenericProblem::new(GridDimensions::new(
            problem.grid_size,
            problem.grid_size,
            number_max,
        ));

        generic.add_constraint(Constraint::Unique.over(CellGroup::Square {
            x: 0,
            y: 0,
            height: problem.grid_size,
            width: problem.grid_size,
        }));
        for i in 0..problem.grid_size {
            generic.add_constraint(Constraint::Add(magic).over(CellGroup::Row(i)));
            generic.add_constraint(Constraint::Add(magic).over(CellGroup::Column(i)));
        }
        generic.add_constraint(Constraint::Add(magic).over(CellGroup::List(
            (0..problem.grid_size).map(|i| Cell::new(i, i)).collect(),
        )));
        generic.add_constraint(
            Constraint::Add(magic).over(CellGroup::List(
                (0..problem.grid_size)
                    .map(|i| Cell::new(i, problem.grid_size - i - 1))
                    .collect(),
            )),
        );
        problem
            .constraints
            .drain(..)
            .for_each(|cst| generic.add_constraint(cst));

        generic
    }
}
