use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use pax_designtime::DesigntimeManager;
use pax_engine::api::{borrow, borrow_mut, Fill, Interpolatable, Stroke};
use pax_engine::pax_manifest::{UniqueTemplateNodeIdentifier, ValueDefinition};
use pax_engine::{log, CoercionRules, Property};

use crate::model::action::orm::{RedoRequested, SerializeRequested, UndoRequested};
use crate::{controls::toolbar, glass, llm_interface::OpenLLMPrompt};

use super::action::orm::group_ungroup::{GroupSelected, GroupType};
use super::action::orm::other::SwapFillStrokeAction;
use super::action::orm::{Copy, Paste};
use super::read_app_state;
use super::{
    action::{self, orm::DeleteSelected, world, Action, ActionContext},
    Component, Tool,
};

impl Interpolatable for InputMapper {}

#[derive(Clone)]
pub struct InputMapper {
    modifier_map: HashMap<RawInput, ModifierKey>,
    key_map: Vec<((RawInput, HashSet<ModifierKey>), InputEvent)>,
}

impl Default for InputMapper {
    fn default() -> Self {
        Self {
            // Modifer key map. Most likely platform specific at some point,
            // might be configuratble in settings?
            modifier_map: HashMap::from([
                (RawInput::Control, ModifierKey::Control),
                (RawInput::Meta, ModifierKey::Meta),
                (RawInput::Shift, ModifierKey::Shift),
                (RawInput::Alt, ModifierKey::Alt),
                (RawInput::Z, ModifierKey::Z),
                (RawInput::Space, ModifierKey::Space),
            ]),
            //Default keymap, will be configurable in settings
            key_map: [
                // --- Select tools ---
                // rectangle
                (
                    (RawInput::R, HashSet::new()),
                    InputEvent::SelectTool(Tool::CreateComponent(Component::Rectangle)),
                ),
                (
                    (RawInput::M, HashSet::new()),
                    InputEvent::SelectTool(Tool::CreateComponent(Component::Rectangle)),
                ),
                // ellipse
                (
                    (RawInput::O, HashSet::new()),
                    InputEvent::SelectTool(Tool::CreateComponent(Component::Ellipse)),
                ),
                (
                    (RawInput::E, HashSet::new()),
                    InputEvent::SelectTool(Tool::CreateComponent(Component::Ellipse)),
                ),
                // pointers
                (
                    (RawInput::V, HashSet::new()),
                    InputEvent::SelectTool(Tool::PointerPercent),
                ),
                (
                    (RawInput::V, HashSet::new()),
                    InputEvent::SelectTool(Tool::PointerPixels),
                ),
                // text
                (
                    (RawInput::T, HashSet::new()),
                    InputEvent::SelectTool(Tool::CreateComponent(Component::Text)),
                ),
                (
                    (RawInput::T, HashSet::from([ModifierKey::Shift])),
                    InputEvent::SelectTool(Tool::CreateComponent(Component::Textbox)),
                ),
                // button
                (
                    (RawInput::B, HashSet::new()),
                    InputEvent::SelectTool(Tool::CreateComponent(Component::Button)),
                ),
                // -- Group/ungroup ops
                (
                    (RawInput::L, HashSet::from([ModifierKey::Meta])),
                    InputEvent::Group(GroupType::Link),
                ),
                (
                    (RawInput::G, HashSet::from([ModifierKey::Meta])),
                    InputEvent::Group(GroupType::Group),
                ),
                // --- Copy/Paste ---
                (
                    (RawInput::C, HashSet::from([ModifierKey::Meta])),
                    InputEvent::Copy,
                ),
                (
                    (RawInput::V, HashSet::from([ModifierKey::Meta])),
                    InputEvent::Paste,
                ),
                // --- Undo and Redo ---
                (
                    (RawInput::Z, HashSet::from([ModifierKey::Meta])),
                    InputEvent::Undo,
                ),
                (
                    (
                        RawInput::Z,
                        HashSet::from([ModifierKey::Meta, ModifierKey::Shift]),
                    ),
                    InputEvent::Redo,
                ),
                // --- Serialize/save ---
                ((RawInput::S, HashSet::new()), InputEvent::Serialize),
                // --- Zoom ---
                (
                    (RawInput::Plus, HashSet::from([ModifierKey::Meta])),
                    InputEvent::ZoomIn,
                ),
                (
                    (RawInput::Minus, HashSet::from([ModifierKey::Meta])),
                    InputEvent::ZoomOut,
                ),
                // --- LLM Prompt ---
                (
                    (RawInput::K, HashSet::from([ModifierKey::Meta])),
                    InputEvent::OpenLLMPrompt,
                ),
                // --- Deletion ---
                (
                    (RawInput::Delete, HashSet::new()),
                    InputEvent::DeleteSelected,
                ),
                (
                    (RawInput::Backspace, HashSet::new()),
                    InputEvent::DeleteSelected,
                ),
                // --- Util ---
                (
                    (RawInput::X, HashSet::from([ModifierKey::Shift])),
                    InputEvent::SwapFillStroke,
                ),
            ]
            .to_vec(),
        }
    }
}

impl InputMapper {
    pub fn to_event(
        &self,
        input: RawInput,
        dir: Dir,
        modifiers: Property<HashSet<ModifierKey>>,
    ) -> Option<&InputEvent> {
        if let Some(modifier) = self.modifier_map.get(&input) {
            modifiers.update(|modifiers| {
                match dir {
                    Dir::Down => modifiers.insert(*modifier),
                    Dir::Up => modifiers.remove(modifier),
                };
            });
        }
        let modifiers = modifiers.get();
        // find the key combination that matches all required keys,
        // and contains the largest number of required keys
        self.key_map
            .iter()
            .filter(|((i, m), _)| i == &input && m.is_subset(&modifiers))
            .max_by_key(|((_, m), _)| m.len())
            .map(|(_, v)| v)
    }

