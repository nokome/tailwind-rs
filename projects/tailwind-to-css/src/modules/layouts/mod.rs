pub use self::{aspect_ratio::TailwindAspect, columns::TailwindColumns, container::TailwindContainer, z_index::TailWindZIndex};
use crate::{
    css_attributes, syntax_error, CssAttribute, CssBehavior, LengthUnit, Result, TailwindArbitrary, TailwindBuilder,
    TailwindInstance,
};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display, Formatter},
};

mod aspect_ratio;
mod columns;
mod container;
mod display;
mod parser;
#[cfg(test)]
mod test;
mod z_index;

#[derive(Copy, Clone, Debug)]
enum BreakKind {
    Before,
    After,
    Inside,
}

#[doc = include_str!("break.md")]
#[derive(Clone, Debug)]
pub struct TailwindBreakLayout {
    kind: BreakKind,
    info: String,
}

#[derive(Copy, Clone, Debug)]
enum BoxDecoration {
    Clone,
    Slice,
}

#[doc = include_str!("box-decoration-break.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindBoxDecoration {
    kind: BoxDecoration,
}

#[derive(Copy, Clone, Debug)]
enum BoxSizing {
    Border,
    Content,
}

#[doc = include_str!("box-sizing.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindBoxSizing {
    kind: BoxSizing,
}

/// https://tailwindcss.com/docs/display
#[derive(Copy, Clone, Debug)]
pub enum TailwindDisplay {
    Block,
    Inline,
    InlineBlock,
    Flex,
    InlineFlex,
    Table,
    InlineTable,
    TableCaption,
}

#[derive(Copy, Clone, Debug)]
pub enum FloatKind {
    Left,
    Right,
    None,
}

#[doc = include_str!("float.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindFloat {
    kind: FloatKind,
}

#[derive(Copy, Clone, Debug)]
enum ClearKind {
    Left,
    Right,
    Both,
    None,
}

#[doc = include_str!("clear.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindClear {
    kind: ClearKind,
}

#[derive(Copy, Clone, Debug)]
enum Isolation {
    Isolate,
    Auto,
}

///
#[derive(Copy, Clone, Debug)]
pub struct TailwindIsolation {
    kind: Isolation,
}

#[derive(Copy, Clone, Debug)]
enum ObjectFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}

/// https://tailwindcss.com/docs/object-fit
#[derive(Copy, Clone, Debug)]
pub struct TailwindObjectFit {
    kind: ObjectFit,
}

/// https://tailwindcss.com/docs/object-fit
#[derive(Clone, Debug)]
pub enum TailwindObjectPosition {
    LeftTop,
    Top,
    RightTop,
    Left,
    Center,
    Right,
    LeftBottom,
    Bottom,
    RightBottom,
    Custom { x: String, y: String },
}

#[derive(Copy, Clone, Debug)]
enum Overflow {
    Auto,
    Hidden,
    Clip,
    Visible,
    Scroll,
}

/// https://tailwindcss.com/docs/overflow#hiding-content-that-overflows
#[derive(Copy, Clone, Debug)]
pub struct TailwindOverflow {
    kind: Overflow,
    axis: Option<bool>,
}

#[derive(Copy, Clone, Debug)]
enum Overscroll {
    Auto,
    Contain,
    None,
}

/// https://tailwindcss.com/docs/overscroll-behavior
#[derive(Copy, Clone, Debug)]
pub struct TailwindOverscroll {
    kind: Overscroll,
    axis: Option<bool>,
}

#[derive(Copy, Clone, Debug)]
enum PositionKind {
    Static,
    Fixed,
    Absolute,
    Relative,
    Sticky,
}

///
// #[doc = include_str!("position.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindPosition {
    kind: PositionKind,
}

#[derive(Copy, Clone, Debug)]
enum Visibility {
    Visible,
    Invisible,
}

/// https://tailwindcss.com/docs/visibility
#[derive(Copy, Clone, Debug)]
pub struct TailwindVisibility {
    kind: Visibility,
}
