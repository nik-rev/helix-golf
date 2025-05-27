//! Parse string of key sequences.
//!
//! Vendored from the Helix codebase.
//!
//! Includes some modifications.

//! Convert the Helix `KeyCode`s into something that [`vhs`](vhs) can understand
//!
//! [vhs]: https://github.com/charmbracelet/vhs

use std::fmt::Display;

use crate::helix_config;

impl Display for KeyEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mods = format!(
            "{}{}{}{}",
            if self.modifiers.contains(KeyModifiers::SUPER) {
                "Meta+"
            } else {
                ""
            },
            if self.modifiers.contains(KeyModifiers::SHIFT) {
                "Shift+"
            } else {
                ""
            },
            if self.modifiers.contains(KeyModifiers::ALT) {
                "Alt+"
            } else {
                ""
            },
            if self.modifiers.contains(KeyModifiers::CONTROL) {
                "Ctrl+"
            } else {
                ""
            },
        );
        let out = match self.code {
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PageUp".to_string(),
            KeyCode::PageDown => "PageDown".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Insert => "Insert".to_string(),
            KeyCode::F(i) => format!("F{i}"),
            KeyCode::Char(ch) => {
                if mods.is_empty() {
                    let ch = if ch == '"' {
                        // Double-quotes escaped with backtick
                        "`\"`".to_string()
                    } else {
                        format!("\"{ch}\"")
                    };
                    format!(r#"Type {ch}"#)
                } else if let Some(mods) = helix_config::remap(mods.as_str(), ch) {
                    mods
                } else {
                    format!(r#"{mods}"{ch}""#)
                }
            }
            KeyCode::Null => "Null".to_string(),
            KeyCode::Esc => "Escape".to_string(),
            KeyCode::CapsLock => "CapsLock".to_string(),
            KeyCode::ScrollLock => "ScrollLock".to_string(),
            KeyCode::NumLock => "NumLock".to_string(),
            KeyCode::PrintScreen => "PrintScreen".to_string(),
            KeyCode::Pause => "Pause".to_string(),
            KeyCode::Menu => "Menu".to_string(),
            KeyCode::KeypadBegin => "KeypadBegin".to_string(),
            KeyCode::Media(media_key_code) => match media_key_code {
                MediaKeyCode::Play => "Play",
                MediaKeyCode::Pause => "Pause",
                MediaKeyCode::PlayPause => "PlayPause",
                MediaKeyCode::Reverse => "Reverse",
                MediaKeyCode::Stop => "Stop",
                MediaKeyCode::FastForward => "FastForward",
                MediaKeyCode::Rewind => "Rewind",
                MediaKeyCode::TrackNext => "TrackNext",
                MediaKeyCode::TrackPrevious => "TrackPrevious",
                MediaKeyCode::Record => "Record",
                MediaKeyCode::LowerVolume => "LowerVolume",
                MediaKeyCode::RaiseVolume => "RaiseVolume",
                MediaKeyCode::MuteVolume => "MuteVolume",
            }
            .to_string(),
            KeyCode::Modifier(modifier_key_code) => match modifier_key_code {
                ModifierKeyCode::LeftShift => "LeftShift",
                ModifierKeyCode::LeftControl => "LeftControl",
                ModifierKeyCode::LeftAlt => "LeftAlt",
                ModifierKeyCode::LeftSuper => "LeftSuper",
                ModifierKeyCode::LeftHyper => "LeftHyper",
                ModifierKeyCode::LeftMeta => "LeftMeta",
                ModifierKeyCode::RightShift => "RightShift",
                ModifierKeyCode::RightControl => "RightControl",
                ModifierKeyCode::RightAlt => "RightAlt",
                ModifierKeyCode::RightSuper => "RightSuper",
                ModifierKeyCode::RightHyper => "RightHyper",
                ModifierKeyCode::RightMeta => "RightMeta",
                ModifierKeyCode::IsoLevel3Shift => "IsoLevel3Shift",
                ModifierKeyCode::IsoLevel5Shift => "IsoLevel5Shift",
            }
            .to_string(),
        };

        write!(f, "{out}")?;

        Ok(())
    }
}

