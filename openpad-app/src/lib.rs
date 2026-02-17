// Order for `cargo makepad check script` (harness runs script_mod in this order).
// crate::components::projects_panel::script_mod(vm);
// crate::app::script_mod(vm);

pub mod app;
pub mod async_runtime;
pub mod components;
pub mod constants;
pub mod state;
pub mod ui;
pub mod utils;
