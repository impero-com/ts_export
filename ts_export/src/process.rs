use crate::solvers::{
    array::ArraySolver, chrono::ChronoSolver, collections::CollectionsSolver,
    generics::GenericsSolver, import::ImportSolver, option::OptionSolver,
    primitives::PrimitivesSolver, reference::ReferenceSolver, tuple::TupleSolver,
};
use crate::{error::TsExportError, import::ImportContext};
use crate::{exporter::ExporterContext, type_solver::TypeSolvingContext};
use serde_derive_internals::{ast::Container, Ctxt, Derive};
use syn::{
    punctuated::Punctuated, DeriveInput, Item, ItemMod, ItemType, Path, PathArguments, PathSegment,
};
use ts_json_subset::export::ExportStatement;

pub struct Process<PS, E> {
    pub content: String,
    pub process_spawner: PS,
    pub exporter: E,
}

pub struct ProcessModule {
    current_path: Path,
    items: Vec<Item>,
    import_context: ImportContext,
}

pub struct ProcessModuleResultData {
    pub statements: Vec<ExportStatement>,
    pub path: Path,
}

pub struct ProcessModuleResult {
    pub data: ProcessModuleResultData,
    pub children: Vec<ProcessModuleResult>,
}

impl ProcessModule {
    pub fn new(current_path: syn::Path, items: Vec<Item>) -> Self {
        let mut import_context = ImportContext::default();
        import_context.parse_imported(&items);
        import_context.parse_scoped(&items);

        ProcessModule {
            current_path,
            items,
            import_context,
        }
    }

    pub fn launch<PS: ProcessSpawner>(
        self,
        process_spawner: &PS,
    ) -> Result<ProcessModuleResult, TsExportError> {
        let ProcessModule {
            current_path,
            import_context,
            items,
        } = self;

        let mut derive_inputs: Vec<(usize, DeriveInput)> = Vec::new();
        let mut type_aliases: Vec<(usize, ItemType)> = Vec::new();
        let mut mod_declarations: Vec<ItemMod> = Vec::new();

        items
            .into_iter()
            .enumerate()
            .for_each(|(index, item)| match item {
                Item::Enum(item) => derive_inputs.push((index, DeriveInput::from(item))),
                Item::Struct(item) => derive_inputs.push((index, DeriveInput::from(item))),
                Item::Type(item) => {
                    type_aliases.push((index, item));
                }
                Item::Mod(item) => {
                    mod_declarations.push(item);
                }
                _ => {}
            });

        let children: Vec<ProcessModuleResult> = mod_declarations
            .into_iter()
            .filter_map(|item_mod| {
                let ident = item_mod.ident;
                let mut path = current_path.clone();
                path.segments.push(PathSegment {
                    ident,
                    arguments: PathArguments::None,
                });
                match item_mod.content {
                    Some((_, items)) => Some(ProcessModule::new(path, items)),
                    _ => process_spawner.create_process(path),
                }
            })
            .map(|process_module| process_module.launch(process_spawner))
            .collect::<Result<_, _>>()?;

        let ctxt = Ctxt::default();
        let containers: Vec<(usize, Container)> = derive_inputs
            .iter()
            .filter_map(|(index, derive_input)| {
                Container::from_ast(&ctxt, &derive_input, Derive::Serialize)
                    .map(|container| (*index, container))
            })
            .collect();

        let mut solving_context = TypeSolvingContext::default();
        solving_context.add_solver(TupleSolver);
        solving_context.add_solver(ReferenceSolver);
        solving_context.add_solver(ArraySolver);
        solving_context.add_solver(CollectionsSolver::default());
        solving_context.add_solver(PrimitivesSolver::default());
        solving_context.add_solver(OptionSolver::default());
        solving_context.add_solver(GenericsSolver);
        solving_context.add_solver(ChronoSolver::default());
        solving_context.add_solver(ImportSolver);

        let exporter = ExporterContext {
            solving_context,
            import_context,
        };

        let type_export_statements = type_aliases.into_iter().map(|(index, item)| {
            exporter
                .export_statements_from_type_alias(item)
                .map(|statements| (index, statements))
        });
        let container_statements = containers.into_iter().map(|(index, container)| {
            exporter
                .export_statements_from_container(container)
                .map(|statements| (index, statements))
        });

        let mut statements: Vec<(usize, Vec<ExportStatement>)> = type_export_statements
            .chain(container_statements)
            .collect::<Result<Vec<_>, _>>()?;

        statements.sort_by_key(|(index, _)| *index);

        let statements: Vec<ExportStatement> = statements
            .into_iter()
            .flat_map(|(_, statements)| statements.into_iter())
            .collect();

        Ok(ProcessModuleResult {
            data: ProcessModuleResultData {
                statements,
                path: current_path,
            },
            children,
        })
    }
}

pub fn extractor(all: &mut Vec<ProcessModuleResultData>, iter: ProcessModuleResult) {
    iter.children
        .into_iter()
        .for_each(|child| extractor(all, child));
    all.push(iter.data);
}

impl<PS, E> Process<PS, E>
where
    PS: ProcessSpawner,
    E: Exporter,
{
    pub fn launch(&self) -> Result<(), TsExportError> {
        let ast = syn::parse_file(&self.content)?;

        let path = Path {
            leading_colon: None,
            segments: Punctuated::default(),
        };

        let res = ProcessModule::new(path, ast.items).launch(&self.process_spawner)?;
        let mut all_results: Vec<ProcessModuleResultData> = Vec::new();
        extractor(&mut all_results, res);

        all_results.into_iter().for_each(|result_data| {
            self.exporter.export_module(result_data);
        });

        Ok(())
    }
}

/// Creates a ProcessModule from a Path
pub trait ProcessSpawner {
    fn create_process(&self, path: Path) -> Option<ProcessModule>;
}

/// Specifies the behaviour of how to handle a resulting process' data
pub trait Exporter {
    fn export_module(&self, process_result: ProcessModuleResultData);
}
