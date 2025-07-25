use crate::constraints::*;
use crate::error::GridCspError;
use crate::model::{Cell, GenericProblem, GridDimensions};

use std::borrow::Borrow;

use itertools::Itertools;
use splr::{Certificate, Config, SolveIF, Solver};

pub struct GridCspSolver {
    var_count: i32,
    grid_vars: Vec<Vec<Vec<i32>>>,
    clauses: Vec<Vec<i32>>,
}

impl GridCspSolver {
    pub fn new(grid: GridDimensions) -> Result<Self, GridCspError> {
        let mut this = GridCspSolver {
            var_count: 0,
            grid_vars: Vec::with_capacity(grid.width),
            clauses: Vec::new(),
        };
        // Generate cell vars
        for x in 0..grid.width {
            this.grid_vars.push(Vec::with_capacity(grid.height));
            for _y in 0..grid.height {
                let vars: Vec<i32> = (0..grid.number_max)
                    .map(|_| this.alloc_var().unwrap())
                    .collect();
                this.add_exactly_one_clause(&vars)?;
                this.grid_vars[x].push(vars);
            }
        }
        Ok(this)
    }

    pub fn get_cell_vars(&self, cell: impl Borrow<Cell>) -> &[i32] {
        let cell = cell.borrow();
        self.grid_vars[cell.x][cell.y].as_ref()
    }

    pub fn alloc_var(&mut self) -> Result<i32, GridCspError> {
        self.var_count = self
            .var_count
            .checked_add(1)
            .ok_or(GridCspError::TooManyVariables)?;
        Ok(self.var_count)
    }

    pub fn add_clause(&mut self, clause: Vec<i32>) {
        debug_assert!(!clause.is_empty());
        debug_assert!(clause.iter().all(|v| *v != 0));
        debug_assert!(clause.iter().all(|v| v.abs() <= self.var_count));
        self.clauses.push(clause)
    }

    pub fn add_alo_clause(&mut self, vars: impl AsRef<[i32]>) {
        self.add_clause(vars.as_ref().to_vec());
    }

    pub fn add_amo_clause(&mut self, vars: impl AsRef<[i32]>) {
        let vars = vars.as_ref();
        for i in 0..vars.len() {
            for j in i + 1..vars.len() {
                self.add_clause(vec![-vars[i], -vars[j]]);
            }
        }
    }

    pub fn add_exactly_one_clause(&mut self, vars: impl AsRef<[i32]>) -> Result<(), GridCspError> {
        // Adapted commander encoding from Kleiner and Kwon paper
        let mut current = vars.as_ref().to_vec();
        let mut next = Vec::<i32>::with_capacity(current.len() / 3 + 1);
        while current.len() > 3 {
            for group in current.chunks(3) {
                if group.len() > 1 {
                    let c = self.alloc_var()?;
                    for i in 0..group.len() {
                        for j in i + 1..group.len() {
                            self.add_clause(vec![-group[i], -group[j]]);
                        }
                        self.add_clause(vec![c, -group[i]])
                    }
                    self.add_clause([&[-c], group].concat());
                    next.push(c);
                } else {
                    next.push(group[0]);
                }
            }
            std::mem::swap(&mut current, &mut next);
            next.clear();
        }
        self.add_alo_clause(&current);
        self.add_amo_clause(&current);
        Ok(())
    }

    pub fn add_alternative_clause(
        &mut self,
        cells: impl AsRef<[Cell]>,
        solutions: impl AsRef<[Vec<u64>]>,
    ) -> Result<(), GridCspError> {
        let cells = cells.as_ref();
        let solutions = solutions.as_ref();
        let mut z_vars = Vec::new();
        for solution in solutions.iter() {
            let z = self.alloc_var()?;
            for (count, value) in solution.iter().dedup_with_count() {
                let vars: Vec<i32> = cells
                    .iter()
                    .map(|c| self.get_cell_vars(c)[*value as usize - 1])
                    .collect();
                for indexes in (0..vars.len()).combinations(cells.len() + 1 - count) {
                    self.add_clause([vec![-z], indexes.iter().map(|i| vars[*i]).collect()].concat())
                }
            }
            z_vars.push(z);
        }
        self.add_clause(z_vars);

        Ok(())
    }

