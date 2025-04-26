//! Backtesting
//! ```mermaid
//! classDiagram
//! direction LR
//!     class BacktestResult {
//!         +GeneralPerformanceMetrics general_performance
//!         +OptionsSpecificMetrics options_metrics
//!         +TradeStatistics trade_statistics
//!         +DrawdownAnalysis drawdown_analysis
//!         +CapitalUtilization capital_utilization
//!         +TimeSeriesData time_series
//!         +Vec~TradeRecord~ trades
//!         +MarketConditionMetrics? market_conditions
//!         +VolatilityData? volatility_data
//!         +AdvancedRiskMetrics? risk_metrics
//!         +SimulationResult? monte_carlo_simulation
//!         +String strategy_name
//!         +DateTime test_period_start
//!         +DateTime test_period_end
//!         +Decimal initial_capital
//!         +Decimal final_capital
//!         +HashMap custom_metrics
//!     }
//!
//!     class GeneralPerformanceMetrics {
//!         +Decimal total_return
//!         +Decimal annualized_return
//!         +Positive? volatility
//!         +Positive? downside_deviation
//!         +Decimal? sharpe_ratio
//!         +Decimal? sortino_ratio
//!         +Decimal? calmar_ratio
//!         +Decimal? win_rate
//!         +Decimal? profit_factor
//!         +Decimal? avg_gain
//!         +Decimal? avg_loss
//!         +Decimal? gain_loss_ratio
//!     }
//!
//!     class OptionsSpecificMetrics {
//!         +Decimal? return_on_margin
//!         +Decimal? return_on_premium
//!         +Decimal? premium_capture
//!         +Decimal? avg_delta_exposure
//!         +Decimal? avg_gamma_exposure
//!         +Decimal? avg_theta_exposure
//!         +Decimal? avg_vega_exposure
//!         +Decimal? calls_percentage
//!         +Decimal? puts_percentage
//!         +Decimal? long_percentage
//!         +Decimal? short_percentage
//!     }
//!
//!     class TradeStatistics {
//!         +usize number_of_trades
//!         +usize winners
//!         +usize losers
//!         +usize break_even
//!         +Decimal average_trade_return
//!         +Decimal median_trade_return
//!         +Decimal? largest_win
//!         +Decimal? largest_loss
//!         +Positive average_holding_period
//!         +Positive median_holding_period
//!         +Positive min_holding_period
//!         +Positive max_holding_period
//!         +usize long_trades
//!         +usize short_trades
//!         +usize call_trades
//!         +usize put_trades
//!         +usize spread_trades
//!     }
//!
//!     class DrawdownAnalysis {
//!         +Decimal max_drawdown
//!         +Positive max_drawdown_duration
//!         +Positive? recovery_duration
//!         +Positive time_to_max_drawdown
//!         +Vec~DrawdownEvent~ drawdowns
//!         +Decimal avg_drawdown
//!         +Positive? avg_recovery_time
//!         +Positive total_underwater_days
//!         +Decimal underwater_percentage
//!     }
//!
//!     class CapitalUtilization {
//!         +Decimal max_capital_used
//!         +Decimal avg_capital_used
//!         +Decimal capital_efficiency
//!         +Decimal total_margin_used
//!         +Decimal max_margin_used
//!         +Decimal avg_margin_used
//!         +Decimal total_premium_paid
//!         +Decimal total_premium_received
//!         +Decimal net_premium
//!         +Decimal max_position_size
//!         +Decimal avg_position_size
//!     }
//!
//!     class TimeSeriesData {
//!         +Vec timestamps
//!         +Vec equity_curve
//!         +Vec drawdown_curve
//!         +Vec margin_usage
//!         +Vec position_count
//!         +Vec? delta_exposure
//!         +Vec? gamma_exposure
//!         +Vec? theta_exposure
//!         +Vec? vega_exposure
//!         +HashMap custom_series
//!         +new(capacity)
//!         +add_data_point()
//!         +add_greeks()
//!         +add_custom_metric()
//!     }
//!
//!     class TradeRecord {
//!         +Uuid id
//!         +DateTime entry_date
//!         +DateTime? exit_date
//!         +Positive? duration
//!         +Uuid? strategy
//!         +Position position
//!         +Decimal? exit_price
//!         +Decimal? slippage
//!         +Decimal? profit_loss
//!         +Decimal? return_percentage
//!         +Decimal? margin_required
//!         +ExitReason? exit_reason
//!         +String? notes
//!         +GreeksSnapshot? entry_greeks
//!         +GreeksSnapshot? exit_greeks
//!     }
//!
//!     class MarketConditionMetrics {
//!         +Positive bull_market_days
//!         +Positive bear_market_days
//!         +Positive sideways_market_days
//!         +Positive high_volatility_days
//!         +Positive low_volatility_days
//!         +Decimal? avg_market_volatility
//!     }
//!
//!     class VolatilityData {
//!         +bool implied_volatility_used
//!         +Decimal? avg_iv_traded
//!         +Decimal? iv_percentile_traded
//!         +Decimal? iv_rank_traded
//!     }
//!
//!     class AdvancedRiskMetrics {
//!         +Decimal? value_at_risk_95
//!         +Decimal? value_at_risk_99
//!         +Decimal? expected_shortfall
//!         +Decimal? tail_ratio
//!         +usize max_consecutive_losses
//!         +Decimal? ulcer_index
//!         +Decimal? pain_index
//!     }
//!
//!     class SimulationResult {
//!         +usize simulation_count
//!         +RiskMetricsSimulation risk_metrics
//!         +HashMap final_equity_percentiles
//!         +Vec? sample_paths
//!     }
//!
//!     class CustomMetrics {
//!         String to Decimal mapping
//!     }
//!
//!     class RollingMetrics {
//!         +Positive window_size
//!         +Vec timestamps
//!         +Vec rolling_returns
//!         +Vec rolling_volatility
//!         +Vec rolling_sharpe
//!         +Vec rolling_sortino
//!         +Vec rolling_win_rate
//!     }
//!
//!     class BenchmarkComparison {
//!         +String benchmark_name
//!         +Decimal benchmark_return
//!         +Decimal alpha
//!         +Decimal beta
//!         +Decimal correlation
//!         +Decimal tracking_error
//!         +Decimal? information_ratio
//!         +Decimal? up_capture
//!         +Decimal? down_capture
//!     }
//!
//!     class DrawdownEvent {
//!         +NaiveDateTime start_date
//!         +NaiveDateTime bottom_date
//!         +NaiveDateTime? recovery_date
//!         +Decimal magnitude
//!         +Positive duration
//!         +Positive? recovery_duration
//!     }
//!
//!     class RiskMetricsSimulation {
//!     }
//!
//!     class EquityPercentiles {
//!         u8 to Decimal mapping
//!     }
//!
//!     class SamplePaths {
//!         Vec of Vec of Decimal
//!     }
//!
//!     class GreeksSnapshot {
//!     }
//!
//!     class Position {
//!     }
//!
//!     class ExitReason {
//!         TargetReached
//!         StopLoss
//!         Expiration
//!         RollOver
//!         ManualClose
//!         MarginCall
//!         Other(String)
//!     }
//!
//!     class CustomSeries {
//!         String to Vec mapping
//!     }
//!
//!     class TimestampSeries {
//!         Vec of DateTime
//!     }
//!
//!     class EquityCurve {
//!         Vec of Decimal
//!     }
//!
//!     class DrawdownCurve {
//!         Vec of Decimal
//!     }
//!
//!     class MarginUsage {
//!         Vec of Decimal
//!     }
//!
//!     class PositionCount {
//!         Vec of usize
//!     }
//!
//!     class GreeksExposure {
//!         Vectors of Delta, Gamma, Theta, Vega
//!     }
//!
//!     class WindowSize {
//!         Positive value
//!     }
//!
//!     class RollingTimestamps {
//!         Vec of NaiveDateTime
//!     }
//!
//!     class RollingReturns {
//!         Vec of Decimal
//!     }
//!
//!     class RollingVolatility {
//!         Vec of Decimal
//!     }
//!
//!     class RollingSharpe {
//!         Vec of Decimal
//!     }
//!
//!     class RollingSortino {
//!         Vec of Decimal
//!     }
//!
//!     class RollingWinRate {
//!         Vec of Decimal
//!     }
//!
//!     <<abstract>> CustomMetrics
//!     <<abstract>> EquityPercentiles
//!     <<abstract>> SamplePaths
//!     <<enumeration>> ExitReason
//!     <<abstract>> CustomSeries
//!     <<abstract>> TimestampSeries
//!     <<abstract>> EquityCurve
//!     <<abstract>> DrawdownCurve
//!     <<abstract>> MarginUsage
//!     <<abstract>> PositionCount
//!     <<abstract>> GreeksExposure
//!     <<abstract>> WindowSize
//!     <<abstract>> RollingTimestamps
//!     <<abstract>> RollingReturns
//!     <<abstract>> RollingVolatility
//!     <<abstract>> RollingSharpe
//!     <<abstract>> RollingSortino
//!     <<abstract>> RollingWinRate
//!
//!     BacktestResult *-- GeneralPerformanceMetrics
//!     BacktestResult *-- OptionsSpecificMetrics
//!     BacktestResult *-- TradeStatistics
//!     BacktestResult *-- DrawdownAnalysis
//!     BacktestResult *-- CapitalUtilization
//!     BacktestResult *-- TimeSeriesData
//!     BacktestResult *-- TradeRecord : many
//!     BacktestResult o-- MarketConditionMetrics : optional
//!     BacktestResult o-- VolatilityData : optional
//!     BacktestResult o-- AdvancedRiskMetrics : optional
//!     BacktestResult o-- SimulationResult : monte_carlo
//!     BacktestResult o-- CustomMetrics : key-value pairs
//!     RollingMetrics --o BacktestResult : calculated from
//!     BenchmarkComparison --o BacktestResult : compared to
//!     DrawdownAnalysis *-- DrawdownEvent : many
//!     SimulationResult *-- RiskMetricsSimulation
//!     SimulationResult o-- EquityPercentiles : percentile map
//!     SimulationResult o-- SamplePaths : optional
//!     TradeRecord o-- GreeksSnapshot : entry_greeks
//!     TradeRecord o-- GreeksSnapshot : exit_greeks
//!     TradeRecord o-- Position : position details
//!     TradeRecord o-- ExitReason : reason for exit
//!     TimeSeriesData o-- CustomSeries : name-values map
//!     TimeSeriesData o-- TimestampSeries : date points
//!     TimeSeriesData o-- EquityCurve : values over time
//!     TimeSeriesData o-- DrawdownCurve : values over time
//!     TimeSeriesData o-- MarginUsage : values over time
//!     TimeSeriesData o-- PositionCount : count over time
//!     TimeSeriesData o-- GreeksExposure : optional
//!     RollingMetrics o-- WindowSize : period length
//!     RollingMetrics o-- RollingTimestamps : period dates
//!     RollingMetrics o-- RollingReturns : values over periods
//!     RollingMetrics o-- RollingVolatility : values over periods
//!     RollingMetrics o-- RollingSharpe : values over periods
//!     RollingMetrics o-- RollingSortino : values over periods
//!     RollingMetrics o-- RollingWinRate : values over periods
//!     Position --o TradeRecord : many
//!     GreeksSnapshot --o TradeRecord
//! ```
//!

