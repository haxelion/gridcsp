use crate::error::GridCspError;
use crate::model::*;
use crate::sat::GridCspSolver;

#[test]
fn kenken_3x3() -> Result<(), GridCspError> {
    let problem: KenKenProblem = serde_json::from_str(
        r#"{
            "grid_size": 3,
            "constraints": [
                {
                    "constraint": { "Mul": 6 },
                    "group": { "List": [{ "x": 0, "y": 0 }, { "x": 1, "y": 0 }] }
                },
                {
                    "constraint": { "Mul": 2 },
                    "group": { "List": [{ "x": 2, "y": 0 }, { "x": 2, "y": 1 }] }
                },
                {
                    "constraint": { "Mul": 6 },
                    "group": { "List": [{ "x": 0, "y": 1 }, { "x": 1, "y": 1 }, { "x": 0, "y": 2 }] }
                },
                {
                    "constraint": { "Mul": 3 },
                    "group": { "List": [{ "x": 1, "y": 2 }, { "x": 2, "y": 2 }] }
                },
                {
                    "constraint": { "Equal": 2 },
                    "group": { "List": [{ "x": 0, "y": 2 }] }
                }
            ]
        }"#,
    )
    .unwrap();
    let mut csp = GridCspSolver::try_from(GenericProblem::from(problem))?;
    let solution = csp.solve_unique()?;
    assert_eq!(solution, vec![vec![3, 1, 2], vec![2, 3, 1], vec![1, 2, 3]]);
    Ok(())
}

#[test]
fn kenken_4x4() -> Result<(), GridCspError> {
    let problem: KenKenProblem = serde_json::from_str(
        r#"{
            "grid_size": 4,
            "constraints": [
                {
                    "constraint": { "Mul": 12 },
                    "group": { "List": [{ "x": 0, "y": 0 }, { "x": 1, "y": 0 }] }
                },
                {
                    "constraint": { "Mul": 2 },
                    "group": { "List": [{ "x": 2, "y": 0 }, { "x": 3, "y": 0 }, { "x": 3, "y": 1 }] }
                },
                {
                    "constraint": { "Div": 4 },
                    "group": { "List": [{ "x": 0, "y": 1 }, { "x": 0, "y": 2 }] }
                },
                {
                    "constraint": { "Mul": 6 },
                    "group": { "List": [{ "x": 1, "y": 1 }, { "x": 1, "y": 2 }] }
                },
                {
                    "constraint": { "Mul": 72 },
                    "group": { "List": [{ "x": 2, "y": 1 }, { "x": 2, "y": 2 }, { "x": 2, "y": 3 }, { "x": 3, "y": 2 }] }
                },
                {
                    "constraint": { "Div": 2 },
                    "group": { "List": [{ "x": 0, "y": 3 }, { "x": 1, "y": 3 }] }
                },
                {
                    "constraint": { "Equal": 4 },
                    "group": { "List": [{ "x": 3, "y": 3 }] }
                }
            ]
        }"#,
    )
    .unwrap();
    let mut csp = GridCspSolver::try_from(GenericProblem::from(problem))?;
    let solution = csp.solve_unique()?;
    assert_eq!(
        solution,
        vec![
            vec![3, 4, 1, 2],
            vec![4, 3, 2, 1],
            vec![1, 2, 4, 3],
            vec![2, 1, 3, 4]
        ]
    );
    Ok(())
}

#[test]
fn kenken_5x5() -> Result<(), GridCspError> {
    let problem: KenKenProblem = serde_json::from_str(
        r#"{
            "grid_size": 5,
            "constraints": [
                {
                    "constraint": { "Sub": 2 },
                    "group": { "List": [{ "x": 0, "y": 0 }, { "x": 1, "y": 0 }] }
                },
                {
                    "constraint": { "Add": 5 },
                    "group": { "List": [{ "x": 2, "y": 0 }, { "x": 2, "y": 1 }] }
                },
                {
                    "constraint": { "Add": 6 },
                    "group": { "List": [{ "x": 3, "y": 0 }, { "x": 3, "y": 1 }] }
                },
                {
                    "constraint": { "Add": 9 },
                    "group": { "List": [{ "x": 4, "y": 0 }, { "x": 4, "y": 1 }, { "x": 4, "y": 2 }] }
                },
                {
                    "constraint": { "Sub": 3 },
                    "group": { "List": [{ "x": 0, "y": 1 }, { "x": 0, "y": 2 }] }
                },
                {
                    "constraint": { "Sub": 2 },
                    "group": { "List": [{ "x": 1, "y": 1 }, { "x": 1, "y": 2 }] }
                },
                {
                    "constraint": { "Add": 8 },
                    "group": { "List": [{ "x": 2, "y": 2 }, { "x": 2, "y": 3 }] }
                },
                {
                    "constraint": { "Equal": 3 },
                    "group": { "List": [{ "x": 3, "y": 2 }] }
                },
                {
                    "constraint": { "Sub": 2 },
                    "group": { "List": [{ "x": 0, "y": 3 }, { "x": 0, "y": 4 }] }
                },
                {
                    "constraint": { "Add": 9 },
                    "group": { "List": [{ "x": 1, "y": 3 }, { "x": 1, "y": 4 }] }
                },
                {
                    "constraint": { "Equal": 2 },
                    "group": { "List": [{ "x": 2, "y": 4 }] }
                },
                {
                    "constraint": { "Sub": 2 },
                    "group": { "List": [{ "x": 3, "y": 3 }, { "x": 3, "y": 4 }] }
                },
                {
                    "constraint": { "Sub": 4 },
                    "group": { "List": [{ "x": 4, "y": 3 }, { "x": 4, "y": 4 }] }
                }
            ]
        }"#,
    )
    .unwrap();
    let mut csp = GridCspSolver::try_from(GenericProblem::from(problem))?;
    let solution = csp.solve_unique()?;
    assert_eq!(
        solution,
        vec![
            vec![4, 5, 2, 1, 3],
            vec![2, 3, 1, 4, 5],
            vec![1, 4, 5, 3, 2],
            vec![5, 1, 3, 2, 4],
            vec![3, 2, 4, 5, 1]
        ]
    );
    Ok(())
}
