/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 16/8/24
******************************************************************************/
use crate::Positive;
use crate::model::Trade;
pub use crate::pnl::PnLCalculator;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter::Sum;
use std::ops::Add;

/// Represents the Profit and Loss (PnL) of a financial instrument.
///
/// This structure captures the financial performance details of an investment or trading position,
/// including both realized and unrealized gains or losses, as well as the initial costs and income
/// associated with the position.
///
/// PnL serves as a fundamental measurement of trading performance, providing a comprehensive view
/// of the current financial status of positions. It is particularly useful for options trading,
/// portfolio management, and financial reporting.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PnL {
    /// The realized profit or loss that has been crystallized through closed positions.
    /// This represents actual gains or losses that have been confirmed by completing the trade.
    pub realized: Option<Decimal>,

    /// The unrealized profit or loss representing the current market value compared to entry price.
    /// This value fluctuates with market movements and represents potential gains or losses if
    /// the position were to be closed at current market prices.
    pub unrealized: Option<Decimal>,

    /// The initial costs associated with entering the position, such as fees, commissions,
    /// or premiums paid when buying options.
    pub initial_costs: Positive,

    /// The initial income received when entering the position, such as premiums collected
    /// when selling options or other upfront payments received.
    pub initial_income: Positive,

    /// The timestamp when this PnL calculation was performed.
    /// Useful for tracking performance over time and creating historical PnL reports.
    pub date_time: DateTime<Utc>,
}

impl PnL {
    /// Creates a new Profit and Loss (PnL) instance.
    ///
    /// This constructor initializes a new PnL object with information about the financial
    /// performance of a trading position, including both realized and unrealized components.
    ///
    /// # Parameters
    ///
    /// * `realized` - The confirmed profit or loss from closed positions, if available.
    ///   This represents actual gains or losses that have been crystallized through completed trades.
    ///
    /// * `unrealized` - The potential profit or loss based on current market values, if available.
    ///   This value represents the theoretical gain or loss if the position were closed at current prices.
    ///
    /// * `initial_costs` - The costs associated with entering the position, such as premiums paid,
    ///   commissions, or fees. Always represented as a positive value.
    ///
    /// * `initial_income` - The income received when entering the position, such as premiums
    ///   collected when selling options. Always represented as a positive value.
    ///
    /// * `date_time` - The timestamp when this PnL calculation was performed, useful for
    ///   tracking performance over time and creating historical reports.
    ///
    /// # Returns
    ///
    /// A new `PnL` instance containing the provided financial performance data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chrono::Utc;
    /// use rust_decimal_macros::dec;
    /// use optionstratlib::pnl::utils::PnL;
    /// use optionstratlib::pos;
    ///
    /// let pnl = PnL::new(
    ///     Some(dec!(500.0)),  // Realized PnL
    ///     Some(dec!(250.0)),  // Unrealized PnL
    ///     pos!(100.0),        // Initial costs
    ///     pos!(350.0),        // Initial income
    ///     Utc::now(),         // Current timestamp
    /// );
    /// ```
    pub fn new(
        realized: Option<Decimal>,
        unrealized: Option<Decimal>,
        initial_costs: Positive,
        initial_income: Positive,
        date_time: DateTime<Utc>,
    ) -> Self {
        PnL {
            realized,
            unrealized,
            initial_costs,
            initial_income,
            date_time,
        }
    }
}

impl fmt::Display for PnL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Helper function to format Option<Decimal> with rounding
        fn format_decimal(value: &Option<Decimal>) -> String {
            match value {
                Some(dec) => dec.round_dp(3).to_string(),
                None => "N/A".to_string(),
            }
        }

        write!(
            f,
            "PnL: {{ realized: {}, unrealized: {}, initial_costs: {}, initial_income: {}, date_time: {} }}",
            format_decimal(&self.realized),
            format_decimal(&self.unrealized),
            self.initial_costs.round_to(3),
            self.initial_income.round_to(3),
            self.date_time
        )
    }
}

impl Sum for PnL {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(PnL::default(), |acc, x| PnL {
            realized: match (acc.realized, x.realized) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            unrealized: match (acc.unrealized, x.unrealized) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            initial_costs: acc.initial_costs + x.initial_costs,
            initial_income: acc.initial_income + x.initial_income,
            date_time: x.date_time, // Tomamos la fecha más reciente
        })
    }
}

impl<'a> Sum<&'a PnL> for PnL {
    fn sum<I: Iterator<Item = &'a PnL>>(iter: I) -> Self {
        iter.fold(PnL::default(), |acc, x| PnL {
            realized: match (acc.realized, x.realized) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            unrealized: match (acc.unrealized, x.unrealized) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            initial_costs: acc.initial_costs + x.initial_costs,
            initial_income: acc.initial_income + x.initial_income,
            date_time: x.date_time, // Tomamos la fecha más reciente
        })
    }
}

impl Add for PnL {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        PnL {
            realized: match (self.realized, other.realized) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            unrealized: match (self.unrealized, other.unrealized) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            initial_costs: self.initial_costs + other.initial_costs,
            initial_income: self.initial_income + other.initial_income,
            date_time: if self.date_time > other.date_time {
                self.date_time
            } else {
                other.date_time
            },
        }
    }
}

impl Add for &PnL {
    type Output = PnL;

