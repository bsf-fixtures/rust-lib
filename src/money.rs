//! Money as integer cents.
//!
//! Never a float. A cent is the smallest unit anyone is billed in, and
//! `0.1 + 0.2` is the reason this type exists.

/// An amount in cents. Signed, because a credit note is negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cents(pub i64);

impl Cents {
    pub const ZERO: Cents = Cents(0);

    /// Parse `"12.34"` into cents, rejecting anything that is not money.
    ///
    /// Returns `None` rather than panicking: the input comes from a file or a
    /// form, so a bad value is an ordinary event, not a bug in the caller.
    pub fn parse(text: &str) -> Option<Cents> {
        let text = text.trim();
        let (sign, rest) = match text.strip_prefix('-') {
            Some(rest) => (-1, rest),
            None => (1, text),
        };
        let (whole, frac) = match rest.split_once('.') {
            Some((w, f)) => (w, f),
            None => (rest, "00"),
        };
        if whole.is_empty() || !whole.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        if frac.len() != 2 || !frac.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let whole: i64 = whole.parse().ok()?;
        let frac: i64 = frac.parse().ok()?;
        whole.checked_mul(100)?.checked_add(frac).map(|n| Cents(sign * n))
    }

    /// Apply a percentage in basis points, rounding half away from zero.
    ///
    /// Half away from zero rather than Rust's default truncation: a tax line
    /// that always rounds down loses a cent on roughly half of all invoices,
    /// and the loss is systematic rather than averaging out.
    pub fn basis_points(self, bps: i64) -> Cents {
        let numerator = self.0 * bps;
        let half = if numerator >= 0 { 5_000 } else { -5_000 };
        Cents((numerator + half) / 10_000)
    }

    pub fn to_string_amount(self) -> String {
        let sign = if self.0 < 0 { "-" } else { "" };
        let abs = self.0.abs();
        format!("{sign}{}.{:02}", abs / 100, abs % 100)
    }
}

impl std::ops::Add for Cents {
    type Output = Cents;
    fn add(self, other: Cents) -> Cents {
        Cents(self.0 + other.0)
    }
}

impl std::iter::Sum for Cents {
    fn sum<I: Iterator<Item = Cents>>(iter: I) -> Cents {
        iter.fold(Cents::ZERO, |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::Cents;

    #[test]
    fn parses_an_ordinary_amount() {
        assert_eq!(Cents::parse("12.34"), Some(Cents(1234)));
        assert_eq!(Cents::parse("0.05"), Some(Cents(5)));
        assert_eq!(Cents::parse("-3.00"), Some(Cents(-300)));
    }

    #[test]
    fn refuses_anything_that_is_not_money() {
        for bad in ["", "abc", "1.5", "1.234", "1,00", "."] {
            assert_eq!(Cents::parse(bad), None, "{bad} should not parse");
        }
    }

    #[test]
    fn rounds_half_away_from_zero() {
        // 7.5 cents of tax is 8, not 7: always rounding down loses a cent on
        // half of all invoices, and the loss does not average out.
        assert_eq!(Cents(100).basis_points(750), Cents(8));
        assert_eq!(Cents(-100).basis_points(750), Cents(-8));
    }

    #[test]
    fn formats_with_two_places() {
        assert_eq!(Cents(5).to_string_amount(), "0.05");
        assert_eq!(Cents(-1234).to_string_amount(), "-12.34");
    }
}
