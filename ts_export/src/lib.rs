use error::TsExportError;
use exporters::stdout::StdoutExport;
use process::Process;
use process_spawner::discard::BypassProcessSpawner;
use type_solver::TypeSolvingContextBuilder;

pub mod display_path;
pub mod error;
pub mod exporter_context;
pub mod exporters;
pub mod import;
pub mod process;
pub mod process_spawner;
pub mod solvers;
pub mod type_solver;

pub use syn;
pub use ts_json_subset as ts;

use std::{fs::File, io::Read, path::Path};

/// Helper function for demo
pub fn process_file<P: AsRef<Path>>(path: P) -> Result<(), TsExportError> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let solving_context = TypeSolvingContextBuilder::default()
        .add_default_solvers()
        .finish();

    Process {
        content,
        process_spawner: BypassProcessSpawner,
        exporter: StdoutExport,
    }
    .launch(&solving_context)?;

    Ok(())
}