    fn add(self, other: Self) -> PnL {
        PnL {
            realized: match (self.realized, other.realized) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            unrealized: match (self.unrealized, other.unrealized) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
            initial_costs: self.initial_costs + other.initial_costs,
            initial_income: self.initial_income + other.initial_income,
            date_time: if self.date_time > other.date_time {
                self.date_time
            } else {
                other.date_time
            },
        }
    }
}

impl From<Trade> for PnL {
    fn from(value: Trade) -> Self {
        PnL {
            realized: Some(value.net()),
            unrealized: None,
            initial_costs: value.cost(),
            initial_income: value.income(),
            date_time: value.datetime(),
        }
    }
}

impl From<&Trade> for PnL {
    fn from(value: &Trade) -> Self {
        PnL {
            realized: Some(value.net()),
            unrealized: None,
            initial_costs: value.cost(),
            initial_income: value.income(),
            date_time: value.datetime(),
        }
    }
}

#[cfg(test)]
mod tests_sum {
    use super::*;
    use crate::pos;
    use rust_decimal_macros::dec;

    #[test]
    fn test_pnl_sum() {
        let pnl1 = PnL {
            realized: Some(dec!(10.0)),
            unrealized: Some(dec!(5.0)),
            initial_costs: pos!(2.0),
            initial_income: pos!(1.0),
            date_time: Utc::now(),
        };

        let pnl2 = PnL {
            realized: Some(dec!(20.0)),
            unrealized: Some(dec!(10.0)),
            initial_costs: pos!(3.0),
            initial_income: pos!(2.0),
            date_time: Utc::now(),
        };

        let sum: PnL = vec![pnl1.clone(), pnl2.clone()].into_iter().sum();

        assert_eq!(sum.realized, Some(dec!(30.0)));
        assert_eq!(sum.unrealized, Some(dec!(15.0)));
        assert_eq!(sum.initial_costs, pos!(5.0));
        assert_eq!(sum.initial_income, pos!(3.0));
    }

    #[test]
    fn test_pnl_sum_with_none() {
        let pnl1 = PnL {
            realized: None,
            unrealized: Some(dec!(5.0)),
            initial_costs: pos!(2.0),
            initial_income: pos!(1.0),
            date_time: Utc::now(),
        };

        let pnl2 = PnL {
            realized: Some(dec!(20.0)),
            unrealized: None,
            initial_costs: pos!(3.0),
            initial_income: pos!(2.0),
            date_time: Utc::now(),
        };

        let sum: PnL = vec![pnl1.clone(), pnl2.clone()].into_iter().sum();

        assert_eq!(sum.realized, Some(dec!(20.0)));
        assert_eq!(sum.unrealized, Some(dec!(5.0)));
        assert_eq!(sum.initial_costs, pos!(5.0));
        assert_eq!(sum.initial_income, pos!(3.0));
    }

    #[test]
    fn test_pnl_sum_reference() {
        let pnl1 = PnL {
            realized: Some(dec!(10.0)),
            unrealized: Some(dec!(5.0)),
            initial_costs: pos!(2.0),
            initial_income: pos!(1.0),
            date_time: Utc::now(),
        };

        let pnl2 = PnL {
            realized: Some(dec!(20.0)),
            unrealized: Some(dec!(10.0)),
            initial_costs: pos!(3.0),
            initial_income: pos!(2.0),
            date_time: Utc::now(),
        };

        let sum: PnL = vec![&pnl1, &pnl2].into_iter().sum();

        assert_eq!(sum.realized, Some(dec!(30.0)));
        assert_eq!(sum.unrealized, Some(dec!(15.0)));
        assert_eq!(sum.initial_costs, pos!(5.0));
        assert_eq!(sum.initial_income, pos!(3.0));
    }
}

#[cfg(test)]
mod tests_add {
    use super::*;
    use crate::pos;
    use rust_decimal_macros::dec;

    #[test]
    fn test_pnl_add() {
        let pnl1 = PnL {
            realized: Some(dec!(10.0)),
            unrealized: Some(dec!(5.0)),
            initial_costs: pos!(2.0),
            initial_income: pos!(1.0),
            date_time: Utc::now(),
        };

        let pnl2 = PnL {
            realized: Some(dec!(20.0)),
            unrealized: Some(dec!(10.0)),
            initial_costs: pos!(3.0),
            initial_income: pos!(2.0),
            date_time: Utc::now(),
        };

        let sum = pnl1 + pnl2;
        assert_eq!(sum.realized, Some(dec!(30.0)));
        assert_eq!(sum.unrealized, Some(dec!(15.0)));
        assert_eq!(sum.initial_costs, pos!(5.0));
        assert_eq!(sum.initial_income, pos!(3.0));
    }

    #[test]
    fn test_pnl_add_ref() {
        let pnl1 = PnL {
            realized: Some(dec!(10.0)),
            unrealized: Some(dec!(5.0)),
            initial_costs: pos!(2.0),
            initial_income: pos!(1.0),
            date_time: Utc::now(),
        };

        let pnl2 = PnL {
            realized: Some(dec!(20.0)),
            unrealized: Some(dec!(10.0)),
            initial_costs: pos!(3.0),
            initial_income: pos!(2.0),
            date_time: Utc::now(),
        };

        let sum = &pnl1 + &pnl2;
        assert_eq!(sum.realized, Some(dec!(30.0)));
        assert_eq!(sum.unrealized, Some(dec!(15.0)));
        assert_eq!(sum.initial_costs, pos!(5.0));
        assert_eq!(sum.initial_income, pos!(3.0));
    }
}
