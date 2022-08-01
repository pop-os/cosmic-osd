use gtk4::prelude::*;
use relm4::{ComponentController, ComponentParts, ComponentSender, RelmApp, SimpleComponent};
use std::{collections::HashMap, io};

use super::polkit_agent_helper::AgentMsg;

#[derive(Default)]
struct PolkitDialogModel {
    action_id: String,
    message: String,
    icon_name: String,
    details: HashMap<String, String>,
    echo: bool,
}

#[derive(Debug)]
enum PolkitDialogMsg {
    Begin {
        action_id: String,
        message: String,
        icon_name: String,
        details: HashMap<String, String>,
        // XXX responder?
    },
    AgentMsg(AgentMsg),
}

#[derive(Debug)]
enum PolkitDialogOutput {
    Cancel,
    Response(String),
}

#[relm4::component]
impl SimpleComponent for PolkitDialogModel {
    type Widgets = PolkitDialogWidgets;

    type InitParams = ();

    type Input = PolkitDialogMsg;
    type Output = PolkitDialogOutput;

    view! {
        gtk4::Dialog {
            #[wrap(Some)]
            set_child = &gtk4::Box {
                gtk4::Image {
                    #[watch]
                    set_icon_name: Some(&model.icon_name),
                },
                gtk4::Label {
                },
                // TODO: Show dropdown for who to authenticate as? Some implementations do?
                gtk4::PasswordEntry {
                },
                gtk4::Expander {
                    set_label: Some("Details"),
                }
            }
        }
    }

    fn init(
        _params: Self::InitParams,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PolkitDialogModel::default();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            PolkitDialogMsg::Begin {
                action_id,
                message,
                icon_name,
                details,
            } => {}
            PolkitDialogMsg::AgentMsg(msg) => match msg {
                AgentMsg::Request(s, echo) => {}
                AgentMsg::ShowError(s) => {}
                AgentMsg::ShowDebug(s) => {}
                AgentMsg::Complete(success) => {}
            },
        }
    }
}

fn f() {
    relm4::ComponentBuilder::<PolkitDialogModel>::new()
        .launch(())
        .detach()
        .sender();
}

struct PolkitDialog {}