    pub fn solve(&mut self) -> Result<Vec<Vec<u64>>, GridCspError> {
        let mut solver = match Solver::try_from((Config::default(), self.clauses.as_ref())) {
            Ok(solver) => solver,
            Err(Ok(Certificate::UNSAT)) => return Err(GridCspError::NoSolution),
            Err(Ok(Certificate::SAT(_))) => unreachable!(),
            Err(Err(err)) => return Err(GridCspError::SolverError(err)),
        };
        let solution = match solver.solve()? {
            Certificate::SAT(items) => items,
            Certificate::UNSAT => return Err(GridCspError::NoSolution),
        };
        let mut grid = Vec::<Vec<u64>>::with_capacity(self.grid_vars.len());
        for column in self.grid_vars.iter() {
            grid.push(Vec::<u64>::with_capacity(column.len()));
            for cell_vars in column.iter() {
                let selected: Vec<u64> = cell_vars
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| {
                        if solution[*v as usize - 1] > 0 {
                            Some(i as u64 + 1)
                        } else {
                            None
                        }
                    })
                    .collect();
                if selected.len() != 1 {
                    return Err(GridCspError::UnexpectedSolution);
                }
                grid.last_mut().unwrap().push(selected[0]);
            }
        }
        Ok(grid)
    }

    #[allow(clippy::needless_range_loop)]
    pub fn solve_unique(&mut self) -> Result<Vec<Vec<u64>>, GridCspError> {
        let grid = self.solve()?;

        let mut antisolution = Vec::<i32>::new();
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                antisolution.push(-self.grid_vars[x][y][grid[x][y] as usize - 1]);
            }
        }
        self.clauses.push(antisolution);
        if self.solve().is_ok() {
            return Err(GridCspError::SolutionNotUnique);
        }
        self.clauses.pop();

        Ok(grid)
    }
}

impl TryFrom<GenericProblem> for GridCspSolver {
    type Error = GridCspError;

    fn try_from(problem: GenericProblem) -> Result<Self, Self::Error> {
        problem.validate()?;
        let mut csp = GridCspSolver::new(problem.grid)?;
        for cg in problem.constraints.iter() {
            let cells = cg.group.to_cells(problem.grid);
            match cg.constraint {
                crate::model::Constraint::Add(v) => {
                    let solutions = add_enumerator(v, cells.len(), problem.grid.number_max);
                    csp.add_alternative_clause(cells, solutions)?;
                }
                crate::model::Constraint::Div(v) => {
                    let solutions = div_enumerator(v, cells.len(), problem.grid.number_max);
                    csp.add_alternative_clause(cells, solutions)?;
                }
                crate::model::Constraint::Equal(v) => {
                    for cell in cells.iter() {
                        let vars = csp.get_cell_vars(cell);
                        csp.add_alo_clause([vars[v as usize - 1]]);
                    }
                }
                crate::model::Constraint::Mul(v) => {
                    let solutions = mul_enumerator(v, cells.len(), problem.grid.number_max);
                    csp.add_alternative_clause(cells, solutions)?;
                }
                crate::model::Constraint::Sub(v) => {
                    let solutions = sub_enumerator(v, cells.len(), problem.grid.number_max);
                    csp.add_alternative_clause(cells, solutions)?;
                }
                crate::model::Constraint::Unique => {
                    let vars: Vec<Vec<i32>> = cells
                        .iter()
                        .map(|c| csp.get_cell_vars(c).to_vec())
                        .collect();
                    for v in 0..problem.grid.number_max {
                        csp.add_amo_clause(
                            vars.iter().map(|vs| vs[v as usize]).collect::<Vec<i32>>(),
                        );
                    }
                }
            }
        }
        Ok(csp)
    }
}
