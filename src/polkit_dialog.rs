use futures::{channel::oneshot, lock::Mutex};
use gtk4::{glib, prelude::*};
use relm4::{ComponentController, ComponentParts, ComponentSender, RelmApp, SimpleComponent};
use std::{collections::HashMap, io, sync::Arc};

use super::polkit_agent::PolkitError;
use super::polkit_agent_helper::{AgentHelper, AgentHelperResponder, AgentMsg};

struct PolkitDialogModel {
    visible: bool,
    show_entry: bool,
    destroyed: bool,
    action_id: String,
    message: String,
    icon_name: String,
    details: HashMap<String, String>,
    password_label: String,
    echo: bool,
    responder: Arc<Mutex<AgentHelperResponder>>,
}

#[derive(Debug)]
enum PolkitDialogMsg {
    AgentMsg(AgentMsg),
    Response(String),
    Cancel,
}

#[relm4::component]
impl SimpleComponent for PolkitDialogModel {
    type Widgets = PolkitDialogWidgets;

    type InitParams = (
        String,
        String,
        String,
        HashMap<String, String>,
        (AgentHelper, AgentHelperResponder),
    );

    type Input = PolkitDialogMsg;
    type Output = Result<(), PolkitError>;

    view! {
        #[name = "dialog"]
        gtk4::Dialog::builder().use_header_bar(1).build() {
            // NOTE: Showing after destroying seems to cancel destruction
            #[watch]
            set_visible: model.visible && !model.destroyed,
            set_receives_default: true,
            set_default_response: gtk4::ResponseType::Accept,
            add_button: ("Cancel", gtk4::ResponseType::Cancel),
            add_button: ("Ok", gtk4::ResponseType::Accept),
            connect_response[sender, entry, password_entry, entry_stack] => move |_, resp| {
                if resp == gtk4::ResponseType::Accept {
                    let text = if entry_stack.visible_child().as_ref() == Some(password_entry.upcast_ref()) {
                        password_entry.text()
                    } else {
                        entry.text()
                    }.to_string();
                    sender.input(PolkitDialogMsg::Response(text));
                } else {
                    sender.input(PolkitDialogMsg::Cancel);
                }
            },
            #[wrap(Some)]
            set_child = &gtk4::Box {
                set_orientation: gtk4::Orientation::Vertical,
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
                    #[name = "entry_stack"]
                    gtk4::Stack {
                        #[watch]
                        set_visible: model.show_entry,
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
        params: Self::InitParams,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (mut helper, responder) = params.4;
        let model = PolkitDialogModel {
            action_id: params.0,
            message: params.1,
            icon_name: params.2,
            details: params.3,
            responder: Arc::new(Mutex::new(responder)),
            echo: false,
            password_label: String::new(),
            visible: false,
            destroyed: false,
            show_entry: false,
        };

        glib::MainContext::default().spawn(glib::clone!(@strong sender => async move {
            loop {
                match helper.next().await {
                    Ok(Some(msg)) => sender.input(PolkitDialogMsg::AgentMsg(msg)),
                    Ok(None) => {
                        break;
                    }
                    Err(err) => {
                        break;
                    }
                }
            }
        }));

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            PolkitDialogMsg::AgentMsg(msg) => match msg {
                AgentMsg::Request(s, echo) => {
                    self.visible = true;
                    self.password_label = s;
                    self.echo = echo;
                    self.show_entry = true;
                }
                AgentMsg::ShowError(s) => {
                    // XXX buttons? Don't show entry?
                    self.visible = true;
                    self.message = s;
                    self.show_entry = false;
                }
                AgentMsg::ShowDebug(s) => {
                    self.visible = true;
                    self.message = s;
                    self.show_entry = false;
                }
                AgentMsg::Complete(success) => {
                    self.destroyed = false;
                    sender.output(if success {
                        Ok(())
                    } else {
                        // XXX right error?
                        Err(PolkitError::Failed)
                    });
                }
            },
            PolkitDialogMsg::Response(resp) => {
                self.visible = false;
                let responder = self.responder.clone();
                glib::MainContext::default().spawn(async move {
                    if let Err(err) = responder.lock().await.response(&resp).await {}
                });
            }
            PolkitDialogMsg::Cancel => {
                self.destroyed = true;
            }
        }
    }

    fn post_view() {
        if model.destroyed {
            sender.output(Err(PolkitError::Cancelled));
            dialog.destroy();
        }
    }
}

pub async fn create_polkit_dialog(
    action_id: String,
    message: String,
    icon_name: String,
    details: HashMap<String, String>,
    helper: (AgentHelper, AgentHelperResponder),
) -> Result<(), PolkitError> {
    let (sender, receiver) = oneshot::channel();
    let mut sender = Some(sender);
    let sender = relm4::ComponentBuilder::<PolkitDialogModel>::new()
        .launch((action_id, message, icon_name, details, helper))
        .connect_receiver(move |_, msg| {
            if let Some(sender) = sender.take() {
                let _ = sender.send(msg);
            }
        })
        .sender()
        .clone();
    let _dropper = DestroyOnDrop(sender);
    receiver.await.unwrap_or(Err(PolkitError::Failed))
}

struct DestroyOnDrop(relm4::Sender<PolkitDialogMsg>);

impl Drop for DestroyOnDrop {
    fn drop(&mut self) {
        self.0.send(PolkitDialogMsg::Cancel);
    }
}
