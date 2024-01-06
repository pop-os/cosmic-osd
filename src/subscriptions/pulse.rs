// Make sure not to fail if pulse not found, and reconnect?
// change to device shouldn't send osd?
// mute osd?

use cosmic::iced;
use futures::{executor::block_on, SinkExt};
use libpulse_binding::{
    callbacks::ListResult,
    context::{
        introspect::{Introspector, ServerInfo, SinkInfo, SourceInfo},
        subscribe::{Facility, InterestMaskSet, Operation},
        Context, FlagSet, State,
    },
    mainloop::standard::{IterateResult, Mainloop},
    volume::Volume
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[derive(Debug)]
pub enum Event {
    SinkVolume(u32),
    SinkMute(bool),
}

pub fn subscription() -> iced::Subscription<Event> {
    iced::subscription::channel(
        "pulse",
        20,
        |sender| async {
            std::thread::spawn(move || thread(sender));
            iced::futures::future::pending().await
        }
    )
}

struct Data {
    default_sink_name: RefCell<Option<String>>,
    sink_volume: Cell<Option<u32>>,
    sink_mute: Cell<Option<bool>>,
    introspector: Introspector,
    sender: RefCell<futures::channel::mpsc::Sender<Event>>,
}

impl Data {
    fn server_info_cb(self: &Rc<Self>, server_info: &ServerInfo) {
        let new_default_sink_name = server_info
            .default_sink_name
            .as_ref()
            .map(|x| x.clone().into_owned());
        let mut default_sink_name = self.default_sink_name.borrow_mut();
        if new_default_sink_name != *default_sink_name {
            if let Some(name) = &new_default_sink_name {
                self.get_sink_info_by_name(name);
            }
            *default_sink_name = new_default_sink_name;
        }
    }

    fn get_server_info(self: &Rc<Self>) {
        let data = self.clone();
        self.introspector
            .get_server_info(move |server_info| data.server_info_cb(server_info));
    }

    fn sink_info_cb(&self, sink_info_res: ListResult<&SinkInfo>) {
        if let ListResult::Item(sink_info) = sink_info_res {
            if sink_info.name.as_deref() != self.default_sink_name.borrow().as_deref() {
                return;
            }
            let volume = sink_info.volume.avg().0 / (Volume::NORMAL.0 / 100);
            if self.sink_mute.get() != Some(sink_info.mute) {
                self.sink_mute.set(Some(sink_info.mute));
                block_on(self.sender.borrow_mut().send(Event::SinkMute(sink_info.mute)));
            }
            if self.sink_volume.get() != Some(volume) {
                self.sink_volume.set(Some(volume));
                block_on(self.sender.borrow_mut().send(Event::SinkVolume(volume)));
            }
        }
    }

    fn get_sink_info_by_index(self: &Rc<Self>, index: u32) {
        let data = self.clone();
        self.introspector
            .get_sink_info_by_index(index, move |sink_info_res| {
                data.sink_info_cb(sink_info_res);
            });
    }

    fn get_sink_info_by_name(self: &Rc<Self>, name: &str) {
        let data = self.clone();
        self.introspector
            .get_sink_info_by_name(name, move |sink_info_res| {
                data.sink_info_cb(sink_info_res);
            });
    }

    fn subscribe_cb(
        self: &Rc<Self>,
        facility: Facility,
        _operation: Option<Operation>,
        index: u32,
    ) {
        match facility {
            Facility::Server => {
                self.get_server_info();
            }
            Facility::Sink => {
                self.get_sink_info_by_index(index);
            }
            _ => {}
        }
    }
}

fn thread(sender: futures::channel::mpsc::Sender<Event>) {
    'outer: loop {
        let mut main_loop = Mainloop::new().unwrap();
        let mut context = Context::new(&main_loop, "cosmic-osd").unwrap();

        let data = Rc::new(Data {
            introspector: context.introspect(),
            sink_volume: Cell::new(None),
            sink_mute: Cell::new(None),
            default_sink_name: RefCell::new(None),
            sender: RefCell::new(sender.clone()),
        });

        let data_clone = data.clone();
        context.set_subscribe_callback(Some(Box::new(move |facility, operation, index| {
            data_clone.subscribe_cb(facility.unwrap(), operation, index);
        })));

        context.connect(None, FlagSet::NOFAIL, None);

        loop {
            if sender.is_closed() {
                return;
            }

            match main_loop.iterate(false) {
                IterateResult::Success(_) => {}
                IterateResult::Err(_e) => {
                    break 'outer;
                }
                IterateResult::Quit(_e) => {
                    break 'outer;
                }
            }

            if context.get_state() == State::Ready {
                break;
            }
        }

        data.get_server_info();
        context.subscribe(InterestMaskSet::SERVER | InterestMaskSet::SINK, |_| {});

        main_loop.run();
    }
}
