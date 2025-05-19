mod constraints;
pub mod error;
pub mod model;
pub mod sat;

#[cfg(test)]
mod tests {

    use crate::{error::GridCspError, model::*, sat::GridCspSolver};

    #[test]
    fn simple_2x2() -> Result<(), GridCspError> {
        let problem = Problem {
            grid_size: 2,
            constraints: vec![
                Constraint::Unique.over(CellGroup::Row(0)),
                Constraint::Unique.over(CellGroup::Row(1)),
                Constraint::Unique.over(CellGroup::Column(0)),
                Constraint::Unique.over(CellGroup::Column(1)),
                CellGroup::List(vec![Cell::new(0, 0)]).constrainted_by(Constraint::Equal(2)),
            ],
        };
        let mut csp = GridCspSolver::try_from(problem)?;
        assert!(csp.solve_unique().is_ok());
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
                CellGroup::List(vec![Cell::new(i, i)])
                    .constrainted_by(Constraint::Equal(i as u64 + 1)),
            );
        }
        let problem = Problem {
            grid_size: size,
            constraints: groups,
        };
        let mut csp = GridCspSolver::try_from(problem)?;
        assert!(csp.solve_unique().is_ok());
        Ok(())
    }

    #[test]
    fn underconstrained_3x3() -> Result<(), GridCspError> {
        let size = 3;
        let mut groups = Vec::<ConstrainedGroup>::new();
        for i in 0..size {
            groups.push(Constraint::Unique.over(CellGroup::Row(i)));
            groups.push(Constraint::Unique.over(CellGroup::Column(i)));
            groups
                .push(CellGroup::List(vec![Cell::new(i, i)]).constrainted_by(Constraint::Equal(1)));
        }
        let problem = Problem {
            grid_size: size,
            constraints: groups,
        };
        let mut csp = GridCspSolver::try_from(problem)?;
        assert!(csp.solve().is_ok());
        assert_eq!(csp.solve_unique(), Err(GridCspError::SolutionNotUnique));
        Ok(())
    }

    #[test]
    fn kenken_3x3() -> Result<(), GridCspError> {
        let size = 3;
        let mut groups = Vec::<ConstrainedGroup>::new();
        for i in 0..size {
            groups.push(Constraint::Unique.over(CellGroup::Row(i)));
            groups.push(Constraint::Unique.over(CellGroup::Column(i)));
        }

        groups
            .push(Constraint::Mul(6).over(CellGroup::List(vec![Cell::new(0, 0), Cell::new(1, 0)])));
        groups
            .push(Constraint::Mul(2).over(CellGroup::List(vec![Cell::new(2, 0), Cell::new(2, 1)])));
        groups.push(Constraint::Mul(6).over(CellGroup::List(vec![
            Cell::new(0, 1),
            Cell::new(1, 1),
            Cell::new(0, 2),
        ])));
        groups
            .push(Constraint::Mul(3).over(CellGroup::List(vec![Cell::new(1, 2), Cell::new(2, 2)])));
        groups.push(Constraint::Equal(2).over(CellGroup::List(vec![Cell::new(0, 2)])));

        let problem = Problem {
            grid_size: size,
            constraints: groups,
        };
        let mut csp = GridCspSolver::try_from(problem)?;
        println!("{:?}", csp.solve_unique().unwrap());
        //assert!(csp.solve_unique().is_ok());
        Ok(())
    }

    #[test]
    fn sudoku_9x9() -> Result<(), GridCspError> {
        let size = 9;
        let mut groups = Vec::<ConstrainedGroup>::new();
        for i in 0..size {
            groups.push(Constraint::Unique.over(CellGroup::Row(i)));
            groups.push(Constraint::Unique.over(CellGroup::Column(i)));
        }
        for x in (0..9).step_by(3) {
            for y in (0..9).step_by(3) {
                groups.push(Constraint::Unique.over(CellGroup::Square {
                    x,
                    y,
                    height: 3,
                    width: 3,
                }));
            }
        }
        groups.push(
            CellGroup::List(vec![Cell::new(5, 0), Cell::new(6, 4)])
                .constrainted_by(Constraint::Equal(1)),
        );
        groups.push(
            CellGroup::List(vec![Cell::new(1, 5), Cell::new(3, 8)])
                .constrainted_by(Constraint::Equal(2)),
        );
        groups.push(
            CellGroup::List(vec![Cell::new(2, 7), Cell::new(4, 5), Cell::new(8, 1)])
                .constrainted_by(Constraint::Equal(3)),
        );
        groups.push(
            CellGroup::List(vec![Cell::new(3, 7), Cell::new(7, 1)])
                .constrainted_by(Constraint::Equal(4)),
        );
        groups.push(
            CellGroup::List(vec![Cell::new(0, 2), Cell::new(8, 6)])
                .constrainted_by(Constraint::Equal(5)),
        );
        groups.push(
            CellGroup::List(vec![Cell::new(0, 6), Cell::new(6, 8)])
                .constrainted_by(Constraint::Equal(6)),
        );
        groups.push(
            CellGroup::List(vec![Cell::new(4, 3), Cell::new(7, 6)])
                .constrainted_by(Constraint::Equal(7)),
        );
        groups.push(
            CellGroup::List(vec![Cell::new(3, 0), Cell::new(6, 3)])
                .constrainted_by(Constraint::Equal(8)),
        );
        let problem = Problem {
            grid_size: size,
            constraints: groups,
        };
        let mut csp = GridCspSolver::try_from(problem)?;
        assert!(csp.solve_unique().is_ok());
        Ok(())
    }
}
