use crate::error::GridCspError;
use crate::model::{GenericProblem, MagicSquareProblem};
use crate::sat::GridCspSolver;

#[test]
fn magic_square_2x2() -> Result<(), GridCspError> {
    let problem: MagicSquareProblem = serde_json::from_str(
        r#"{
            "grid_size": 2,
            "constraints": []
        }"#,
    )
    .unwrap();
    let mut csp = GridCspSolver::try_from(GenericProblem::from(problem))?;
    assert_eq!(csp.solve(), Err(GridCspError::NoSolution));
    Ok(())
}

#[test]
fn magic_square_3x3() -> Result<(), GridCspError> {
    let problem: MagicSquareProblem = serde_json::from_str(
        r#"{
            "grid_size": 3,
            "constraints": [
                {
                    "constraint": { "Equal": 4 },
                    "group": { "List": [{ "x": 0, "y": 0 }] }
                },
                {
                    "constraint": { "Equal": 2 },
                    "group": { "List": [{ "x": 2, "y": 0 }] }
                }
            ]
        }"#,
    )
    .unwrap();
    let mut csp = GridCspSolver::try_from(GenericProblem::from(problem))?;
    let solution = csp.solve_unique()?;
    assert_eq!(solution, vec![vec![4, 3, 8], vec![9, 5, 1], vec![2, 7, 6]]);
    Ok(())
}

#[test]
fn magic_square_4x4() -> Result<(), GridCspError> {
    let problem: MagicSquareProblem = serde_json::from_str(
        r#"{
            "grid_size": 4,
            "constraints": []
        }"#,
    )
    .unwrap();
    let mut csp = GridCspSolver::try_from(GenericProblem::from(problem))?;
    csp.solve()?;
    Ok(())
}
