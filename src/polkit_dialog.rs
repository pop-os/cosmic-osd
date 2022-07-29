use std::{collections::HashMap, io};

struct PolkitDialog {}

impl PolkitDialog {
    fn new(
        action_id: String,
        message: String,
        icon_name: String,
        details: HashMap<String, String>,
    ) -> Self {
        PolkitDialog {}
    }

    fn request(&self, s: &str, echo: bool) {}

    fn show_error(&self, s: &str) {}

    fn show_debug(&self, s: &str) {}

    fn complete(&self, success: bool) {}
}
