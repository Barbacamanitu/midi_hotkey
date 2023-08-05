mod hotkey;
mod util;
use hotkey::HotkeyMap;
use nih_plug::prelude::*;
use process_path::get_dylib_path;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

struct MidiNoteEvent {
    timing: u32,
    voice_id: Option<i32>,
    channel: u8,
    note: u8,
    velocity: f32,
}

struct MidiHotkey {
    params: Arc<MidiHotkeyParams>,
    hotkey_map: hotkey::HotkeyMap,
    noteon: Vec<MidiNoteEvent>,
    noteoff: Vec<MidiNoteEvent>,
}

#[derive(Default, Params)]
struct MidiHotkeyParams {}

impl Default for MidiHotkey {
    fn default() -> Self {
        Self {
            params: Arc::new(MidiHotkeyParams::default()),
            hotkey_map: HotkeyMap::from_json(&util::read_json_file()),
            noteon: Vec::new(),
            noteoff: Vec::new(),
        }
    }
}

impl Plugin for MidiHotkey {
    const NAME: &'static str = "MIDI Hotkey";
    const VENDOR: &'static str = "Barbacamanitu";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // This plugin doesn't have any audio IO
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];

    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // We'll invert the channel, note index, velocity, pressure, CC value, pitch bend, and
        // anything else that is invertable for all events we receive
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn {
                    timing,
                    voice_id,
                    channel,
                    note,
                    velocity,
                } => {
                    let hk = self.hotkey_map.hotkeys.get(&note);

                    match hk {
                        Some(hotkey_entry) => {
                            for note in &hotkey_entry.outputs {
                                self.noteon.push(MidiNoteEvent {
                                    timing: timing,
                                    voice_id: voice_id,
                                    channel: channel,
                                    note: note.note,
                                    velocity: note.velocity,
                                });
                            }
                        }
                        None => context.send_event(NoteEvent::NoteOn {
                            timing,
                            voice_id,
                            channel: channel,
                            note: note,
                            velocity: velocity,
                        }),
                    }
                }
                NoteEvent::NoteOff {
                    timing,
                    voice_id,
                    channel,
                    note,
                    velocity,
                } => {
                    let hk = self.hotkey_map.hotkeys.get(&note);
                    match hk {
                        Some(hotkey_entry) => {
                            for note in hotkey_entry.outputs.iter() {
                                self.noteoff.push(MidiNoteEvent {
                                    timing: timing,
                                    voice_id: voice_id,
                                    channel: channel,
                                    note: note.note,
                                    velocity: 0.0,
                                });
                            }
                        }
                        None => context.send_event(NoteEvent::NoteOff {
                            timing,
                            voice_id,
                            channel: channel,
                            note: note,
                            velocity: velocity,
                        }),
                    }
                }
                _ => (),
            }
        }

        self.noteon.iter().for_each(|noteon| {
            context.send_event(NoteEvent::NoteOn {
                timing: noteon.timing,
                voice_id: noteon.voice_id,
                channel: noteon.channel,
                note: noteon.note,
                velocity: noteon.velocity,
            });
        });
        self.noteoff.iter().for_each(|noteoff| {
            context.send_event(NoteEvent::NoteOff {
                timing: noteoff.timing,
                voice_id: noteoff.voice_id,
                channel: noteoff.channel,
                note: noteoff.note,
                velocity: noteoff.velocity,
            });
        });
        self.noteon = Vec::new();
        self.noteoff = Vec::new();

        ProcessStatus::Normal
    }
}

impl ClapPlugin for MidiHotkey {
    const CLAP_ID: &'static str = "com.moist-plugins-gmbh.midi-inverter";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("Inverts all note and MIDI signals in ways you don't want to");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];
}

impl Vst3Plugin for MidiHotkey {
    const VST3_CLASS_ID: [u8; 16] = *b"M1d1Inv3r70rzAaA";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Tools];
}

nih_export_clap!(MidiHotkey);
nih_export_vst3!(MidiHotkey);
