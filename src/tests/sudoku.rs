use crate::error::GridCspError;
use crate::model::*;
use crate::sat::GridCspSolver;

#[test]
fn sudoku_9x9() -> Result<(), GridCspError> {
    let problem: SudokuProblem = serde_json::from_str(
        r#"{
            "grid_size": 9,
            "constraints": [
                {
                    "constraint": { "Equal": 1 },
                    "group": { "List": [{ "x": 5, "y": 0 }, { "x": 6, "y": 4 }] }
                },
                {
                    "constraint": { "Equal": 2 },
                    "group": { "List": [{ "x": 1, "y": 5 }, { "x": 3, "y": 8 }] }
                },
                {
                    "constraint": { "Equal": 3 },
                    "group": { "List": [{ "x": 2, "y": 7 }, { "x": 4, "y": 5 }, { "x": 8, "y": 1 }] }
                },
                {
                    "constraint": { "Equal": 4 },
                    "group": { "List": [{ "x": 3, "y": 7 }, { "x": 7, "y": 1 }] }
                },
                {
                    "constraint": { "Equal": 5 },
                    "group": { "List": [{ "x": 0, "y": 2 }, { "x": 8, "y": 6 }] }
                },
                {
                    "constraint": { "Equal": 6 },
                    "group": { "List": [{ "x": 0, "y": 6 }, { "x": 6, "y": 8 }] }
                },
                {
                    "constraint": { "Equal": 7 },
                    "group": { "List": [{ "x": 4, "y": 3 }, { "x": 7, "y": 6 }] }
                },
                {
                    "constraint": { "Equal": 8 },
                    "group": { "List": [{ "x": 3, "y": 0 }, { "x": 6, "y": 3 }] }
                }
            ]
        }"#,
    )
    .unwrap();
    let mut csp = GridCspSolver::try_from(GenericProblem::try_from(problem)?)?;
    let solution = csp.solve_unique()?;
    assert_eq!(
        solution,
        vec![
            vec![2, 1, 5, 3, 4, 7, 6, 8, 9],
            vec![3, 8, 9, 1, 6, 2, 4, 5, 7],
            vec![7, 6, 4, 5, 9, 8, 2, 3, 1],
            vec![8, 7, 3, 6, 5, 1, 9, 4, 2],
            vec![4, 9, 2, 7, 8, 3, 1, 6, 5],
            vec![1, 5, 6, 4, 2, 9, 8, 7, 3],
            vec![5, 2, 7, 8, 1, 4, 3, 9, 6],
            vec![6, 4, 1, 9, 3, 5, 7, 2, 8],
            vec![9, 3, 8, 2, 7, 6, 5, 1, 4]
        ]
    );
    Ok(())
}
