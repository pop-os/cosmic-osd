// Copyright 2026 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic_settings_audio_client::{self as audio_client};

pub type NodeId = u32;

#[derive(Debug, Default)]
pub struct Model {
    sinks: Nodes,
    sources: Nodes,
    pub active_sink: ActiveNode,
    pub active_source: ActiveNode,
    default_sink: Option<NodeId>,
    default_source: Option<NodeId>,
}

#[derive(Debug, Default)]
pub struct Nodes {
    active: Option<usize>,
    mute: Vec<bool>,
    id: Vec<NodeId>,
    volume: Vec<u32>,
}

impl Nodes {
    pub fn remove(&mut self, node_id: u32) -> bool {
        let Some(pos) = self.id.iter().position(|id| node_id == *id) else {
            return false;
        };
        self.mute.remove(pos);
        self.id.remove(pos);
        self.volume.remove(pos);
        if self.active == Some(pos) {
            self.active = None;
        }
        true
    }
}

#[derive(Debug, Default)]
pub struct ActiveNode {
    pub volume: u32,
    pub mute: bool,
}

pub enum Response {
    SinkVolume(u32, bool),
    SourceVolume(u32, bool),
}

impl Model {
    pub fn update(&mut self, event: audio_client::Event) -> Option<Response> {
        match event {
            audio_client::Event::NodeMute(node_id, mute) => {
                if let Some(pos) = self.sinks.id.iter().position(|id| node_id == *id) {
                    self.sinks.mute[pos] = mute;
                    if self.sinks.active == Some(pos) && self.active_sink.mute != mute {
                        self.active_sink.mute = mute;
                        return Some(Response::SinkVolume(self.sinks.volume[pos], mute));
                    }
                } else if let Some(pos) = self.sources.id.iter().position(|id| node_id == *id) {
                    self.sources.mute[pos] = mute;
                    if self.sources.active == Some(pos) && self.active_source.mute != mute {
                        self.active_source.mute = mute;
                        return Some(Response::SourceVolume(self.sources.volume[pos], mute));
                    }
                }
            }

            audio_client::Event::NodeVolume(node_id, volume, _balance) => {
                if let Some(pos) = self.sinks.id.iter().position(|id| node_id == *id) {
                    self.sinks.volume[pos] = volume;
                    if self.default_sink.as_ref().is_some_and(|&id| id == node_id)
                        && let Some(pos) = self.sinks.active
                    {
                        let changed = self.active_sink.mute != self.sinks.mute[pos]
                            || self.active_sink.volume != self.sinks.volume[pos];
                        self.active_sink.mute = self.sinks.mute[pos];
                        self.active_sink.volume = self.sinks.volume[pos];

                        return changed.then_some(Response::SinkVolume(
                            self.active_sink.volume,
                            self.active_sink.mute,
                        ));
                    }
                } else if let Some(pos) = self.sources.id.iter().position(|id| node_id == *id) {
                    self.sources.volume[pos] = volume;
                    if self
                        .default_source
                        .as_ref()
                        .is_some_and(|&id| id == node_id)
                        && let Some(pos) = self.sources.active
                    {
                        let changed = self.active_source.mute != self.sources.mute[pos]
                            || self.active_source.volume != self.sinks.volume[pos];
                        self.active_source.mute = self.sources.mute[pos];
                        self.active_source.volume = self.sources.volume[pos];
                        return changed.then_some(Response::SourceVolume(
                            self.active_source.volume,
                            self.active_source.mute,
                        ));
                    }
                }
            }

            audio_client::Event::DefaultSink(node_id) => {
                self.default_sink = Some(node_id);
                if let Some(pos) = self.sinks.id.iter().position(|&id| id == node_id) {
                    self.sinks.active = Some(pos);
                    self.active_sink.mute = self.sinks.mute[pos];
                    self.active_sink.volume = self.sinks.volume[pos];
                    return Some(Response::SinkVolume(
                        self.active_sink.volume,
                        self.active_sink.mute,
                    ));
                }
            }

            audio_client::Event::DefaultSource(node_id) => {
                self.default_source = Some(node_id);
                if let Some(pos) = self.sources.id.iter().position(|&id| id == node_id) {
                    self.sources.active = Some(pos);
                    self.active_source.mute = self.sources.mute[pos];
                    self.active_source.volume = self.sources.volume[pos];
                    return Some(Response::SourceVolume(
                        self.active_source.volume,
                        self.active_source.mute,
                    ));
                }
            }

            audio_client::Event::Node(node_id, node) => {
                if node.is_sink {
                    let pos = if let Some(pos) = self.sinks.id.iter().position(|&id| id == node_id)
                    {
                        pos
                    } else {
                        self.sinks.id.push(node_id);
                        self.sinks.volume.push(0);
                        self.sinks.mute.push(false);
                        self.sinks.id.len() - 1
                    };

                    if let Some(default_node_id) = self.default_sink
                        && default_node_id == node_id
                    {
                        self.sinks.active = Some(pos);
                        self.active_sink.mute = self.sinks.mute[pos];
                        self.active_sink.volume = self.sinks.volume[pos];
                    }
                } else {
                    let pos =
                        if let Some(pos) = self.sources.id.iter().position(|&id| id == node_id) {
                            pos
                        } else {
                            self.sources.id.push(node_id);
                            self.sources.volume.push(0);
                            self.sources.mute.push(false);
                            self.sources.id.len() - 1
                        };

                    if let Some(default_node_id) = self.default_source
                        && default_node_id == node_id
                    {
                        self.sources.active = Some(pos);
                        self.active_source.mute = self.sources.mute[pos];
                        self.active_source.volume = self.sources.volume[pos];
                    }
                }
            }

            audio_client::Event::RemoveNode(node_id) => {
                if !self.sinks.remove(node_id) {
                    self.sources.remove(node_id);
                }
            }

            _ => (),
        }

        None
    }
}
