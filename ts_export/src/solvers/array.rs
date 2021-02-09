/// Solver for the Array type variant
use crate::{
    error::TsExportError,
    exporter_context::ExporterContext,
    type_solver::{SolverResult, TypeInfo, TypeSolver},
};
use syn::Type;
use ts_json_subset::types::{ArrayType, PrimaryType, TsType};

pub struct ArraySolver;

impl TypeSolver for ArraySolver {
    fn solve_as_type(
        &self,
        solving_context: &ExporterContext,
        solver_info: &TypeInfo,
    ) -> SolverResult<TsType, TsExportError> {
        let result = match solver_info.ty {
            Type::Array(ty) => solving_context.solve_type(&TypeInfo {
                generics: solver_info.generics,
                ty: ty.elem.as_ref(),
            }),
            Type::Slice(ty) => solving_context.solve_type(&TypeInfo {
                generics: solver_info.generics,
                ty: ty.elem.as_ref(),
            }),
            _ => {
                return SolverResult::Continue;
            }
        };

        match result {
            Ok(TsType::PrimaryType(primary)) => SolverResult::Solved(TsType::PrimaryType(
                PrimaryType::ArrayType(ArrayType::new(primary)),
            )),
            // TODO: This is maybe unreachable ?
            Ok(ts_ty) => SolverResult::Error(TsExportError::UnexpectedType(ts_ty)),
            Err(e) => SolverResult::Error(e),
        }
    }
}