// Mozilla Public License Version 2.0
// ==================================
//
// 1. Definitions
// --------------
//
// 1.1. "Contributor"
//     means each individual or legal entity that creates, contributes to
//     the creation of, or owns Covered Software.
//
// 1.2. "Contributor Version"
//     means the combination of the Contributions of others (if any) used
//     by a Contributor and that particular Contributor's Contribution.
//
// 1.3. "Contribution"
//     means Covered Software of a particular Contributor.
//
// 1.4. "Covered Software"
//     means Source Code Form to which the initial Contributor has attached
//     the notice in Exhibit A, the Executable Form of such Source Code
//     Form, and Modifications of such Source Code Form, in each case
//     including portions thereof.
//
// 1.5. "Incompatible With Secondary Licenses"
//     means
//
//     (a) that the initial Contributor has attached the notice described
//         in Exhibit B to the Covered Software; or
//
//     (b) that the Covered Software was made available under the terms of
//         version 1.1 or earlier of the License, but not also under the
//         terms of a Secondary License.
//
// 1.6. "Executable Form"
//     means any form of the work other than Source Code Form.
//
// 1.7. "Larger Work"
//     means a work that combines Covered Software with other material, in
//     a separate file or files, that is not Covered Software.
//
// 1.8. "License"
//     means this document.
//
// 1.9. "Licensable"
//     means having the right to grant, to the maximum extent possible,
//     whether at the time of the initial grant or subsequently, any and
//     all of the rights conveyed by this License.
//
// 1.10. "Modifications"
//     means any of the following:
//
//     (a) any file in Source Code Form that results from an addition to,
//         deletion from, or modification of the contents of Covered
//         Software; or
//
//     (b) any new file in Source Code Form that contains any Covered
//         Software.
//
// 1.11. "Patent Claims" of a Contributor
//     means any patent claim(s), including without limitation, method,
//     process, and apparatus claims, in any patent Licensable by such
//     Contributor that would be infringed, but for the grant of the
//     License, by the making, using, selling, offering for sale, having
//     made, import, or transfer of either its Contributions or its
//     Contributor Version.
//
// 1.12. "Secondary License"
//     means either the GNU General Public License, Version 2.0, the GNU
//     Lesser General Public License, Version 2.1, the GNU Affero General
//     Public License, Version 3.0, or any later versions of those
//     licenses.
//
// 1.13. "Source Code Form"
//     means the form of the work preferred for making modifications.
//
// 1.14. "You" (or "Your")
//     means an individual or a legal entity exercising rights under this
//     License. For legal entities, "You" includes any entity that
//     controls, is controlled by, or is under common control with You. For
//     purposes of this definition, "control" means (a) the power, direct
//     or indirect, to cause the direction or management of such entity,
//     whether by contract or otherwise, or (b) ownership of more than
//     fifty percent (50%) of the outstanding shares or beneficial
//     ownership of such entity.
//
// 2. License Grants and Conditions
// --------------------------------
//
// 2.1. Grants
//
// Each Contributor hereby grants You a world-wide, royalty-free,
// non-exclusive license:
//
// (a) under intellectual property rights (other than patent or trademark)
//     Licensable by such Contributor to use, reproduce, make available,
//     modify, display, perform, distribute, and otherwise exploit its
//     Contributions, either on an unmodified basis, with Modifications, or
//     as part of a Larger Work; and
//
// (b) under Patent Claims of such Contributor to make, use, sell, offer
//     for sale, have made, import, and otherwise transfer either its
//     Contributions or its Contributor Version.
//
// 2.2. Effective Date
//
// The licenses granted in Section 2.1 with respect to any Contribution
// become effective for each Contribution on the date the Contributor first
// distributes such Contribution.
//
// 2.3. Limitations on Grant Scope
//
// The licenses granted in this Section 2 are the only rights granted under
// this License. No additional rights or licenses will be implied from the
// distribution or licensing of Covered Software under this License.
// Notwithstanding Section 2.1(b) above, no patent license is granted by a
// Contributor:
//
// (a) for any code that a Contributor has removed from Covered Software;
//     or
//
// (b) for infringements caused by: (i) Your and any other third party's
//     modifications of Covered Software, or (ii) the combination of its
//     Contributions with other software (except as part of its Contributor
//     Version); or
//
// (c) under Patent Claims infringed by Covered Software in the absence of
//     its Contributions.
//
// This License does not grant any rights in the trademarks, service marks,
// or logos of any Contributor (except as may be necessary to comply with
// the notice requirements in Section 3.4).
//
// 2.4. Subsequent Licenses
//
// No Contributor makes additional grants as a result of Your choice to
// distribute the Covered Software under a subsequent version of this
// License (see Section 10.2) or under the terms of a Secondary License (if
// permitted under the terms of Section 3.3).
//
// 2.5. Representation
//
// Each Contributor represents that the Contributor believes its
// Contributions are its original creation(s) or it has sufficient rights
// to grant the rights to its Contributions conveyed by this License.
//
// 2.6. Fair Use
//
// This License is not intended to limit any rights You have under
// applicable copyright doctrines of fair use, fair dealing, or other
// equivalents.
//
// 2.7. Conditions
//
// Sections 3.1, 3.2, 3.3, and 3.4 are conditions of the licenses granted
// in Section 2.1.
//
// 3. Responsibilities
// -------------------
//
// 3.1. Distribution of Source Form
//
// All distribution of Covered Software in Source Code Form, including any
// Modifications that You create or to which You contribute, must be under
// the terms of this License. You must inform recipients that the Source
// Code Form of the Covered Software is governed by the terms of this
// License, and how they can obtain a copy of this License. You may not
// attempt to alter or restrict the recipients' rights in the Source Code
// Form.
//
// 3.2. Distribution of Executable Form
//
// If You distribute Covered Software in Executable Form then:
//
// (a) such Covered Software must also be made available in Source Code
//     Form, as described in Section 3.1, and You must inform recipients of
//     the Executable Form how they can obtain a copy of such Source Code
//     Form by reasonable means in a timely manner, at a charge no more
//     than the cost of distribution to the recipient; and
//
// (b) You may distribute such Executable Form under the terms of this
//     License, or sublicense it under different terms, provided that the
//     license for the Executable Form does not attempt to limit or alter
//     the recipients' rights in the Source Code Form under this License.
//
// 3.3. Distribution of a Larger Work
//
// You may create and distribute a Larger Work under terms of Your choice,
// provided that You also comply with the requirements of this License for
// the Covered Software. If the Larger Work is a combination of Covered
// Software with a work governed by one or more Secondary Licenses, and the
// Covered Software is not Incompatible With Secondary Licenses, this
// License permits You to additionally distribute such Covered Software
// under the terms of such Secondary License(s), so that the recipient of
// the Larger Work may, at their option, further distribute the Covered
// Software under the terms of either this License or such Secondary
// License(s).
//
// 3.4. Notices
//
// You may not remove or alter the substance of any license notices
// (including copyright notices, patent notices, disclaimers of warranty,
// or limitations of liability) contained within the Source Code Form of
// the Covered Software, except that You may alter any license notices to
// the extent required to remedy known factual inaccuracies.
//
// 3.5. Application of Additional Terms
//
// You may choose to offer, and to charge a fee for, warranty, support,
// indemnity or liability obligations to one or more recipients of Covered
// Software. However, You may do so only on Your own behalf, and not on
// behalf of any Contributor. You must make it absolutely clear that any
// such warranty, support, indemnity, or liability obligation is offered by
// You alone, and You hereby agree to indemnify every Contributor for any
// liability incurred by such Contributor as a result of warranty, support,
// indemnity or liability terms You offer. You may include additional
// disclaimers of warranty and limitations of liability specific to any
// jurisdiction.
//
// 4. Inability to Comply Due to Statute or Regulation
// ---------------------------------------------------
//
// If it is impossible for You to comply with any of the terms of this
// License with respect to some or all of the Covered Software due to
// statute, judicial order, or regulation then You must: (a) comply with
// the terms of this License to the maximum extent possible; and (b)
// describe the limitations and the code they affect. Such description must
// be placed in a text file included with all distributions of the Covered
// Software under this License. Except to the extent prohibited by statute
// or regulation, such description must be sufficiently detailed for a
// recipient of ordinary skill to be able to understand it.
//
// 5. Termination
// --------------
//
// 5.1. The rights granted under this License will terminate automatically
// if You fail to comply with any of its terms. However, if You become
// compliant, then the rights granted under this License from a particular
// Contributor are reinstated (a) provisionally, unless and until such
// Contributor explicitly and finally terminates Your grants, and (b) on an
// ongoing basis, if such Contributor fails to notify You of the
// non-compliance by some reasonable means prior to 60 days after You have
// come back into compliance. Moreover, Your grants from a particular
// Contributor are reinstated on an ongoing basis if such Contributor
// notifies You of the non-compliance by some reasonable means, this is the
// first time You have received notice of non-compliance with this License
// from such Contributor, and You become compliant prior to 30 days after
// Your receipt of the notice.
//
// 5.2. If You initiate litigation against any entity by asserting a patent
// infringement claim (excluding declaratory judgment actions,
// counter-claims, and cross-claims) alleging that a Contributor Version
// directly or indirectly infringes any patent, then the rights granted to
// You by any and all Contributors for the Covered Software under Section
// 2.1 of this License shall terminate.
//
// 5.3. In the event of termination under Sections 5.1 or 5.2 above, all
// end user license agreements (excluding distributors and resellers) which
// have been validly granted by You or Your distributors under this License
// prior to termination shall survive termination.
//
// ************************************************************************
// *                                                                      *
// *  6. Disclaimer of Warranty                                           *
// *  -------------------------                                           *
// *                                                                      *
// *  Covered Software is provided under this License on an "as is"       *
// *  basis, without warranty of any kind, either expressed, implied, or  *
// *  statutory, including, without limitation, warranties that the       *
// *  Covered Software is free of defects, merchantable, fit for a        *
// *  particular purpose or non-infringing. The entire risk as to the     *
// *  quality and performance of the Covered Software is with You.        *
// *  Should any Covered Software prove defective in any respect, You     *
// *  (not any Contributor) assume the cost of any necessary servicing,   *
// *  repair, or correction. This disclaimer of warranty constitutes an   *
// *  essential part of this License. No use of any Covered Software is   *
// *  authorized under this License except under this disclaimer.         *
// *                                                                      *
// ************************************************************************
//
// ************************************************************************
// *                                                                      *
// *  7. Limitation of Liability                                          *
// *  --------------------------                                          *
// *                                                                      *
// *  Under no circumstances and under no legal theory, whether tort      *
// *  (including negligence), contract, or otherwise, shall any           *
// *  Contributor, or anyone who distributes Covered Software as          *
// *  permitted above, be liable to You for any direct, indirect,         *
// *  special, incidental, or consequential damages of any character      *
// *  including, without limitation, damages for lost profits, loss of    *
// *  goodwill, work stoppage, computer failure or malfunction, or any    *
// *  and all other commercial damages or losses, even if such party      *
// *  shall have been informed of the possibility of such damages. This   *
// *  limitation of liability shall not apply to liability for death or   *
// *  personal injury resulting from such party's negligence to the       *
// *  extent applicable law prohibits such limitation. Some               *
// *  jurisdictions do not allow the exclusion or limitation of           *
// *  incidental or consequential damages, so this exclusion and          *
// *  limitation may not apply to You.                                    *
// *                                                                      *
// ************************************************************************
//
// 8. Litigation
// -------------
//
// Any litigation relating to this License may be brought only in the
// courts of a jurisdiction where the defendant maintains its principal
// place of business and such litigation shall be governed by laws of that
// jurisdiction, without reference to its conflict-of-law provisions.
// Nothing in this Section shall prevent a party's ability to bring
// cross-claims or counter-claims.
//
// 9. Miscellaneous
// ----------------
//
// This License represents the complete agreement concerning the subject
// matter hereof. If any provision of this License is held to be
// unenforceable, such provision shall be reformed only to the extent
// necessary to make it enforceable. Any law or regulation which provides
// that the language of a contract shall be construed against the drafter
// shall not be used to construe this License against a Contributor.
//
// 10. Versions of the License
// ---------------------------
//
// 10.1. New Versions
//
// Mozilla Foundation is the license steward. Except as provided in Section
// 10.3, no one other than the license steward has the right to modify or
// publish new versions of this License. Each version will be given a
// distinguishing version number.
//
// 10.2. Effect of New Versions
//
// You may distribute the Covered Software under the terms of the version
// of the License under which You originally received the Covered Software,
// or under the terms of any subsequent version published by the license
// steward.
//
// 10.3. Modified Versions
//
// If you create software not governed by this License, and you want to
// create a new license for such software, you may create and use a
// modified version of this License if you rename the license and remove
// any references to the name of the license steward (except to note that
// such modified license differs from this License).
//
// 10.4. Distributing Source Code Form that is Incompatible With Secondary
// Licenses
//
// If You choose to distribute Source Code Form that is Incompatible With
// Secondary Licenses under the terms of this version of the License, the
// notice described in Exhibit B of this License must be attached.
//
// Exhibit A - Source Code Form License Notice
// -------------------------------------------
//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// If it is not possible or desirable to put the notice in a particular
// file, then You may include the notice in a location (such as a LICENSE
// file in a relevant directory) where a recipient would be likely to look
// for such a notice.
//
// You may add additional accurate notices of copyright ownership.
//
// Exhibit B - "Incompatible With Secondary Licenses" Notice
// ---------------------------------------------------------
//
//   This Source Code Form is "Incompatible With Secondary Licenses", as
//   defined by the Mozilla Public License, v. 2.0.

