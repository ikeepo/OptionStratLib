::::::::::::::::::::::::::::::::::::::::::: width-limiter
:::::::::::::::::::::::::::::::::::::::::: {#main-content .section .content}
:::: main-heading
::: rustdoc-breadcrumbs
[optionstratlib](../../index.html)::[strategies](../index.html)::[probabilities](index.html)
:::

# Trait [ProbabilityAnalysis]{.trait}Copy item path

[[Source](../../../src/optionstratlib/strategies/probabilities/core.rs.html#41-350){.src}
]{.sub-heading}
::::

``` {.rust .item-decl}
pub trait ProbabilityAnalysis: Strategies + Profit {
    // Required methods
    fn get_expiration(&self) -> Result<ExpirationDate, ProbabilityError>;
    fn get_risk_free_rate(&self) -> Option<Decimal>;
    fn get_profit_ranges(
        &self,
    ) -> Result<Vec<ProfitLossRange>, ProbabilityError>;
    fn get_loss_ranges(&self) -> Result<Vec<ProfitLossRange>, ProbabilityError>;

    // Provided methods
    fn analyze_probabilities(
        &self,
        volatility_adj: Option<VolatilityAdjustment>,
        trend: Option<PriceTrend>,
    ) -> Result<StrategyProbabilityAnalysis, ProbabilityError> { ... }
    fn expected_value(
        &self,
        volatility_adj: Option<VolatilityAdjustment>,
        trend: Option<PriceTrend>,
    ) -> Result<Positive, ProbabilityError> { ... }
    fn probability_of_profit(
        &self,
        volatility_adj: Option<VolatilityAdjustment>,
        trend: Option<PriceTrend>,
    ) -> Result<Positive, ProbabilityError> { ... }
    fn probability_of_loss(
        &self,
        volatility_adj: Option<VolatilityAdjustment>,
        trend: Option<PriceTrend>,
    ) -> Result<Positive, ProbabilityError> { ... }
    fn calculate_extreme_probabilities(
        &self,
        volatility_adj: Option<VolatilityAdjustment>,
        trend: Option<PriceTrend>,
    ) -> Result<(Positive, Positive), ProbabilityError> { ... }
}
```

Expand description

::: docblock
Trait for analyzing probabilities and risk metrics of option strategies

This trait provides methods to analyze the probability characteristics
of options strategies, including probability of profit/loss, expected
value, and risk-reward metrics.

## [§](#type-requirements){.doc-anchor}Type Requirements

Implementors must also implement:

- The `Strategies` trait, which provides access to strategy
  configuration
- The `Profit` trait, which provides profit calculation capabilities

## [§](#key-features){.doc-anchor}Key Features

- Calculate probability of profit for option strategies
- Compute expected values with adjustments for volatility and price
  trends
- Determine break-even points and risk-reward ratios
- Analyze extreme outcome probabilities (max profit and max loss
  scenarios)
:::

## Required Methods[§](#required-methods){.anchor} {#required-methods .section-header}

::::::::::: methods
::: {#tymethod.get_expiration .section .method}
[Source](../../../src/optionstratlib/strategies/probabilities/core.rs.html#320){.src
.rightside}

#### fn [get_expiration](#tymethod.get_expiration){.fn}(&self) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<[ExpirationDate](../../model/types/enum.ExpirationDate.html "enum optionstratlib::model::types::ExpirationDate"){.enum}, [ProbabilityError](../../error/probability/enum.ProbabilityError.html "enum optionstratlib::error::probability::ProbabilityError"){.enum}\> {#fn-get_expirationself---resultexpirationdate-probabilityerror .code-header}
:::

::: docblock
Get the expiration date of the option strategy

##### [§](#returns){.doc-anchor}Returns

- `Result<ExpirationDate, ProbabilityError>`: The expiration date or an
  error
:::

::: {#tymethod.get_risk_free_rate .section .method}
[Source](../../../src/optionstratlib/strategies/probabilities/core.rs.html#326){.src
.rightside}

#### fn [get_risk_free_rate](#tymethod.get_risk_free_rate){.fn}(&self) -\> [Option](https://doc.rust-lang.org/1.86.0/core/option/enum.Option.html "enum core::option::Option"){.enum}\<Decimal\> {#fn-get_risk_free_rateself---optiondecimal .code-header}
:::

::: docblock
Get the current risk-free interest rate

##### [§](#returns-1){.doc-anchor}Returns

- `Option<Decimal>`: The risk-free rate as a decimal, or None if not
  available
:::

::: {#tymethod.get_profit_ranges .section .method}
[Source](../../../src/optionstratlib/strategies/probabilities/core.rs.html#333){.src
.rightside}

#### fn [get_profit_ranges](#tymethod.get_profit_ranges){.fn}(&self) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<[Vec](https://doc.rust-lang.org/1.86.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec"){.struct}\<[ProfitLossRange](../../model/struct.ProfitLossRange.html "struct optionstratlib::model::ProfitLossRange"){.struct}\>, [ProbabilityError](../../error/probability/enum.ProbabilityError.html "enum optionstratlib::error::probability::ProbabilityError"){.enum}\> {#fn-get_profit_rangesself---resultvecprofitlossrange-probabilityerror .code-header}
:::

::: docblock
Get the price ranges that would result in a profit

##### [§](#returns-2){.doc-anchor}Returns

- `Result<Vec<ProfitLossRange>, ProbabilityError>`: A vector of price
  ranges that result in profit, or an error
:::

::: {#tymethod.get_loss_ranges .section .method}
[Source](../../../src/optionstratlib/strategies/probabilities/core.rs.html#349){.src
.rightside}

#### fn [get_loss_ranges](#tymethod.get_loss_ranges){.fn}(&self) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<[Vec](https://doc.rust-lang.org/1.86.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec"){.struct}\<[ProfitLossRange](../../model/struct.ProfitLossRange.html "struct optionstratlib::model::ProfitLossRange"){.struct}\>, [ProbabilityError](../../error/probability/enum.ProbabilityError.html "enum optionstratlib::error::probability::ProbabilityError"){.enum}\> {#fn-get_loss_rangesself---resultvecprofitlossrange-probabilityerror .code-header}
:::

::: docblock
##### [§](#get-profitloss-ranges){.doc-anchor}Get Profit/Loss Ranges

Returns a collection of price ranges with associated probabilities for
profit and loss scenarios.

This function analyzes the strategy to identify distinct price ranges
where the strategy would result in either profit or loss at expiration.
Each range includes probability information based on the statistical
model for the underlying asset.

###### [§](#returns-3){.doc-anchor}Returns

- `Result<Vec<ProfitLossRange>, ProbabilityError>` - On success, returns
  a vector of profit/loss ranges sorted by their price boundaries. On
  failure, returns a `ProbabilityError` indicating what went wrong
  during the analysis.
:::
:::::::::::

## Provided Methods[§](#provided-methods){.anchor} {#provided-methods .section-header}

::::::::::::: methods
::: {#method.analyze_probabilities .section .method}
[Source](../../../src/optionstratlib/strategies/probabilities/core.rs.html#65-102){.src
.rightside}

#### fn [analyze_probabilities](#method.analyze_probabilities){.fn}( &self, volatility_adj: [Option](https://doc.rust-lang.org/1.86.0/core/option/enum.Option.html "enum core::option::Option"){.enum}\<[VolatilityAdjustment](struct.VolatilityAdjustment.html "struct optionstratlib::strategies::probabilities::VolatilityAdjustment"){.struct}\>, trend: [Option](https://doc.rust-lang.org/1.86.0/core/option/enum.Option.html "enum core::option::Option"){.enum}\<[PriceTrend](struct.PriceTrend.html "struct optionstratlib::strategies::probabilities::PriceTrend"){.struct}\>, ) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<[StrategyProbabilityAnalysis](struct.StrategyProbabilityAnalysis.html "struct optionstratlib::strategies::probabilities::StrategyProbabilityAnalysis"){.struct}, [ProbabilityError](../../error/probability/enum.ProbabilityError.html "enum optionstratlib::error::probability::ProbabilityError"){.enum}\> {#fn-analyze_probabilities-self-volatility_adj-optionvolatilityadjustment-trend-optionpricetrend---resultstrategyprobabilityanalysis-probabilityerror .code-header}
:::

::: docblock
Calculate probability analysis for a strategy

Performs a comprehensive probability analysis for an option strategy,
taking into account optional volatility adjustments and price trend
parameters.

##### [§](#parameters){.doc-anchor}Parameters

- `volatility_adj`: Optional volatility adjustment parameters
- `trend`: Optional price trend parameters indicating market direction
  bias

##### [§](#returns-4){.doc-anchor}Returns

- `Result<StrategyProbabilityAnalysis, ProbabilityError>`: Structured
  analysis results or an error

##### [§](#analysis-components){.doc-anchor}Analysis Components

The returned analysis includes:

- Probability of profit
- Probability of reaching maximum profit
- Probability of suffering maximum loss
- Expected value
- Break-even points
- Risk-reward ratio
:::

::: {#method.expected_value .section .method}
[Source](../../../src/optionstratlib/strategies/probabilities/core.rs.html#129-191){.src
.rightside}

#### fn [expected_value](#method.expected_value){.fn}( &self, volatility_adj: [Option](https://doc.rust-lang.org/1.86.0/core/option/enum.Option.html "enum core::option::Option"){.enum}\<[VolatilityAdjustment](struct.VolatilityAdjustment.html "struct optionstratlib::strategies::probabilities::VolatilityAdjustment"){.struct}\>, trend: [Option](https://doc.rust-lang.org/1.86.0/core/option/enum.Option.html "enum core::option::Option"){.enum}\<[PriceTrend](struct.PriceTrend.html "struct optionstratlib::strategies::probabilities::PriceTrend"){.struct}\>, ) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<[Positive](../../model/positive/struct.Positive.html "struct optionstratlib::model::positive::Positive"){.struct}, [ProbabilityError](../../error/probability/enum.ProbabilityError.html "enum optionstratlib::error::probability::ProbabilityError"){.enum}\> {#fn-expected_value-self-volatility_adj-optionvolatilityadjustment-trend-optionpricetrend---resultpositive-probabilityerror .code-header}
:::

::: docblock
This function calculates the expected value of an option strategy based
on an underlying price, volatility adjustments, and price trends.

##### [§](#parameters-1){.doc-anchor}Parameters

- `volatility_adj`: An optional `VolatilityAdjustment` parameter, which
  contains the base volatility and the number of standard deviations to
  adjust.
- `trend`: An optional `PriceTrend` parameter, which indicates the
  annual drift rate and the confidence level for the trend.

##### [§](#returns-5){.doc-anchor}Returns

- `Result<Positive, String>`: On success, returns a `Positive`
  representing the expected value. On failure, returns an error message
  as a `String`.

The function performs the following operations:

- Determines the pricing range using the underlying asset's price and
  steps based on 1% increments of the current price.
- Calculates the single-point probability for each price within the
  range using the provided volatility adjustments and price trends.
- Computes the expected value by summing up the product of calculated
  probabilities and the strategy's profit at each price point.
- Logs the calculated range with probabilities for diagnostic purposes.

This function relies on several auxiliary methods and traits, such as
`get_underlying_price`, `best_range_to_show`, and `calculate_profit_at`,
which are defined in the module's traits and utilities.
:::

::: {#method.probability_of_profit .section .method}
[Source](../../../src/optionstratlib/strategies/probabilities/core.rs.html#206-224){.src
.rightside}

#### fn [probability_of_profit](#method.probability_of_profit){.fn}( &self, volatility_adj: [Option](https://doc.rust-lang.org/1.86.0/core/option/enum.Option.html "enum core::option::Option"){.enum}\<[VolatilityAdjustment](struct.VolatilityAdjustment.html "struct optionstratlib::strategies::probabilities::VolatilityAdjustment"){.struct}\>, trend: [Option](https://doc.rust-lang.org/1.86.0/core/option/enum.Option.html "enum core::option::Option"){.enum}\<[PriceTrend](struct.PriceTrend.html "struct optionstratlib::strategies::probabilities::PriceTrend"){.struct}\>, ) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<[Positive](../../model/positive/struct.Positive.html "struct optionstratlib::model::positive::Positive"){.struct}, [ProbabilityError](../../error/probability/enum.ProbabilityError.html "enum optionstratlib::error::probability::ProbabilityError"){.enum}\> {#fn-probability_of_profit-self-volatility_adj-optionvolatilityadjustment-trend-optionpricetrend---resultpositive-probabilityerror .code-header}
:::

::: docblock
Calculate probability of profit

Calculates the probability that the option strategy will result in a
profit at expiration. This method aggregates probabilities across all
price ranges that would result in a profit.

##### [§](#parameters-2){.doc-anchor}Parameters

- `volatility_adj`: Optional volatility adjustment parameters
- `trend`: Optional price trend parameters

##### [§](#returns-6){.doc-anchor}Returns

- `Result<Positive, ProbabilityError>`: The probability of profit
  (between 0 and 1) or an error
:::

::: {#method.probability_of_loss .section .method}
[Source](../../../src/optionstratlib/strategies/probabilities/core.rs.html#239-257){.src
.rightside}

#### fn [probability_of_loss](#method.probability_of_loss){.fn}( &self, volatility_adj: [Option](https://doc.rust-lang.org/1.86.0/core/option/enum.Option.html "enum core::option::Option"){.enum}\<[VolatilityAdjustment](struct.VolatilityAdjustment.html "struct optionstratlib::strategies::probabilities::VolatilityAdjustment"){.struct}\>, trend: [Option](https://doc.rust-lang.org/1.86.0/core/option/enum.Option.html "enum core::option::Option"){.enum}\<[PriceTrend](struct.PriceTrend.html "struct optionstratlib::strategies::probabilities::PriceTrend"){.struct}\>, ) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<[Positive](../../model/positive/struct.Positive.html "struct optionstratlib::model::positive::Positive"){.struct}, [ProbabilityError](../../error/probability/enum.ProbabilityError.html "enum optionstratlib::error::probability::ProbabilityError"){.enum}\> {#fn-probability_of_loss-self-volatility_adj-optionvolatilityadjustment-trend-optionpricetrend---resultpositive-probabilityerror .code-header}
:::

::: docblock
Calculate probability of loss

Calculates the probability that the option strategy will result in a
loss at expiration. This method aggregates probabilities across all
price ranges that would result in a loss.

##### [§](#parameters-3){.doc-anchor}Parameters

- `volatility_adj`: Optional volatility adjustment parameters
- `trend`: Optional price trend parameters

##### [§](#returns-7){.doc-anchor}Returns

- `Result<Positive, ProbabilityError>`: The probability of loss (between
  0 and 1) or an error
:::

::: {#method.calculate_extreme_probabilities .section .method}
[Source](../../../src/optionstratlib/strategies/probabilities/core.rs.html#273-314){.src
.rightside}

#### fn [calculate_extreme_probabilities](#method.calculate_extreme_probabilities){.fn}( &self, volatility_adj: [Option](https://doc.rust-lang.org/1.86.0/core/option/enum.Option.html "enum core::option::Option"){.enum}\<[VolatilityAdjustment](struct.VolatilityAdjustment.html "struct optionstratlib::strategies::probabilities::VolatilityAdjustment"){.struct}\>, trend: [Option](https://doc.rust-lang.org/1.86.0/core/option/enum.Option.html "enum core::option::Option"){.enum}\<[PriceTrend](struct.PriceTrend.html "struct optionstratlib::strategies::probabilities::PriceTrend"){.struct}\>, ) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<([Positive](../../model/positive/struct.Positive.html "struct optionstratlib::model::positive::Positive"){.struct}, [Positive](../../model/positive/struct.Positive.html "struct optionstratlib::model::positive::Positive"){.struct}), [ProbabilityError](../../error/probability/enum.ProbabilityError.html "enum optionstratlib::error::probability::ProbabilityError"){.enum}\> {#fn-calculate_extreme_probabilities-self-volatility_adj-optionvolatilityadjustment-trend-optionpricetrend---resultpositive-positive-probabilityerror .code-header}
:::

::: docblock
Calculate extreme probabilities (max profit and max loss)

Calculates the probabilities of reaching the maximum possible profit and
suffering the maximum possible loss for the strategy.

##### [§](#parameters-4){.doc-anchor}Parameters

- `volatility_adj`: Optional volatility adjustment parameters
- `trend`: Optional price trend parameters

##### [§](#returns-8){.doc-anchor}Returns

- `Result<(Positive, Positive), ProbabilityError>`: A tuple containing
  (probability_of_max_profit, probability_of_max_loss) or an error
:::
:::::::::::::

## Implementors[§](#implementors){.anchor} {#implementors .section-header}

:::::::::::::::::: {#implementors-list}
::: {#impl-ProbabilityAnalysis-for-BearCallSpread .section .impl}
[Source](../../../src/optionstratlib/strategies/bear_call_spread.rs.html#717-773){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-BearCallSpread){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [BearCallSpread](../bear_call_spread/struct.BearCallSpread.html "struct optionstratlib::strategies::bear_call_spread::BearCallSpread"){.struct} {#impl-probabilityanalysis-for-bearcallspread .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-BearPutSpread .section .impl}
[Source](../../../src/optionstratlib/strategies/bear_put_spread.rs.html#699-763){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-BearPutSpread){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [BearPutSpread](../bear_put_spread/struct.BearPutSpread.html "struct optionstratlib::strategies::bear_put_spread::BearPutSpread"){.struct} {#impl-probabilityanalysis-for-bearputspread .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-BullCallSpread .section .impl}
[Source](../../../src/optionstratlib/strategies/bull_call_spread.rs.html#712-776){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-BullCallSpread){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [BullCallSpread](../bull_call_spread/struct.BullCallSpread.html "struct optionstratlib::strategies::bull_call_spread::BullCallSpread"){.struct} {#impl-probabilityanalysis-for-bullcallspread .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-BullPutSpread .section .impl}
[Source](../../../src/optionstratlib/strategies/bull_put_spread.rs.html#813-877){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-BullPutSpread){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [BullPutSpread](../bull_put_spread/struct.BullPutSpread.html "struct optionstratlib::strategies::bull_put_spread::BullPutSpread"){.struct} {#impl-probabilityanalysis-for-bullputspread .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-LongButterflySpread .section .impl}
[Source](../../../src/optionstratlib/strategies/butterfly_spread.rs.html#877-964){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-LongButterflySpread){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [LongButterflySpread](../butterfly_spread/struct.LongButterflySpread.html "struct optionstratlib::strategies::butterfly_spread::LongButterflySpread"){.struct} {#impl-probabilityanalysis-for-longbutterflyspread .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-ShortButterflySpread .section .impl}
[Source](../../../src/optionstratlib/strategies/butterfly_spread.rs.html#1844-1931){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-ShortButterflySpread){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [ShortButterflySpread](../butterfly_spread/struct.ShortButterflySpread.html "struct optionstratlib::strategies::butterfly_spread::ShortButterflySpread"){.struct} {#impl-probabilityanalysis-for-shortbutterflyspread .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-CallButterfly .section .impl}
[Source](../../../src/optionstratlib/strategies/call_butterfly.rs.html#863-940){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-CallButterfly){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [CallButterfly](../call_butterfly/struct.CallButterfly.html "struct optionstratlib::strategies::call_butterfly::CallButterfly"){.struct} {#impl-probabilityanalysis-for-callbutterfly .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-CustomStrategy .section .impl}
[Source](../../../src/optionstratlib/strategies/custom.rs.html#668-743){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-CustomStrategy){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [CustomStrategy](../custom/struct.CustomStrategy.html "struct optionstratlib::strategies::custom::CustomStrategy"){.struct} {#impl-probabilityanalysis-for-customstrategy .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-IronButterfly .section .impl}
[Source](../../../src/optionstratlib/strategies/iron_butterfly.rs.html#926-1005){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-IronButterfly){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [IronButterfly](../iron_butterfly/struct.IronButterfly.html "struct optionstratlib::strategies::iron_butterfly::IronButterfly"){.struct} {#impl-probabilityanalysis-for-ironbutterfly .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-IronCondor .section .impl}
[Source](../../../src/optionstratlib/strategies/iron_condor.rs.html#961-1040){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-IronCondor){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [IronCondor](../iron_condor/struct.IronCondor.html "struct optionstratlib::strategies::iron_condor::IronCondor"){.struct} {#impl-probabilityanalysis-for-ironcondor .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-PoorMansCoveredCall .section .impl}
[Source](../../../src/optionstratlib/strategies/poor_mans_covered_call.rs.html#745-801){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-PoorMansCoveredCall){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [PoorMansCoveredCall](../poor_mans_covered_call/struct.PoorMansCoveredCall.html "struct optionstratlib::strategies::poor_mans_covered_call::PoorMansCoveredCall"){.struct} {#impl-probabilityanalysis-for-poormanscoveredcall .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-LongStraddle .section .impl}
[Source](../../../src/optionstratlib/strategies/straddle.rs.html#1530-1608){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-LongStraddle){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [LongStraddle](../straddle/struct.LongStraddle.html "struct optionstratlib::strategies::straddle::LongStraddle"){.struct} {#impl-probabilityanalysis-for-longstraddle .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-ShortStraddle .section .impl}
[Source](../../../src/optionstratlib/strategies/straddle.rs.html#750-828){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-ShortStraddle){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [ShortStraddle](../straddle/struct.ShortStraddle.html "struct optionstratlib::strategies::straddle::ShortStraddle"){.struct} {#impl-probabilityanalysis-for-shortstraddle .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-LongStrangle .section .impl}
[Source](../../../src/optionstratlib/strategies/strangle.rs.html#1834-1912){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-LongStrangle){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [LongStrangle](../strangle/struct.LongStrangle.html "struct optionstratlib::strategies::strangle::LongStrangle"){.struct} {#impl-probabilityanalysis-for-longstrangle .code-header}
:::

::: {#impl-ProbabilityAnalysis-for-ShortStrangle .section .impl}
[Source](../../../src/optionstratlib/strategies/strangle.rs.html#846-924){.src
.rightside}[§](#impl-ProbabilityAnalysis-for-ShortStrangle){.anchor}

### impl [ProbabilityAnalysis](trait.ProbabilityAnalysis.html "trait optionstratlib::strategies::probabilities::ProbabilityAnalysis"){.trait} for [ShortStrangle](../strangle/struct.ShortStrangle.html "struct optionstratlib::strategies::strangle::ShortStrangle"){.struct} {#impl-probabilityanalysis-for-shortstrangle .code-header}
:::
::::::::::::::::::
::::::::::::::::::::::::::::::::::::::::::
:::::::::::::::::::::::::::::::::::::::::::
