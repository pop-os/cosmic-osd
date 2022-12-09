mod components;
mod polkit_agent;
mod polkit_agent_helper;
// mod polkit_dialog;
mod settings_daemon;

fn main() {
    components::app::main().unwrap();
}
