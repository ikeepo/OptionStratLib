:::::::::::::::::::::::::::::: width-limiter
::::::::::::::::::::::::::::: {#main-content .section .content}
:::: main-heading
::: rustdoc-breadcrumbs
[optionstratlib](../../index.html)::[strategies](../index.html)::[base](index.html)
:::

# Trait [Strategable]{.trait}Copy item path

[[Source](../../../src/optionstratlib/strategies/base.rs.html#46-108){.src}
]{.sub-heading}
::::

``` {.rust .item-decl}
pub trait Strategable:
    Strategies
    + StrategyConstructor
    + Profit
    + Graph
    + ProbabilityAnalysis
    + Greeks
    + DeltaNeutrality
    + PnLCalculator {
    // Provided methods
    fn info(&self) -> Result<StrategyBasics, StrategyError> { ... }
    fn type_name(&self) -> StrategyType { ... }
    fn name(&self) -> String { ... }
}
```

Expand description

::: docblock
This trait defines common functionalities for all trading strategies. It
combines several other traits, requiring implementations for methods
related to strategy information, construction, optimization, profit
calculation, graphing, probability analysis, Greeks calculation, delta
neutrality, and P&L calculation.
:::

## Provided Methods[§](#provided-methods){.anchor} {#provided-methods .section-header}

::::::::: methods
::: {#method.info .section .method}
[Source](../../../src/optionstratlib/strategies/base.rs.html#66-71){.src
.rightside}

#### fn [info](#method.info){.fn}(&self) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<[StrategyBasics](struct.StrategyBasics.html "struct optionstratlib::strategies::base::StrategyBasics"){.struct}, [StrategyError](../../error/strategies/enum.StrategyError.html "enum optionstratlib::error::strategies::StrategyError"){.enum}\> {#fn-infoself---resultstrategybasics-strategyerror .code-header}
:::

::: docblock
Returns basic information about the strategy, such as its name, type,
and description.

This method returns an error by default, as it is expected to be
implemented by specific strategy types. The error indicates that the
`info` operation is not supported for the given strategy type.

##### [§](#returns){.doc-anchor}Returns

A `Result` containing the `StrategyBasics` struct if successful, or a
`StrategyError` if the operation is not supported.
:::

::: {#method.type_name .section .method}
[Source](../../../src/optionstratlib/strategies/base.rs.html#82-89){.src
.rightside}

#### fn [type_name](#method.type_name){.fn}(&self) -\> [StrategyType](enum.StrategyType.html "enum optionstratlib::strategies::base::StrategyType"){.enum} {#fn-type_nameself---strategytype .code-header}
:::

::: docblock
Returns the type of the strategy.

This method attempts to retrieve the strategy type from the `info()`
method. If `info()` returns an error (indicating it's not implemented
for the specific strategy), it asserts with a message and returns
`StrategyType::Custom`.

##### [§](#returns-1){.doc-anchor}Returns

The `StrategyType` of the strategy.
:::

::: {#method.name .section .method}
[Source](../../../src/optionstratlib/strategies/base.rs.html#100-107){.src
.rightside}

#### fn [name](#method.name){.fn}(&self) -\> [String](https://doc.rust-lang.org/1.86.0/alloc/string/struct.String.html "struct alloc::string::String"){.struct} {#fn-nameself---string .code-header}
:::

::: docblock
Returns the name of the strategy.

This method attempts to retrieve the strategy name from the `info()`
method. If `info()` returns an error (indicating it's not implemented
for the specific strategy), it asserts with a message and returns
"Unknown".

##### [§](#returns-2){.doc-anchor}Returns

The name of the strategy as a `String`.
:::
:::::::::

## Implementors[§](#implementors){.anchor} {#implementors .section-header}

:::::::::::::::::: {#implementors-list}
::: {#impl-Strategable-for-BearCallSpread .section .impl}
[Source](../../../src/optionstratlib/strategies/bear_call_spread.rs.html#448-456){.src
.rightside}[§](#impl-Strategable-for-BearCallSpread){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [BearCallSpread](../bear_call_spread/struct.BearCallSpread.html "struct optionstratlib::strategies::bear_call_spread::BearCallSpread"){.struct} {#impl-strategable-for-bearcallspread .code-header}
:::

::: {#impl-Strategable-for-BearPutSpread .section .impl}
[Source](../../../src/optionstratlib/strategies/bear_put_spread.rs.html#428-436){.src
.rightside}[§](#impl-Strategable-for-BearPutSpread){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [BearPutSpread](../bear_put_spread/struct.BearPutSpread.html "struct optionstratlib::strategies::bear_put_spread::BearPutSpread"){.struct} {#impl-strategable-for-bearputspread .code-header}
:::

::: {#impl-Strategable-for-BullCallSpread .section .impl}
[Source](../../../src/optionstratlib/strategies/bull_call_spread.rs.html#437-445){.src
.rightside}[§](#impl-Strategable-for-BullCallSpread){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [BullCallSpread](../bull_call_spread/struct.BullCallSpread.html "struct optionstratlib::strategies::bull_call_spread::BullCallSpread"){.struct} {#impl-strategable-for-bullcallspread .code-header}
:::

::: {#impl-Strategable-for-BullPutSpread .section .impl}
[Source](../../../src/optionstratlib/strategies/bull_put_spread.rs.html#443-451){.src
.rightside}[§](#impl-Strategable-for-BullPutSpread){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [BullPutSpread](../bull_put_spread/struct.BullPutSpread.html "struct optionstratlib::strategies::bull_put_spread::BullPutSpread"){.struct} {#impl-strategable-for-bullputspread .code-header}
:::

::: {#impl-Strategable-for-LongButterflySpread .section .impl}
[Source](../../../src/optionstratlib/strategies/butterfly_spread.rs.html#523-531){.src
.rightside}[§](#impl-Strategable-for-LongButterflySpread){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [LongButterflySpread](../butterfly_spread/struct.LongButterflySpread.html "struct optionstratlib::strategies::butterfly_spread::LongButterflySpread"){.struct} {#impl-strategable-for-longbutterflyspread .code-header}
:::

::: {#impl-Strategable-for-ShortButterflySpread .section .impl}
[Source](../../../src/optionstratlib/strategies/butterfly_spread.rs.html#1499-1507){.src
.rightside}[§](#impl-Strategable-for-ShortButterflySpread){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [ShortButterflySpread](../butterfly_spread/struct.ShortButterflySpread.html "struct optionstratlib::strategies::butterfly_spread::ShortButterflySpread"){.struct} {#impl-strategable-for-shortbutterflyspread .code-header}
:::

::: {#impl-Strategable-for-CallButterfly .section .impl}
[Source](../../../src/optionstratlib/strategies/call_butterfly.rs.html#521-529){.src
.rightside}[§](#impl-Strategable-for-CallButterfly){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [CallButterfly](../call_butterfly/struct.CallButterfly.html "struct optionstratlib::strategies::call_butterfly::CallButterfly"){.struct} {#impl-strategable-for-callbutterfly .code-header}
:::

::: {#impl-Strategable-for-CustomStrategy .section .impl}
[Source](../../../src/optionstratlib/strategies/custom.rs.html#378-386){.src
.rightside}[§](#impl-Strategable-for-CustomStrategy){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [CustomStrategy](../custom/struct.CustomStrategy.html "struct optionstratlib::strategies::custom::CustomStrategy"){.struct} {#impl-strategable-for-customstrategy .code-header}
:::

::: {#impl-Strategable-for-IronButterfly .section .impl}
[Source](../../../src/optionstratlib/strategies/iron_butterfly.rs.html#582-590){.src
.rightside}[§](#impl-Strategable-for-IronButterfly){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [IronButterfly](../iron_butterfly/struct.IronButterfly.html "struct optionstratlib::strategies::iron_butterfly::IronButterfly"){.struct} {#impl-strategable-for-ironbutterfly .code-header}
:::

::: {#impl-Strategable-for-IronCondor .section .impl}
[Source](../../../src/optionstratlib/strategies/iron_condor.rs.html#591-599){.src
.rightside}[§](#impl-Strategable-for-IronCondor){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [IronCondor](../iron_condor/struct.IronCondor.html "struct optionstratlib::strategies::iron_condor::IronCondor"){.struct} {#impl-strategable-for-ironcondor .code-header}
:::

::: {#impl-Strategable-for-PoorMansCoveredCall .section .impl}
[Source](../../../src/optionstratlib/strategies/poor_mans_covered_call.rs.html#468-476){.src
.rightside}[§](#impl-Strategable-for-PoorMansCoveredCall){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [PoorMansCoveredCall](../poor_mans_covered_call/struct.PoorMansCoveredCall.html "struct optionstratlib::strategies::poor_mans_covered_call::PoorMansCoveredCall"){.struct} {#impl-strategable-for-poormanscoveredcall .code-header}
:::

::: {#impl-Strategable-for-LongStraddle .section .impl}
[Source](../../../src/optionstratlib/strategies/straddle.rs.html#1280-1288){.src
.rightside}[§](#impl-Strategable-for-LongStraddle){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [LongStraddle](../straddle/struct.LongStraddle.html "struct optionstratlib::strategies::straddle::LongStraddle"){.struct} {#impl-strategable-for-longstraddle .code-header}
:::

::: {#impl-Strategable-for-ShortStraddle .section .impl}
[Source](../../../src/optionstratlib/strategies/straddle.rs.html#475-483){.src
.rightside}[§](#impl-Strategable-for-ShortStraddle){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [ShortStraddle](../straddle/struct.ShortStraddle.html "struct optionstratlib::strategies::straddle::ShortStraddle"){.struct} {#impl-strategable-for-shortstraddle .code-header}
:::

::: {#impl-Strategable-for-LongStrangle .section .impl}
[Source](../../../src/optionstratlib/strategies/strangle.rs.html#1488-1496){.src
.rightside}[§](#impl-Strategable-for-LongStrangle){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [LongStrangle](../strangle/struct.LongStrangle.html "struct optionstratlib::strategies::strangle::LongStrangle"){.struct} {#impl-strategable-for-longstrangle .code-header}
:::

::: {#impl-Strategable-for-ShortStrangle .section .impl}
[Source](../../../src/optionstratlib/strategies/strangle.rs.html#466-474){.src
.rightside}[§](#impl-Strategable-for-ShortStrangle){.anchor}

### impl [Strategable](trait.Strategable.html "trait optionstratlib::strategies::base::Strategable"){.trait} for [ShortStrangle](../strangle/struct.ShortStrangle.html "struct optionstratlib::strategies::strangle::ShortStrangle"){.struct} {#impl-strategable-for-shortstrangle .code-header}
:::
::::::::::::::::::
:::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::