use bitflags::bitflags;
use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("Could not parse the keys")]
pub struct ParseKeysError {
    #[source_code]
    src: NamedSource<String>,
    error: String,
    #[label("{error}")]
    span: SourceSpan,
}

pub fn parse_keys(keys_str: &str, filename: &str) -> Result<Vec<KeyEvent>, ParseKeysError> {
    let mut keys_res: Result<_, ParseKeysError> = Ok(Vec::new());
    let mut i = 0;
    while let Ok(keys) = &mut keys_res {
        if i >= keys_str.len() {
            break;
        }
        if !keys_str.is_char_boundary(i) {
            i += 1;
            continue;
        }

        let s = &keys_str[i..];
        let mut end_i = 1;
        while !s.is_char_boundary(end_i) {
            end_i += 1;
        }
        let c = &s[..end_i];
        if c == ">" {
            keys.push(keys::GREATER_THAN);
            i += end_i;
        } else if c != "<" {
            keys.push(if c == "-" { keys::MINUS } else { c });
            i += end_i;
        } else {
            match s.find('>') {
                Some(end_i) => {
                    keys.push(&s[1..end_i]);
                    i += end_i + 1;
                }
                None => {
                    keys.push(keys::LESS_THAN);
                    i += end_i;
                }
            }
        }
    }
    keys_res.and_then(|keys| {
        keys.into_iter()
            .map(|s| KeyEvent::from_str(s, filename, keys_str))
            .collect()
    })
}

