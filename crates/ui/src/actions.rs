use gpui::{actions, Action, SharedString};
use serde::Deserialize;

#[derive(Clone, Action, PartialEq, Eq, Deserialize)]
#[action(namespace = ui, no_json)]
pub struct Confirm {
    /// Is confirm with secondary.
    pub secondary: bool,
}

/// Write a specific string to the clipboard.
///
/// Carries its own text so a menu item can copy a value captured when the menu
/// opened, independent of any later selection state. Deliberately has no key
/// binding, so menus that dispatch it show no (potentially wrong) accelerator.
#[derive(Clone, Action, PartialEq, Eq, Deserialize)]
#[action(namespace = ui, no_json)]
pub struct CopyText {
    /// The text to write to the clipboard.
    pub text: SharedString,
}

actions!(ui, [Cancel, SelectUp, SelectDown, SelectLeft, SelectRight, SelectFirst, SelectLast, SelectPrevColumn, SelectNextColumn, SelectPageUp, SelectPageDown, CopySelection]);

