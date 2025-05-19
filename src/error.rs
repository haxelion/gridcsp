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
            GridCspError::CellOutOfBound(x, y) => write!(f, "Cell ({}, {}) is out of bound", x, y),
            GridCspError::ColumnOutOfBound(x) => write!(f, "Column {} is out of bound", x),
            GridCspError::RowOutOfBound(y) => write!(f, "Row {} is out of bound", y),
            GridCspError::SquareOutOfBound(x, y) => {
                write!(f, "Square corner ({}, {}) is out of bound", x, y)
            }
            GridCspError::ConstrainedGroupTooSmall => write!(f, "Constrained group is too small"),
            GridCspError::ConstrainedGroupTooBig => write!(f, "Constrained group is too big"),
            GridCspError::TooManyVariables => write!(f, "Problem has too many variables"),
            GridCspError::SolverError(err) => write!(f, "Solver error: {}", err),
            GridCspError::NoSolution => write!(f, "Problem has no solution"),
            GridCspError::UnexpectedSolution => write!(f, "Solver produced unexpected solution"),
            GridCspError::SolutionNotUnique => write!(f, "Problem has multiple solutions"),
        }
    }
}

impl Error for GridCspError {}
