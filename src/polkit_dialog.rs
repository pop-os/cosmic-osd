use gtk4::prelude::*;
use relm4::{ComponentController, ComponentParts, ComponentSender, RelmApp, SimpleComponent};
use std::{collections::HashMap, io};

use super::polkit_agent_helper::{AgentHelper, AgentHelperResponder, AgentMsg};

#[derive(Default)]
struct PolkitDialogModel {
    visible: bool,
    action_id: String,
    message: String,
    icon_name: String,
    details: HashMap<String, String>,
    password_label: String,
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
    Response(String),
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
            #[watch]
            set_visible: model.visible,
            #[wrap(Some)]
            set_child = &gtk4::Box {
                gtk4::Image {
                    #[watch]
                    set_icon_name: Some(&model.icon_name),
                },
                gtk4::Label {
                    #[watch]
                    set_label: &model.message,
                },
                // TODO: Show dropdown for who to authenticate as? Some implementations do?
                // TODO: User to authenticate as
                gtk4::Box {
                    set_orientation: gtk4::Orientation::Horizontal,
                    gtk4::Label {
                        #[watch]
                        set_label: &model.password_label,
                    },
                    gtk4::Stack {
                        #[name = "entry"]
                        gtk4::Entry {
                            set_activates_default: true,
                        },
                        #[name = "password_entry"]
                        gtk4::PasswordEntry {
                            set_activates_default: true,
                        },
                        #[watch]
                        set_visible_child: if model.echo {
                            entry.upcast_ref::<gtk4::Widget>()
                        } else {
                            password_entry.upcast_ref::<gtk4::Widget>()
                        }
                    }
                },
                /*
                gtk4::Expander {
                    set_label: Some("Details"),
                }
                */
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
            } => {
                self.visible = true;
                self.action_id = action_id;
                self.message = message;
                self.icon_name = icon_name;
                self.details = details;
            }
            PolkitDialogMsg::AgentMsg(msg) => match msg {
                AgentMsg::Request(s, echo) => {
                    self.visible = true;
                    self.password_label = s;
                    self.echo = echo;
                }
                AgentMsg::ShowError(s) => {
                    // XXX buttons? Don't show entry?
                    self.visible = true;
                    self.message = s;
                }
                AgentMsg::ShowDebug(s) => {
                    self.visible = true;
                    self.message = s;
                }
                AgentMsg::Complete(success) => {
                    self.visible = false; // TODO: destroy widget/component?
                                          // Notify PolkitAgent
                }
            },
            PolkitDialogMsg::Response(String) => {}
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
