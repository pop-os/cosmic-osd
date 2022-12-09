mod components;
mod dbus;
mod polkit_agent;
mod polkit_agent_helper;
mod settings_daemon;

fn main() {
    components::app::main().unwrap();
}
