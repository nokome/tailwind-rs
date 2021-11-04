use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindPadding {
    negative: bool,
    axis: SpacingAxis,
    size: SpacingSize,
}

// noinspection DuplicatedCode
impl Display for TailwindPadding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.negative {
            f.write_char('-')?
        }
        write!(f, "{}-{}", self.axis, self.size)
    }
}

// noinspection DuplicatedCode
impl TailwindInstance for TailwindPadding {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let mut out = BTreeSet::new();
        self.axis.write_attributes(&mut out, self.size.get_properties());
        out
    }
}

// noinspection DuplicatedCode
impl TailwindPadding {
    /// https://tailwindcss.com/docs/margin
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, axis: &str, negative: bool) -> Result<Self> {
        let axis = match pattern {
            ["m"] => SpacingAxis { class: "m", attributes: &["margin"] },
            ["mx"] => SpacingAxis { class: "mx", attributes: &["margin-left", "margin-right"] },
            ["my"] => SpacingAxis { class: "my", attributes: &["margin-top", "margin-bottom"] },
            ["ml"] => SpacingAxis { class: "ml", attributes: &["margin-left"] },
            ["mr"] => SpacingAxis { class: "mr", attributes: &["margin-right"] },
            ["mt"] => SpacingAxis { class: "mt", attributes: &["margin-top"] },
            ["mb"] => SpacingAxis { class: "mb", attributes: &["margin-bottom"] },
            _ => return syntax_error!("Unknown margin axis"),
        };
        let size = SpacingSize::parse(pattern, arbitrary)?;
        Ok(Self { negative, axis, size })
    }
    /// https://tailwindcss.com/docs/margin#arbitrary-values
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary, axis: SpacingAxis, negative: bool) -> Result<Self> {
        let size = SpacingSize::parse_arbitrary(arbitrary)?;
        Ok(Self { negative, axis, size })
    }
}