    pub fn to_action(&self, event: &InputEvent, dir: Dir) -> Option<Box<dyn Action>> {
        match (event, dir) {
            (&InputEvent::Group(group_type), Dir::Down) => {
                Some(Box::new(GroupSelected { group_type }))
            }
            (&InputEvent::SelectTool(tool), Dir::Down) => {
                Some(Box::new(toolbar::SelectTool { tool }))
            }
            (&InputEvent::ZoomIn, Dir::Down) => Some(Box::new(world::Zoom { closer: true })),
            (&InputEvent::ZoomOut, Dir::Down) => Some(Box::new(world::Zoom { closer: false })),
            (&InputEvent::OpenLLMPrompt, Dir::Down) => Some(Box::new(OpenLLMPrompt)),
            (&InputEvent::DeleteSelected, Dir::Down) => Some(Box::new(DeleteSelected {})),
            (&InputEvent::Undo, Dir::Down) => Some(Box::new(UndoRequested)),
            (&InputEvent::Redo, Dir::Down) => Some(Box::new(RedoRequested)),
            (&InputEvent::Serialize, Dir::Down) => Some(Box::new(SerializeRequested {})),
            (&InputEvent::Copy, Dir::Down) => Some(Box::new({
                struct CopySelected;
                impl Action for CopySelected {
                    fn perform(&self, ctx: &mut ActionContext) -> Result<()> {
                        let ids = ctx.app_state.selected_template_node_ids.get();
                        let subtrees = Copy { ids: &ids }.perform(ctx)?;
                        ctx.app_state.clip_board.set(subtrees);
                        Ok(())
                    }
                }
                CopySelected
            })),
            (&InputEvent::Paste, Dir::Down) => Some(Box::new({
                struct PasteClipboard;

                impl Action for PasteClipboard {
                    fn perform(&self, ctx: &mut ActionContext) -> Result<()> {
                        let t = ctx.transaction("paste");
                        let subtrees = ctx.app_state.clip_board.get();
                        t.run(|| {
                            Paste {
                                subtrees: &subtrees,
                            }
                            .perform(ctx)
                        })
                        .map(|_| ())
                    }
                }
                PasteClipboard
            })),
            (InputEvent::SwapFillStroke, Dir::Down) => Some(Box::new(SwapFillStrokeAction)),
            (InputEvent::SwapFillStroke, Dir::Up)
            | (InputEvent::SelectTool(_), Dir::Up)
            | (InputEvent::Space, Dir::Down)
            | (InputEvent::Space, Dir::Up)
            | (InputEvent::ZoomIn, Dir::Up)
            | (InputEvent::ZoomOut, Dir::Up)
            | (InputEvent::OpenLLMPrompt, Dir::Up)
            | (InputEvent::DeleteSelected, Dir::Up)
            | (InputEvent::Undo, Dir::Up)
            | (InputEvent::Redo, Dir::Up)
            | (InputEvent::Serialize, Dir::Up)
            | (InputEvent::Copy, Dir::Up)
            | (InputEvent::Paste, Dir::Up)
            | (InputEvent::ToggleLinkGroup, Dir::Down)
            | (InputEvent::ToggleLinkGroup, Dir::Up)
            | (InputEvent::Group(_), Dir::Up) => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Down,
    Up,
}

// This represents the actual input performed by the user
// to be rebindable in a settingsview
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RawInput {
    Delete,
    Backspace,
    R,
    V,
    C,
    Control,
    Space,
    Plus,
    Minus,
    Alt,
    Z,
    Meta,
    Shift,
    K,
    S,
    M,
    O,
    E,
    T,
    B,
    L,
    G,
    X,
}

// TODO make RawInput be what is returned by the engine itself, instead
// of performing conversion here
impl TryFrom<String> for RawInput {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Ok(match value.to_lowercase().as_str() {
            "r" => Self::R,
            "m" => Self::M,
            "v" => Self::V,
            "z" => Self::Z,
            "k" => Self::K,
            "s" => Self::S,
            "c" => Self::C,
            "o" => Self::O,
            "e" => Self::E,
            "t" => Self::T,
            "b" => Self::B,
            "l" => Self::L,
            "g" => Self::G,
            "x" => Self::X,
            " " => Self::Space,
            "control" => Self::Control,
            "=" => Self::Plus,
            "-" => Self::Minus,
            "alt" => Self::Alt,
            "meta" => Self::Meta,
            "shift" => Self::Shift,
            "delete" => Self::Delete,
            "backspace" => Self::Backspace,
            _ => return Err(anyhow!("no configured raw input mapping for {:?}", value)),
        })
    }
}

impl Interpolatable for InputEvent {}
// This represents the "actions" that can be taken by the user that could
// potentially be remapped to arbitrary keys. Only user input events: no
// internal message passing to be done using these tupes
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum InputEvent {
    SelectTool(Tool),
    Space,
    ZoomIn,
    ZoomOut,
    OpenLLMPrompt,
    DeleteSelected,
    Undo,
    Redo,
    Serialize,
    Copy,
    Paste,
    ToggleLinkGroup,
    Group(GroupType),
    SwapFillStroke,
}

impl Interpolatable for ModifierKey {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy)]
pub enum ModifierKey {
    Control,
    Alt,
    Shift,
    Meta,
    Space,
    // "zoom mode"
    Z,
}
