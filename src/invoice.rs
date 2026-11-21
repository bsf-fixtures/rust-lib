//! An invoice: lines, tax, total.

use crate::money::Cents;

#[derive(Debug, Clone)]
pub struct Line {
    pub description: String,
    pub quantity: u32,
    pub unit_price: Cents,
}

impl Line {
    pub fn subtotal(&self) -> Cents {
        Cents(self.unit_price.0 * i64::from(self.quantity))
    }
}

#[derive(Debug, Clone)]
pub struct Invoice {
    pub number: String,
    pub lines: Vec<Line>,
    /// Tax rate in basis points: 2000 is 20%.
    pub tax_bps: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Total {
    pub net: Cents,
    pub tax: Cents,
    pub gross: Cents,
}

impl Invoice {
    pub fn total(&self) -> Total {
        let net: Cents = self.lines.iter().map(Line::subtotal).sum();
        // Tax on the net total, not line by line: rounding each line
        // separately and adding them up gives a different number, and the
        // one people check against is the total.
        let tax = net.basis_points(self.tax_bps);
        Total { net, tax, gross: net + tax }
    }
}

#[cfg(test)]
mod tests {
    use super::{Invoice, Line};
    use crate::money::Cents;

    fn line(quantity: u32, unit: i64) -> Line {
        Line { description: "work".into(), quantity, unit_price: Cents(unit) }
    }

    #[test]
    fn adds_the_lines_and_the_tax() {
        let invoice = Invoice { number: "INV-1".into(), lines: vec![line(2, 1000), line(1, 550)], tax_bps: 2000 };
        let total = invoice.total();
        assert_eq!(total.net, Cents(2550));
        assert_eq!(total.tax, Cents(510));
        assert_eq!(total.gross, Cents(3060));
    }

    #[test]
    fn taxes_the_total_rather_than_each_line() {
        // Three lines of 0.01 at 50%: taxing each gives 1 cent (0.005 rounded
        // up, three times); taxing the total of 0.03 gives 2. The total is
        // what the customer checks.
        let invoice = Invoice {
            number: "INV-2".into(),
            lines: vec![line(1, 1), line(1, 1), line(1, 1)],
            tax_bps: 5000,
        };
        assert_eq!(invoice.total().tax, Cents(2));
    }

    #[test]
    fn an_empty_invoice_totals_zero() {
        let invoice = Invoice { number: "INV-3".into(), lines: vec![], tax_bps: 2000 };
        assert_eq!(invoice.total().gross, Cents::ZERO);
    }
}