/// GeneralPerformanceMetrics
///
/// Purpose:
/// Stores key performance and risk metrics for any trading strategy.
///
/// Fields:
/// total_return: Total return over the entire period.
/// annualized_return: Yearly compounded return.
/// volatility: Standard deviation of returns (optional).
/// downside_deviation: Standard deviation of negative returns (optional).
/// sharpe_ratio: Risk-adjusted return vs volatility (optional).
/// sortino_ratio: Risk-adjusted return vs downside risk (optional).
/// calmar_ratio: Annualized return over max drawdown (optional).
/// win_rate: % of winning trades (optional).
/// profit_factor: Gross profits / Gross losses (optional).
/// avg_gain: Average winning trade (optional).
/// avg_loss: Average losing trade (optional).
/// gain_loss_ratio: Avg gain divided by avg loss (optional).
pub mod metrics;

/// This module defines the structure that aggregates and stores all results produced during a trading backtest simulation.
///
/// It includes:
/// Core performance metrics (returns, volatility, Sharpe, win rates, etc.).
/// Options-specific metrics (return on margin, Greeks exposure).
/// Detailed trade statistics and individual trade records.
/// Full drawdown analysis (max drawdown, recovery times).
/// Capital utilization data (capital usage, margin tracking).
/// Time-series data for an equity curve, drawdowns, Greeks, and custom series.
/// Optional metrics about market conditions (bull/bear days, volatility environment).
/// Optional implied volatility tracking at trade times.
/// Optional advanced risk metrics (VaR, Expected Shortfall, Ulcer Index).
/// Optional Monte Carlo simulations for probabilistic risk and outcome analysis.
/// General metadata: strategy name, test period start/end, initial/final capital, and space for custom metrics.
///
/// The BacktestResult struct is designed to fully capture all relevant information about how a strategy performed during a test, allowing deep analytics, reporting, and risk assessment.
pub mod results;

/// This module defines the core data structures needed to analyze trading performance, risk, and capital utilization for backtests or live trading systems.
///
/// It captures:
/// Global trading statistics like number of trades, average returns, win/loss rates, and trade holding times.
/// Full analysis of drawdown events, including severity, duration, and recovery tracking.
/// Usage of capital and margin over time, including how efficiently capital is deployed.
/// Time-series tracking for key metrics like equity curve, drawdowns, margin usage, and options Greeks exposure.
/// Full trade-by-trade recording, storing all relevant information from entry to exit (including slippage, PnL, Greeks, exit reasons).
/// Implied volatility data recorded during trades to analyze volatility conditions at trade times.
///
/// It is designed to be fully serializable (serde) for easy storage, reporting, or integration into larger analytics systems.
pub mod types;
