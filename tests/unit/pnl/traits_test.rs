use chrono::Utc;
use optionstratlib::error::TransactionError;
use optionstratlib::pnl::{PnL, PnLCalculator, Transaction, TransactionAble};
use optionstratlib::strategies::DeltaAdjustment;
use optionstratlib::{ExpirationDate, Positive, pos, spos};
use rust_decimal_macros::dec;
use std::error::Error;

// A simple implementation of PnLCalculator for testing
struct TestPnLCalculator;

impl PnLCalculator for TestPnLCalculator {
    fn calculate_pnl(
        &self,
        underlying_price: &Positive,
        expiration_date: ExpirationDate,
        implied_volatility: &Positive,
    ) -> Result<PnL, Box<dyn Error>> {
        // Simple implementation for testing
        let realized = underlying_price.to_dec() * dec!(0.5);
        let unrealized = implied_volatility.to_dec() * dec!(100.0);

        Ok(PnL::new(
            Some(realized),
            Some(unrealized),
            pos!(10.0),
            pos!(20.0),
            expiration_date.get_date()?,
        ))
    }

    fn calculate_pnl_at_expiration(
        &self,
        underlying_price: &Positive,
    ) -> Result<PnL, Box<dyn Error>> {
        // Simple implementation for testing
        let realized = underlying_price.to_dec() * dec!(2.0);

        Ok(PnL::new(
            Some(realized),
            None,
            pos!(10.0),
            pos!(20.0),
            Utc::now(),
        ))
    }
}

// A simple implementation of TransactionAble for testing
struct TestTransactionManager {
    transactions: Vec<Transaction>,
}

impl TestTransactionManager {
    fn new() -> Self {
        TestTransactionManager {
            transactions: Vec::new(),
        }
    }
}

impl TransactionAble for TestTransactionManager {
    fn add_transaction(&mut self, transaction: Transaction) -> Result<(), TransactionError> {
        self.transactions.push(transaction);
        Ok(())
    }

    fn get_transactions(&self) -> Result<Vec<Transaction>, TransactionError> {
        Ok(self.transactions.clone())
    }
}

#[test]
fn test_pnl_calculator_trait() {
    let calculator = TestPnLCalculator;

    // Test calculate_pnl
    let underlying_price = pos!(100.0);
    let expiration_date = ExpirationDate::Days(pos!(30.0));
    let implied_volatility = pos!(0.2);

    let pnl = calculator
        .calculate_pnl(&underlying_price, expiration_date, &implied_volatility)
        .unwrap();

    assert_eq!(pnl.realized, Some(dec!(50.0))); // 100.0 * 0.5
    assert_eq!(pnl.unrealized, Some(dec!(20.0))); // 0.2 * 100.0
    assert_eq!(pnl.initial_costs, pos!(10.0));
    assert_eq!(pnl.initial_income, pos!(20.0));

    // Test calculate_pnl_at_expiration
    let pnl_at_expiration = calculator
        .calculate_pnl_at_expiration(&underlying_price)
        .unwrap();

    assert_eq!(pnl_at_expiration.realized, Some(dec!(200.0))); // 100.0 * 2.0
    assert_eq!(pnl_at_expiration.unrealized, None);
    assert_eq!(pnl_at_expiration.initial_costs, pos!(10.0));
    assert_eq!(pnl_at_expiration.initial_income, pos!(20.0));
}

#[test]
#[should_panic(expected = "adjustments_pnl is not implemented for this Strategy")]
fn test_pnl_calculator_adjustments_pnl_default_implementation() {
    let calculator = TestPnLCalculator;
    let adjustments = DeltaAdjustment::NoAdjustmentNeeded;

    // This should panic with the message "adjustments_pnl is not implemented for this Strategy"
    let _ = calculator.adjustments_pnl(&adjustments);
}

#[test]
fn test_transaction_able_trait() {
    let mut manager = TestTransactionManager::new();

    // Create test transactions
    let transaction1 = Transaction::new(
        optionstratlib::model::TradeStatus::Open,
        Some(Utc::now()),
        optionstratlib::OptionType::European,
        optionstratlib::Side::Long,
        optionstratlib::OptionStyle::Call,
        pos!(1.0),
        pos!(100.0),
        pos!(5.0),
        spos!(100.0),
        spos!(30.0),
        spos!(0.2),
    );

    let transaction2 = Transaction::new(
        optionstratlib::model::TradeStatus::Open,
        Some(Utc::now()),
        optionstratlib::OptionType::European,
        optionstratlib::Side::Short,
        optionstratlib::OptionStyle::Put,
        pos!(2.0),
        pos!(95.0),
        pos!(7.0),
        spos!(100.0),
        spos!(30.0),
        spos!(0.2),
    );

    // Test add_transaction
    manager.add_transaction(transaction1.clone()).unwrap();
    manager.add_transaction(transaction2.clone()).unwrap();

    // Test get_transactions
    let transactions = manager.get_transactions().unwrap();
    assert_eq!(transactions.len(), 2);
    assert_eq!(transactions[0].quantity(), pos!(1.0));
    assert_eq!(transactions[0].premium(), pos!(100.0));
    assert_eq!(transactions[0].fees(), pos!(5.0));
    assert_eq!(transactions[1].quantity(), pos!(2.0));
    assert_eq!(transactions[1].premium(), pos!(95.0));
    assert_eq!(transactions[1].fees(), pos!(7.0));
}
