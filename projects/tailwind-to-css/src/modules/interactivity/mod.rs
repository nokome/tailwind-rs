use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
};

use crate::{
    css_attributes, CssAttributes, Negative, Result, StandardValue, TailwindArbitrary, TailwindBuilder, TailwindColor,
    TailwindInstance, TailwindScrollMargin, TailwindScrollPadding,
};

pub(crate) use self::snap::snap_adaptor;
pub use self::{
    accent::TailwindAccentColor,
    appearance::TailwindAppearance,
    caret::TailwindCaretColor,
    cursor::TailwindCursor,
    pointer::TailwindPointerEvents,
    resize::TailwindResize,
    scroll::TailwindScroll,
    select::TailwindSelect,
    snap::{snap_align::TailwindSnapAlign, snap_stop::TailwindSnapStop, snap_type::TailwindSnapType},
    torch::TailwindTorch,
    will_change::TailwindWillChange,
};

mod accent;
mod appearance;
mod caret;
mod cursor;
mod pointer;
mod resize;
mod scroll;
mod select;
mod snap;
#[cfg(test)]
mod test;
mod torch;
mod will_change;
