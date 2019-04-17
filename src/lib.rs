//! Truncate and ellipse strings in a human-friendly way.
//!
//! Uses unicode extended grapheme clusters.
//!
//! # Example
//! ```
//! use ellipse::Ellipse;
//!
//! let input = "🇩🇪🇬🇧🇮🇹🇫🇷";
//! assert_eq!(&input.truncate_ellipse(2), "🇩🇪🇬🇧...");
//! ```

use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;

/// Truncate and ellipse strings in a human-friendly way.
///
/// # Example
/// ```
/// use ellipse::Ellipse;
///
/// let input = "🇩🇪🇬🇧🇮🇹🇫🇷";
/// assert_eq!(&input.truncate_ellipse(2), "🇩🇪🇬🇧...");
/// ```
pub trait Ellipse {
    type Output;

    /// Truncate to a length of `len` extended grapheme clusters.
    ///
    /// Truncating to a length of 0 will yield the empty element without an
    /// attached ellipsis.
    fn truncate_ellipse(&self, len: usize) -> Self::Output;
}

impl<'a> Ellipse for &'a str {
    type Output = Cow<'a, str>;

    fn truncate_ellipse(&self, len: usize) -> Self::Output {
        if self.graphemes(true).count() <= len {
            return Cow::Borrowed(self);
        } else if len == 0 {
            return Cow::Borrowed("");
        }

        let result = self
            .graphemes(true)
            .take(len)
            .chain("...".graphemes(true))
            .collect();
        Cow::Owned(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        assert_eq!(&"Hello, World!".truncate_ellipse(3), "Hel...",);
    }

    #[test]
    fn weird_chars() {
        assert_eq!(
            &"🇩🇪🇬🇧🇮🇹🇫🇷".truncate_ellipse(2),
            "🇩🇪🇬🇧..."
        );
    }

    #[test]
    fn truncate_to_empty() {
        assert_eq!(&"Hello, World!".truncate_ellipse(0), "");
    }
}
