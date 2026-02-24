mod editor;
mod process_runner;
mod version_control;

pub use editor::{EditorService, SystemEditorService};
pub use process_runner::{ProcessRunner, SystemProcessRunner};
pub use version_control::{GitService, VersionControlService};
