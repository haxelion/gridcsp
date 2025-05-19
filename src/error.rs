use std::error::Error;
use std::fmt;

use splr::SolverError;

#[derive(Debug, PartialEq)]
pub enum GridCspError {
    CellOutOfBound(usize, usize),
    ColumnOutOfBound(usize),
    RowOutOfBound(usize),
    SquareOutOfBound(usize, usize),
    ConstrainedGroupTooSmall,
    ConstrainedGroupTooBig,
    UnsupportedSudokuSize,
    TooManyVariables,
    SolverError(SolverError),
    NoSolution,
    UnexpectedSolution,
    SolutionNotUnique,
}

impl From<SolverError> for GridCspError {
    fn from(err: SolverError) -> Self {
        GridCspError::SolverError(err)
    }
}

impl fmt::Display for GridCspError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridCspError::CellOutOfBound(x, y) => write!(f, "Cell ({x}, {y}) is out of bound"),
            GridCspError::ColumnOutOfBound(x) => write!(f, "Column {x} is out of bound"),
            GridCspError::RowOutOfBound(y) => write!(f, "Row {y} is out of bound"),
            GridCspError::SquareOutOfBound(x, y) => {
                write!(f, "Square corner ({x}, {y}) is out of bound")
            }
            GridCspError::ConstrainedGroupTooSmall => write!(f, "Constrained group is too small"),
            GridCspError::ConstrainedGroupTooBig => write!(f, "Constrained group is too big"),
            GridCspError::UnsupportedSudokuSize => write!(f, "Sudoku size is not a perfect square"),
            GridCspError::TooManyVariables => write!(f, "Problem has too many variables"),
            GridCspError::SolverError(err) => write!(f, "Solver error: {err}"),
            GridCspError::NoSolution => write!(f, "Problem has no solution"),
            GridCspError::UnexpectedSolution => write!(f, "Solver produced unexpected solution"),
            GridCspError::SolutionNotUnique => write!(f, "Problem has multiple solutions"),
        }
    }
}

impl Error for GridCspError {}