impl KeyEvent {
    pub fn from_str(s: &str, filename: &str, keys_str: &str) -> Result<Self, ParseKeysError> {
        let mut tokens: Vec<_> = s.split('-').collect();
        let mut code = match tokens.pop().ok_or_else(|| ParseKeysError {
            src: NamedSource::new(filename, keys_str.to_string()),
            error: "Missing key code".to_string(),
            span: (0, 4).into(),
        })? {
            keys::BACKSPACE => KeyCode::Backspace,
            keys::ENTER => KeyCode::Enter,
            keys::LEFT => KeyCode::Left,
            keys::RIGHT => KeyCode::Right,
            keys::UP => KeyCode::Up,
            keys::DOWN => KeyCode::Down,
            keys::HOME => KeyCode::Home,
            keys::END => KeyCode::End,
            keys::PAGEUP => KeyCode::PageUp,
            keys::PAGEDOWN => KeyCode::PageDown,
            keys::TAB => KeyCode::Tab,
            keys::DELETE => KeyCode::Delete,
            keys::INSERT => KeyCode::Insert,
            keys::NULL => KeyCode::Null,
            keys::ESC => KeyCode::Esc,
            keys::SPACE => KeyCode::Char(' '),
            keys::MINUS => KeyCode::Char('-'),
            keys::LESS_THAN => KeyCode::Char('<'),
            keys::GREATER_THAN => KeyCode::Char('>'),
            keys::CAPS_LOCK => KeyCode::CapsLock,
            keys::SCROLL_LOCK => KeyCode::ScrollLock,
            keys::NUM_LOCK => KeyCode::NumLock,
            keys::PRINT_SCREEN => KeyCode::PrintScreen,
            keys::PAUSE => KeyCode::Pause,
            keys::MENU => KeyCode::Menu,
            keys::KEYPAD_BEGIN => KeyCode::KeypadBegin,
            keys::PLAY => KeyCode::Media(MediaKeyCode::Play),
            keys::PAUSE_MEDIA => KeyCode::Media(MediaKeyCode::Pause),
            keys::PLAY_PAUSE => KeyCode::Media(MediaKeyCode::PlayPause),
            keys::STOP => KeyCode::Media(MediaKeyCode::Stop),
            keys::REVERSE => KeyCode::Media(MediaKeyCode::Reverse),
            keys::FAST_FORWARD => KeyCode::Media(MediaKeyCode::FastForward),
            keys::REWIND => KeyCode::Media(MediaKeyCode::Rewind),
            keys::TRACK_NEXT => KeyCode::Media(MediaKeyCode::TrackNext),
            keys::TRACK_PREVIOUS => KeyCode::Media(MediaKeyCode::TrackPrevious),
            keys::RECORD => KeyCode::Media(MediaKeyCode::Record),
            keys::LOWER_VOLUME => KeyCode::Media(MediaKeyCode::LowerVolume),
            keys::RAISE_VOLUME => KeyCode::Media(MediaKeyCode::RaiseVolume),
            keys::MUTE_VOLUME => KeyCode::Media(MediaKeyCode::MuteVolume),
            keys::LEFT_SHIFT => KeyCode::Modifier(ModifierKeyCode::LeftShift),
            keys::LEFT_CONTROL => KeyCode::Modifier(ModifierKeyCode::LeftControl),
            keys::LEFT_ALT => KeyCode::Modifier(ModifierKeyCode::LeftAlt),
            keys::LEFT_SUPER => KeyCode::Modifier(ModifierKeyCode::LeftSuper),
            keys::LEFT_HYPER => KeyCode::Modifier(ModifierKeyCode::LeftHyper),
            keys::LEFT_META => KeyCode::Modifier(ModifierKeyCode::LeftMeta),
            keys::RIGHT_SHIFT => KeyCode::Modifier(ModifierKeyCode::RightShift),
            keys::RIGHT_CONTROL => KeyCode::Modifier(ModifierKeyCode::RightControl),
            keys::RIGHT_ALT => KeyCode::Modifier(ModifierKeyCode::RightAlt),
            keys::RIGHT_SUPER => KeyCode::Modifier(ModifierKeyCode::RightSuper),
            keys::RIGHT_HYPER => KeyCode::Modifier(ModifierKeyCode::RightHyper),
            keys::RIGHT_META => KeyCode::Modifier(ModifierKeyCode::RightMeta),
            keys::ISO_LEVEL_3_SHIFT => KeyCode::Modifier(ModifierKeyCode::IsoLevel3Shift),
            keys::ISO_LEVEL_5_SHIFT => KeyCode::Modifier(ModifierKeyCode::IsoLevel5Shift),
            single if single.chars().count() == 1 => KeyCode::Char(single.chars().next().unwrap()),
            function if function.len() > 1 && function.starts_with('F') => {
                let function: String = function.chars().skip(1).collect();
                let function = str::parse::<u8>(&function).map_err(|err| ParseKeysError {
                    src: NamedSource::new(filename, keys_str.to_string()),
                    error: err.to_string(),
                    span: (0, 4).into(),
                })?;
                (function > 0 && function < 25)
                    .then_some(KeyCode::F(function))
                    .ok_or_else(|| ParseKeysError {
                        src: NamedSource::new(filename, keys_str.to_string()),
                        error: format!("Invalid function key {function}"),
                        span: (0, 4).into(),
                    })?
            }
            // Checking that the last token is empty ensures that this branch is only taken if
            // `-` is used as a code. For example this branch will not be taken for `S-` (which is
            // missing a code).
            _ if s.ends_with('-') && tokens.last().is_some_and(|t| t.is_empty()) => {
                if s == "-" {
                    return Ok(KeyEvent {
                        code: KeyCode::Char('-'),
                        modifiers: KeyModifiers::empty(),
                    });
                } else {
                    let suggestion = format!("{}-{}", s.trim_end_matches('-'), keys::MINUS);
                    return Err(ParseKeysError {
                        src: NamedSource::new(filename, keys_str.to_string()),
                        error: format!(
                            "Key '-' cannot be used with modifiers, use '{suggestion}' instead"
                        ),
                        span: (0, 4).into(),
                    });
                }
            }
            invalid => {
                return Err(ParseKeysError {
                    src: NamedSource::new(filename, keys_str.to_string()),
                    error: format!("Invalid key code '{invalid}'"),
                    span: (0, 4).into(),
                });
            }
        };

        let mut modifiers = KeyModifiers::empty();
        for token in tokens {
            let flag = match token {
                "shift" => KeyModifiers::SHIFT,
                "S" => {
                    return Err(ParseKeysError {
                        src: NamedSource::new(filename, keys_str.to_string()),
                        error: format!("Invalid key modifier '{token}-': Use 'shift-' instead"),
                        span: (0, 4).into(),
                    });
                }
                // NOTE: in Helix it is `A`
                "alt" => KeyModifiers::ALT,
                "A" => {
                    return Err(ParseKeysError {
                        src: NamedSource::new(filename, keys_str.to_string()),
                        error: format!("Invalid key modifier '{token}-': Use 'alt-' instead"),
                        span: (0, 4).into(),
                    });
                }
                // NOTE: in Helix it is `C`
                "ctrl" => KeyModifiers::CONTROL,
                "C" => {
                    return Err(ParseKeysError {
                        src: NamedSource::new(filename, keys_str.to_string()),
                        error: format!("Invalid key modifier '{token}-': Use 'ctrl-' instead"),
                        span: (0, 4).into(),
                    });
                }
                "Meta" | "Cmd" | "Win" => KeyModifiers::SUPER,
                _ => {
                    return Err(ParseKeysError {
                        src: NamedSource::new(filename, keys_str.to_string()),
                        error: format!("Invalid key modifier '{token}-'"),
                        span: (0, 4).into(),
                    });
                }
            };

            if modifiers.contains(flag) {
                return Err(ParseKeysError {
                    src: NamedSource::new(filename, keys_str.to_string()),
                    error: format!("Repeated key modifier '{token}-'"),
                    span: (0, 4).into(),
                });
            }
            modifiers.insert(flag);
        }

        // Normalize character keys so that characters like C-S-r and C-R
        // are represented by equal KeyEvents.
        match code {
            KeyCode::Char(ch)
                if ch.is_ascii_lowercase() && modifiers.contains(KeyModifiers::SHIFT) =>
            {
                code = KeyCode::Char(ch.to_ascii_uppercase());
                modifiers.remove(KeyModifiers::SHIFT);
            }
            _ => (),
        }

        Ok(KeyEvent { code, modifiers })
    }
}

