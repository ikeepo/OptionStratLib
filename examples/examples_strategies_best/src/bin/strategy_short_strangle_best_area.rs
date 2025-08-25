use optionstratlib::prelude::*;
use rust_decimal::Decimal;
use std::error::Error;
use tracing::{debug, info};

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger();
    let mut option_chain =
        OptionChain::load_from_json("./examples/Chains/SP500-18-oct-2024-5781.88.json")?;
    option_chain.update_expiration_date(get_x_days_formatted(30));
    let chain_params = option_chain.to_build_params()?;
    info!("Chain params: {:#?}", chain_params);
    let mut option_chain = OptionChain::build_chain(&chain_params);
    option_chain.update_greeks();

    debug!("Option Chain:  {}", option_chain);
    let underlying_price = option_chain.underlying_price;
    let mut strategy = ShortStrangle::new(
        "SP500".to_string(),
        underlying_price, // underlying_price
        Positive::ZERO,   // call_strike
        Positive::ZERO,   // put_strike
        ExpirationDate::Days(Positive::ZERO),
        Positive::ZERO, // implied_volatility
        Positive::ZERO, // implied_volatility
        Decimal::ZERO,  // risk_free_rate
        Positive::ZERO, // dividend_yield
        pos!(1.0),      // quantity
        Positive::ZERO, // premium_short_call
        Positive::ZERO, // premium_short_put
        pos!(0.12),     // open_fee_short_call
        pos!(0.12),     // close_fee_short_call
        pos!(0.12),     // open_fee_short_put
        pos!(0.12),     // close_fee_short_put
    );
    strategy.get_best_area(&option_chain, FindOptimalSide::Deltable(pos!(0.3)));
    info!("Strategy:  {:#?}", strategy);
    let range = strategy.get_range_of_profit().unwrap_or(Positive::ZERO);
    info!("Title: {}", strategy.get_title());
    info!("Break Even Points: {:?}", strategy.break_even_points);
    info!(
        "Net Premium Received: ${:.2}",
        strategy.get_net_premium_received()?
    );
    info!(
        "Max Profit: ${:.2}",
        strategy.get_max_profit().unwrap_or(Positive::ZERO)
    );
    info!(
        "Max Loss: ${:0.2}",
        strategy.get_max_loss().unwrap_or(Positive::ZERO)
    );
    info!("Total Fees: ${:.2}", strategy.get_fees()?);
    info!(
        "Range of Profit: ${:.2} {:.2}%",
        range,
        (range / 2.0) / underlying_price * 100.0
    );
    info!("Profit Area: {:.2}%", strategy.get_profit_area()?);
    info!("Delta:  {:#?}", strategy.delta_neutrality()?);
    if strategy.get_profit_ratio()? > Positive::ZERO.into() {
        debug!("Strategy:  {:#?}", strategy);
        let path: &std::path::Path =
            "Draws/Strategy/short_strangle_profit_loss_chart_best_area.png".as_ref();
        strategy.write_png(path)?;
    }
    info!("Greeks:  {:#?}", strategy.greeks());

    Ok(())
}
