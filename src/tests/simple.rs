use crate::error::GridCspError;
use crate::model::*;
use crate::sat::GridCspSolver;

#[test]
fn simple_2x2() -> Result<(), GridCspError> {
    let problem: GenericProblem = serde_json::from_str(
        r#"{
            "grid": {"width": 2, "height": 2, "number_max": 2},
            "constraints": [
                {
                    "constraint": "Unique",
                    "group": { "Row": 0 }
                },
                {
                    "constraint": "Unique",
                    "group": { "Row": 1 }
                },
                {
                    "constraint": "Unique",
                    "group": { "Column": 0 }
                },
                {
                    "constraint": "Unique",
                    "group": { "Column": 1 }
                },
                {
                    "constraint": { "Equal": 2 },
                    "group": { "List": [{ "x": 0, "y": 0 }] }
                }
            ]
        }"#,
    )
    .unwrap();
    let mut csp = GridCspSolver::try_from(problem)?;
    let solution = csp.solve_unique()?;
    assert_eq!(solution, vec![vec![2, 1], vec![1, 2]]);
    Ok(())
}

#[test]
fn simple_3x3() -> Result<(), GridCspError> {
    let size = 3;
    let mut groups = Vec::<ConstrainedGroup>::new();
    for i in 0..size {
        groups.push(Constraint::Unique.over(CellGroup::Row(i)));
        groups.push(Constraint::Unique.over(CellGroup::Column(i)));
        groups.push(
            CellGroup::List(vec![Cell::new(i, i)]).constrainted_by(Constraint::Equal(i as u64 + 1)),
        );
    }
    let problem = GenericProblem {
        grid: GridDimensions::new(3, 3, 3),
        constraints: groups,
    };
    let mut csp = GridCspSolver::try_from(problem)?;
    let solution = csp.solve_unique()?;
    assert_eq!(solution, vec![vec![1, 3, 2], vec![3, 2, 1], vec![2, 1, 3]]);
    Ok(())
}

#[test]
fn underconstrained_3x3() -> Result<(), GridCspError> {
    let size = 3;
    let mut groups = Vec::<ConstrainedGroup>::new();
    for i in 0..size {
        groups.push(Constraint::Unique.over(CellGroup::Row(i)));
        groups.push(Constraint::Unique.over(CellGroup::Column(i)));
        groups.push(CellGroup::List(vec![Cell::new(i, i)]).constrainted_by(Constraint::Equal(1)));
    }
    let problem = GenericProblem {
        grid: GridDimensions::new(3, 3, 3),
        constraints: groups,
    };
    let mut csp = GridCspSolver::try_from(problem)?;
    assert_eq!(csp.solve_unique(), Err(GridCspError::SolutionNotUnique));
    Ok(())
}