/// Represents a key event.
// We use a newtype here because we want to customize Deserialize and Display.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
    // TODO: crossterm now supports kind & state if terminal supports kitty's extended protocol
}

/// Represents a key.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyCode {
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page down key.
    PageDown,
    /// Tab key.
    Tab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// F key.
    ///
    /// `KeyCode::F(1)` represents F1 key, etc.
    F(u8),
    /// A character.
    ///
    /// `KeyCode::Char('c')` represents `c` character, etc.
    Char(char),
    /// Null.
    Null,
    /// Escape key.
    Esc,
    /// CapsLock key.
    CapsLock,
    /// ScrollLock key.
    ScrollLock,
    /// NumLock key.
    NumLock,
    /// PrintScreen key.
    PrintScreen,
    /// Pause key.
    Pause,
    /// Menu key.
    Menu,
    /// KeypadBegin key.
    KeypadBegin,
    /// A media key.
    Media(MediaKeyCode),
    /// A modifier key.
    Modifier(ModifierKeyCode),
}

/// Represents a media key (as part of [`KeyCode::Media`]).
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MediaKeyCode {
    /// Play media key.
    Play,
    /// Pause media key.
    Pause,
    /// Play/Pause media key.
    PlayPause,
    /// Reverse media key.
    Reverse,
    /// Stop media key.
    Stop,
    /// Fast-forward media key.
    FastForward,
    /// Rewind media key.
    Rewind,
    /// Next-track media key.
    TrackNext,
    /// Previous-track media key.
    TrackPrevious,
    /// Record media key.
    Record,
    /// Lower-volume media key.
    LowerVolume,
    /// Raise-volume media key.
    RaiseVolume,
    /// Mute media key.
    MuteVolume,
}

