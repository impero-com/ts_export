use std::{collections::HashMap, rc::Rc};

use syn::Type;
use ts_json_subset::types::TsType;

use crate::{
    display_path::DisplayPath,
    error::TsExportError,
    type_solver::{SolverResult, TypeInfo, TypeSolver, TypeSolvingContext},
};

pub struct PathSolver {
    pub entries: HashMap<String, Rc<dyn TypeSolver>>,
}

impl PathSolver {
    pub fn add_entry<S: TypeSolver + 'static>(&mut self, ident: String, solver: Rc<S>) {
        self.entries.insert(ident, solver);
    }
}

impl TypeSolver for PathSolver {
    fn solve_as_type(
        &self,
        solving_context: &TypeSolvingContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        let TypeInfo { ty, .. } = solver_info;
        match ty {
            Type::Path(ty) => {
                let ident = DisplayPath(&ty.path).to_string();
                match self.entries.get(&ident) {
                    Some(solver) => solver.solve_as_type(solving_context, solver_info),
                    _ => SolverResult::Continue,
                }
            }
            _ => SolverResult::Continue,
        }
    }
}