/// Represents a media key (as part of [`KeyCode::Modifier`]).
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ModifierKeyCode {
    /// Left Shift key.
    LeftShift,
    /// Left Control key.
    LeftControl,
    /// Left Alt key.
    LeftAlt,
    /// Left Super key.
    LeftSuper,
    /// Left Hyper key.
    LeftHyper,
    /// Left Meta key.
    LeftMeta,
    /// Right Shift key.
    RightShift,
    /// Right Control key.
    RightControl,
    /// Right Alt key.
    RightAlt,
    /// Right Super key.
    RightSuper,
    /// Right Hyper key.
    RightHyper,
    /// Right Meta key.
    RightMeta,
    /// Iso Level3 Shift key.
    IsoLevel3Shift,
    /// Iso Level5 Shift key.
    IsoLevel5Shift,
}

bitflags! {
    /// Represents key modifiers (shift, control, alt).
    #[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const SUPER = 0b0000_1000;
        const NONE = 0b0000_0000;
    }
}

pub(crate) mod keys {
    pub(crate) const BACKSPACE: &str = "backspace";
    // NOTE: In Helix, it is "ret"
    pub(crate) const ENTER: &str = "enter";
    pub(crate) const LEFT: &str = "left";
    pub(crate) const RIGHT: &str = "right";
    pub(crate) const UP: &str = "up";
    pub(crate) const DOWN: &str = "down";
    pub(crate) const HOME: &str = "home";
    pub(crate) const END: &str = "end";
    pub(crate) const PAGEUP: &str = "pageup";
    pub(crate) const PAGEDOWN: &str = "pagedown";
    pub(crate) const TAB: &str = "tab";
    pub(crate) const DELETE: &str = "del";
    pub(crate) const INSERT: &str = "ins";
    pub(crate) const NULL: &str = "null";
    pub(crate) const ESC: &str = "esc";
    pub(crate) const SPACE: &str = "space";
    pub(crate) const MINUS: &str = "minus";
    pub(crate) const LESS_THAN: &str = "lt";
    pub(crate) const GREATER_THAN: &str = "gt";
    pub(crate) const CAPS_LOCK: &str = "capslock";
    pub(crate) const SCROLL_LOCK: &str = "scrolllock";
    pub(crate) const NUM_LOCK: &str = "numlock";
    pub(crate) const PRINT_SCREEN: &str = "printscreen";
    pub(crate) const PAUSE: &str = "pause";
    pub(crate) const MENU: &str = "menu";
    pub(crate) const KEYPAD_BEGIN: &str = "keypadbegin";
    pub(crate) const PLAY: &str = "play";
    pub(crate) const PAUSE_MEDIA: &str = "pausemedia";
    pub(crate) const PLAY_PAUSE: &str = "playpause";
    pub(crate) const REVERSE: &str = "reverse";
    pub(crate) const STOP: &str = "stop";
    pub(crate) const FAST_FORWARD: &str = "fastforward";
    pub(crate) const REWIND: &str = "rewind";
    pub(crate) const TRACK_NEXT: &str = "tracknext";
    pub(crate) const TRACK_PREVIOUS: &str = "trackprevious";
    pub(crate) const RECORD: &str = "record";
    pub(crate) const LOWER_VOLUME: &str = "lowervolume";
    pub(crate) const RAISE_VOLUME: &str = "raisevolume";
    pub(crate) const MUTE_VOLUME: &str = "mutevolume";
    pub(crate) const LEFT_SHIFT: &str = "leftshift";
    pub(crate) const LEFT_CONTROL: &str = "leftcontrol";
    pub(crate) const LEFT_ALT: &str = "leftalt";
    pub(crate) const LEFT_SUPER: &str = "leftsuper";
    pub(crate) const LEFT_HYPER: &str = "lefthyper";
    pub(crate) const LEFT_META: &str = "leftmeta";
    pub(crate) const RIGHT_SHIFT: &str = "rightshift";
    pub(crate) const RIGHT_CONTROL: &str = "rightcontrol";
    pub(crate) const RIGHT_ALT: &str = "rightalt";
    pub(crate) const RIGHT_SUPER: &str = "rightsuper";
    pub(crate) const RIGHT_HYPER: &str = "righthyper";
    pub(crate) const RIGHT_META: &str = "rightmeta";
    pub(crate) const ISO_LEVEL_3_SHIFT: &str = "isolevel3shift";
    pub(crate) const ISO_LEVEL_5_SHIFT: &str = "isolevel5shift";
}
