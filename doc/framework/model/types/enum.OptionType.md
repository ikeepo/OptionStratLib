:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::: width-limiter
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::: {#main-content .section .content}
:::: main-heading
::: rustdoc-breadcrumbs
[optionstratlib](../../index.html)::[model](../index.html)::[types](index.html)
:::

# Enum [OptionType]{.enum}Copy item path

[[Source](../../../src/optionstratlib/model/types.rs.html#491-607){.src}
]{.sub-heading}
::::

``` {.rust .item-decl}
pub enum OptionType {
Show 15 variants    European,
    American,
    Bermuda {
        exercise_dates: Vec<f64>,
    },
    Asian {
        averaging_type: AsianAveragingType,
    },
    Barrier {
        barrier_type: BarrierType,
        barrier_level: f64,
    },
    Binary {
        binary_type: BinaryType,
    },
    Lookback {
        lookback_type: LookbackType,
    },
    Compound {
        underlying_option: Box<OptionType>,
    },
    Chooser {
        choice_date: f64,
    },
    Cliquet {
        reset_dates: Vec<f64>,
    },
    Rainbow {
        num_assets: usize,
    },
    Spread {
        second_asset: f64,
    },
    Quanto {
        exchange_rate: f64,
    },
    Exchange {
        second_asset: f64,
    },
    Power {
        exponent: f64,
    },
}
```

Expand description

::: docblock
Represents the type of option in a financial context. Options can be
categorized into various types based on their characteristics and the
conditions under which they can be exercised.
:::

## Variants[§](#variants){.anchor} {#variants .variants .section-header}

:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::: variants
::: {#variant.European .section .variant}
[§](#variant.European){.anchor}

### European {#european .code-header}
:::

::: docblock
A European option can only be exercised at the expiry date. This type of
option does not allow the holder to exercise the option before the
specified expiration date. European options are simpler to price and
analyze because their payoff is only determined at a single point in
time.
:::

::: {#variant.American .section .variant}
[§](#variant.American){.anchor}

### American {#american .code-header}
:::

::: docblock
An American option can be exercised at any time before and including the
expiry date. This provides the holder with more flexibility compared to
European options, as the holder can choose the optimal time to exercise
the option based on market conditions. The ability to exercise at any
point adds complexity to the pricing model, typically requiring binomial
or numerical methods.
:::

::: {#variant.Bermuda .section .variant}
[§](#variant.Bermuda){.anchor}

### Bermuda {#bermuda .code-header}
:::

::: docblock
A Bermuda option can be exercised on specific dates before the expiry
date. These specified dates are usually predetermined and occur at
regular intervals (e.g., monthly or quarterly). Bermuda options offer a
compromise between the flexibility of American options and the
simplicity of European options.
:::

::::: {#variant.Bermuda.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Bermuda.field.exercise_dates){.anchor
.field}`exercise_dates: `[`Vec`](https://doc.rust-lang.org/1.86.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec"){.struct}`<`[`f64`](https://doc.rust-lang.org/1.86.0/std/primitive.f64.html){.primitive}`>`]{#variant.Bermuda.field.exercise_dates
.section-header}

::: docblock
The specific dates on which the option can be exercised before expiry.
:::
::::
:::::

::: {#variant.Asian .section .variant}
[§](#variant.Asian){.anchor}

### Asian {#asian .code-header}
:::

::: docblock
An Asian option, where the payoff depends on the average price of the
underlying asset over a certain period. There are two types of averaging
methods: arithmetic and geometric. Asian options are useful for reducing
the risk of market manipulation at the expiry date and are common in
commodities markets.
:::

::::: {#variant.Asian.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Asian.field.averaging_type){.anchor
.field}`averaging_type: `[`AsianAveragingType`](enum.AsianAveragingType.html "enum optionstratlib::model::types::AsianAveragingType"){.enum}]{#variant.Asian.field.averaging_type
.section-header}

::: docblock
The method used to calculate the average price (arithmetic or
geometric).
:::
::::
:::::

::: {#variant.Barrier .section .variant}
[§](#variant.Barrier){.anchor}

### Barrier {#barrier .code-header}
:::

::: docblock
A Barrier option becomes active or inactive only if the underlying asset
reaches a certain barrier level. These options can be classified into
knock-in or knock-out, and further into up-and-in, up-and-out,
down-and-in, and down-and-out. Barrier options are used for hedging
strategies and typically have lower premiums compared to standard
options.
:::

::::::: {#variant.Barrier.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Barrier.field.barrier_type){.anchor
.field}`barrier_type: `[`BarrierType`](enum.BarrierType.html "enum optionstratlib::model::types::BarrierType"){.enum}]{#variant.Barrier.field.barrier_type
.section-header}

::: docblock
The type of barrier that triggers the option's activation or
deactivation.
:::
::::

:::: sub-variant-field
[[§](#variant.Barrier.field.barrier_level){.anchor
.field}`barrier_level: `[`f64`](https://doc.rust-lang.org/1.86.0/std/primitive.f64.html){.primitive}]{#variant.Barrier.field.barrier_level
.section-header}

::: docblock
The specific level that the underlying asset must reach for the barrier
to be triggered.
:::
::::
:::::::

::: {#variant.Binary .section .variant}
[§](#variant.Binary){.anchor}

### Binary {#binary .code-header}
:::

::: docblock
A Binary option provides a fixed payoff if the underlying asset is above
or below a certain level at expiry. Also known as digital options, they
include cash-or-nothing and asset-or-nothing types. Binary options are
simpler to understand but can be riskier due to their all-or-nothing
payoff structure.
:::

::::: {#variant.Binary.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Binary.field.binary_type){.anchor
.field}`binary_type: `[`BinaryType`](enum.BinaryType.html "enum optionstratlib::model::types::BinaryType"){.enum}]{#variant.Binary.field.binary_type
.section-header}

::: docblock
The specific type of binary option.
:::
::::
:::::

::: {#variant.Lookback .section .variant}
[§](#variant.Lookback){.anchor}

### Lookback {#lookback .code-header}
:::

::: docblock
A Lookback option allows the holder to "look back" over time and
determine the payoff based on the maximum or minimum underlying asset
price during the option's life. There are two main types: fixed strike,
where the strike price is set at the beginning, and floating strike,
where the strike price is set at the maximum or minimum observed price.
Lookback options are useful for maximizing profit and are typically more
expensive due to their enhanced payoff structure.
:::

::::: {#variant.Lookback.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Lookback.field.lookback_type){.anchor
.field}`lookback_type: `[`LookbackType`](enum.LookbackType.html "enum optionstratlib::model::types::LookbackType"){.enum}]{#variant.Lookback.field.lookback_type
.section-header}

::: docblock
The specific type of lookback option.
:::
::::
:::::

::: {#variant.Compound .section .variant}
[§](#variant.Compound){.anchor}

### Compound {#compound .code-header}
:::

::: docblock
A Compound option has an option as its underlying asset. This means the
holder has the right to buy or sell another option. Compound options can
be nested, adding layers of optionality and complexity, and are useful
in structured finance and corporate finance.
:::

::::: {#variant.Compound.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Compound.field.underlying_option){.anchor
.field}`underlying_option: `[`Box`](https://doc.rust-lang.org/1.86.0/alloc/boxed/struct.Box.html "struct alloc::boxed::Box"){.struct}`<`[`OptionType`](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum}`>`]{#variant.Compound.field.underlying_option
.section-header}

::: docblock
The underlying option, which can be any type of option, adding a layer
of complexity.
:::
::::
:::::

::: {#variant.Chooser .section .variant}
[§](#variant.Chooser){.anchor}

### Chooser {#chooser .code-header}
:::

::: docblock
A Chooser option allows the holder to choose, at a certain date, whether
the option will be a call or a put. This flexibility allows the holder
to make a decision based on the prevailing market conditions at the
choice date. Chooser options are valuable in volatile markets but can be
expensive due to their flexibility.
:::

::::: {#variant.Chooser.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Chooser.field.choice_date){.anchor
.field}`choice_date: `[`f64`](https://doc.rust-lang.org/1.86.0/std/primitive.f64.html){.primitive}]{#variant.Chooser.field.choice_date
.section-header}

::: docblock
The specific date on which the holder must choose whether the option
becomes a call or a put.
:::
::::
:::::

::: {#variant.Cliquet .section .variant}
[§](#variant.Cliquet){.anchor}

### Cliquet {#cliquet .code-header}
:::

::: docblock
A Cliquet option, also known as a ratchet option, resets its strike
price at certain dates. This allows the option to capture gains
periodically, locking in profits and reducing downside risk. Cliquet
options are complex and often used in structured products and guaranteed
equity bonds.
:::

::::: {#variant.Cliquet.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Cliquet.field.reset_dates){.anchor
.field}`reset_dates: `[`Vec`](https://doc.rust-lang.org/1.86.0/alloc/vec/struct.Vec.html "struct alloc::vec::Vec"){.struct}`<`[`f64`](https://doc.rust-lang.org/1.86.0/std/primitive.f64.html){.primitive}`>`]{#variant.Cliquet.field.reset_dates
.section-header}

::: docblock
The specific dates on which the strike price is reset.
:::
::::
:::::

::: {#variant.Rainbow .section .variant}
[§](#variant.Rainbow){.anchor}

### Rainbow {#rainbow .code-header}
:::

::: docblock
A Rainbow option is based on the performance of two or more underlying
assets. The payoff is typically based on the best or worst performing
asset, or a combination of their performances. Rainbow options are
useful for diversifying risk across multiple assets and are common in
multi-asset portfolios.
:::

::::: {#variant.Rainbow.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Rainbow.field.num_assets){.anchor
.field}`num_assets: `[`usize`](https://doc.rust-lang.org/1.86.0/std/primitive.usize.html){.primitive}]{#variant.Rainbow.field.num_assets
.section-header}

::: docblock
The number of underlying assets the option is based on.
:::
::::
:::::

::: {#variant.Spread .section .variant}
[§](#variant.Spread){.anchor}

### Spread {#spread .code-header}
:::

::: docblock
A Spread option is based on the difference between the prices of two
underlying assets. These options are used to profit from the relative
performance of two assets, often in the same sector or market. Spread
options can be used for arbitrage opportunities and to hedge against
relative price movements.
:::

::::: {#variant.Spread.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Spread.field.second_asset){.anchor
.field}`second_asset: `[`f64`](https://doc.rust-lang.org/1.86.0/std/primitive.f64.html){.primitive}]{#variant.Spread.field.second_asset
.section-header}

::: docblock
The price of the second asset involved in the spread.
:::
::::
:::::

::: {#variant.Quanto .section .variant}
[§](#variant.Quanto){.anchor}

### Quanto {#quanto .code-header}
:::

::: docblock
A Quanto option has a payoff that depends on the underlying asset price
in one currency, but the payoff is made in another currency at a fixed
exchange rate. This type of option eliminates the currency risk for
investors in a different currency zone. Quanto options are common in
international markets where investors seek exposure to foreign assets
without taking on currency risk.
:::

::::: {#variant.Quanto.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Quanto.field.exchange_rate){.anchor
.field}`exchange_rate: `[`f64`](https://doc.rust-lang.org/1.86.0/std/primitive.f64.html){.primitive}]{#variant.Quanto.field.exchange_rate
.section-header}

::: docblock
The fixed exchange rate at which the payoff is converted.
:::
::::
:::::

::: {#variant.Exchange .section .variant}
[§](#variant.Exchange){.anchor}

### Exchange {#exchange .code-header}
:::

::: docblock
An Exchange option gives the holder the right to exchange one asset for
another. These options are often used in mergers and acquisitions, where
one company's stock can be exchanged for another's. Exchange options
provide flexibility in managing different asset exposures and can be
tailored for specific corporate events.
:::

::::: {#variant.Exchange.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Exchange.field.second_asset){.anchor
.field}`second_asset: `[`f64`](https://doc.rust-lang.org/1.86.0/std/primitive.f64.html){.primitive}]{#variant.Exchange.field.second_asset
.section-header}

::: docblock
The price of the second asset involved in the exchange.
:::
::::
:::::

::: {#variant.Power .section .variant}
[§](#variant.Power){.anchor}

### Power {#power .code-header}
:::

::: docblock
A Power option has a payoff based on the underlying asset price raised
to a certain power. This can amplify the gains (or losses) based on the
underlying asset's performance. Power options are exotic derivatives and
are used for speculative purposes and in scenarios where large movements
in the underlying asset are expected.
:::

::::: {#variant.Power.fields .sub-variant}
#### Fields

:::: sub-variant-field
[[§](#variant.Power.field.exponent){.anchor
.field}`exponent: `[`f64`](https://doc.rust-lang.org/1.86.0/std/primitive.f64.html){.primitive}]{#variant.Power.field.exponent
.section-header}

::: docblock
The exponent to which the underlying asset price is raised.
:::
::::
:::::
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::

## Trait Implementations[§](#trait-implementations){.anchor} {#trait-implementations .section-header}

:::::::::::::::::::::::::::::::::::::: {#trait-implementations-list}
::: {#impl-Clone-for-OptionType .section .impl}
[Source](../../../src/optionstratlib/model/types.rs.html#490){.src
.rightside}[§](#impl-Clone-for-OptionType){.anchor}

### impl [Clone](https://doc.rust-lang.org/1.86.0/core/clone/trait.Clone.html "trait core::clone::Clone"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-clone-for-optiontype .code-header}
:::

::::::: impl-items
::: {#method.clone .section .method .trait-impl}
[Source](../../../src/optionstratlib/model/types.rs.html#490){.src
.rightside}[§](#method.clone){.anchor}

#### fn [clone](https://doc.rust-lang.org/1.86.0/core/clone/trait.Clone.html#tymethod.clone){.fn}(&self) -\> [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#fn-cloneself---optiontype .code-header}
:::

::: docblock
Returns a copy of the value. [Read
more](https://doc.rust-lang.org/1.86.0/core/clone/trait.Clone.html#tymethod.clone)
:::

::: {#method.clone_from .section .method .trait-impl}
[[1.0.0]{.since title="Stable since Rust version 1.0.0"} ·
[Source](https://doc.rust-lang.org/1.86.0/src/core/clone.rs.html#174){.src}]{.rightside}[§](#method.clone_from){.anchor}

#### fn [clone_from](https://doc.rust-lang.org/1.86.0/core/clone/trait.Clone.html#method.clone_from){.fn}(&mut self, source: &Self) {#fn-clone_frommut-self-source-self .code-header}
:::

::: docblock
Performs copy-assignment from `source`. [Read
more](https://doc.rust-lang.org/1.86.0/core/clone/trait.Clone.html#method.clone_from)
:::
:::::::

::: {#impl-Debug-for-OptionType .section .impl}
[Source](../../../src/optionstratlib/model/types.rs.html#490){.src
.rightside}[§](#impl-Debug-for-OptionType){.anchor}

### impl [Debug](https://doc.rust-lang.org/1.86.0/core/fmt/trait.Debug.html "trait core::fmt::Debug"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-debug-for-optiontype .code-header}
:::

::::: impl-items
::: {#method.fmt-1 .section .method .trait-impl}
[Source](../../../src/optionstratlib/model/types.rs.html#490){.src
.rightside}[§](#method.fmt-1){.anchor}

#### fn [fmt](https://doc.rust-lang.org/1.86.0/core/fmt/trait.Debug.html#tymethod.fmt){.fn}(&self, f: &mut [Formatter](https://doc.rust-lang.org/1.86.0/core/fmt/struct.Formatter.html "struct core::fmt::Formatter"){.struct}\<\'\_\>) -\> [Result](https://doc.rust-lang.org/1.86.0/core/fmt/type.Result.html "type core::fmt::Result"){.type} {#fn-fmtself-f-mut-formatter_---result .code-header}
:::

::: docblock
Formats the value using the given formatter. [Read
more](https://doc.rust-lang.org/1.86.0/core/fmt/trait.Debug.html#tymethod.fmt)
:::
:::::

::: {#impl-Deserialize%3C'de%3E-for-OptionType .section .impl}
[Source](../../../src/optionstratlib/model/types.rs.html#490){.src
.rightside}[§](#impl-Deserialize%3C'de%3E-for-OptionType){.anchor}

### impl\<\'de\> [Deserialize](https://docs.rs/serde/1.0.219/serde/de/trait.Deserialize.html "trait serde::de::Deserialize"){.trait}\<\'de\> for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#implde-deserializede-for-optiontype .code-header}
:::

:::::: impl-items
:::: {#method.deserialize .section .method .trait-impl}
[Source](../../../src/optionstratlib/model/types.rs.html#490){.src
.rightside}[§](#method.deserialize){.anchor}

#### fn [deserialize](https://docs.rs/serde/1.0.219/serde/de/trait.Deserialize.html#tymethod.deserialize){.fn}\<\_\_D\>(\_\_deserializer: \_\_D) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<Self, \_\_D::[Error](https://docs.rs/serde/1.0.219/serde/de/trait.Deserializer.html#associatedtype.Error "type serde::de::Deserializer::Error"){.associatedtype}\> {#fn-deserialize__d__deserializer-__d---resultself-__derror .code-header}

::: where
where \_\_D:
[Deserializer](https://docs.rs/serde/1.0.219/serde/de/trait.Deserializer.html "trait serde::de::Deserializer"){.trait}\<\'de\>,
:::
::::

::: docblock
Deserialize this value from the given Serde deserializer. [Read
more](https://docs.rs/serde/1.0.219/serde/de/trait.Deserialize.html#tymethod.deserialize)
:::
::::::

::: {#impl-Display-for-OptionType .section .impl}
[Source](../../../src/optionstratlib/model/format.rs.html#151-200){.src
.rightside}[§](#impl-Display-for-OptionType){.anchor}

### impl [Display](https://doc.rust-lang.org/1.86.0/core/fmt/trait.Display.html "trait core::fmt::Display"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-display-for-optiontype .code-header}
:::

::::: impl-items
::: {#method.fmt .section .method .trait-impl}
[Source](../../../src/optionstratlib/model/format.rs.html#152-199){.src
.rightside}[§](#method.fmt){.anchor}

#### fn [fmt](https://doc.rust-lang.org/1.86.0/core/fmt/trait.Display.html#tymethod.fmt){.fn}(&self, f: &mut [Formatter](https://doc.rust-lang.org/1.86.0/core/fmt/struct.Formatter.html "struct core::fmt::Formatter"){.struct}\<\'\_\>) -\> [Result](https://doc.rust-lang.org/1.86.0/core/fmt/type.Result.html "type core::fmt::Result"){.type} {#fn-fmtself-f-mut-formatter_---result-1 .code-header}
:::

::: docblock
Formats the value using the given formatter. [Read
more](https://doc.rust-lang.org/1.86.0/core/fmt/trait.Display.html#tymethod.fmt)
:::
:::::

::: {#impl-PartialEq-for-OptionType .section .impl}
[Source](../../../src/optionstratlib/model/types.rs.html#490){.src
.rightside}[§](#impl-PartialEq-for-OptionType){.anchor}

### impl [PartialEq](https://doc.rust-lang.org/1.86.0/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-partialeq-for-optiontype .code-header}
:::

::::::: impl-items
::: {#method.eq .section .method .trait-impl}
[Source](../../../src/optionstratlib/model/types.rs.html#490){.src
.rightside}[§](#method.eq){.anchor}

#### fn [eq](https://doc.rust-lang.org/1.86.0/core/cmp/trait.PartialEq.html#tymethod.eq){.fn}(&self, other: &[OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum}) -\> [bool](https://doc.rust-lang.org/1.86.0/std/primitive.bool.html){.primitive} {#fn-eqself-other-optiontype---bool .code-header}
:::

::: docblock
Tests for `self` and `other` values to be equal, and is used by `==`.
:::

::: {#method.ne .section .method .trait-impl}
[[1.0.0]{.since title="Stable since Rust version 1.0.0"} ·
[Source](https://doc.rust-lang.org/1.86.0/src/core/cmp.rs.html#261){.src}]{.rightside}[§](#method.ne){.anchor}

#### fn [ne](https://doc.rust-lang.org/1.86.0/core/cmp/trait.PartialEq.html#method.ne){.fn}(&self, other: [&Rhs](https://doc.rust-lang.org/1.86.0/std/primitive.reference.html){.primitive}) -\> [bool](https://doc.rust-lang.org/1.86.0/std/primitive.bool.html){.primitive} {#fn-neself-other-rhs---bool .code-header}
:::

::: docblock
Tests for `!=`. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
:::
:::::::

::: {#impl-Payoff-for-OptionType .section .impl}
[Source](../../../src/optionstratlib/model/types.rs.html#655-692){.src
.rightside}[§](#impl-Payoff-for-OptionType){.anchor}

### impl [Payoff](../../pricing/trait.Payoff.html "trait optionstratlib::pricing::Payoff"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-payoff-for-optiontype .code-header}
:::

::::: impl-items
::: {#method.payoff .section .method .trait-impl}
[Source](../../../src/optionstratlib/model/types.rs.html#656-691){.src
.rightside}[§](#method.payoff){.anchor}

#### fn [payoff](../../pricing/trait.Payoff.html#tymethod.payoff){.fn}(&self, info: &[PayoffInfo](../../pricing/struct.PayoffInfo.html "struct optionstratlib::pricing::PayoffInfo"){.struct}) -\> [f64](https://doc.rust-lang.org/1.86.0/std/primitive.f64.html){.primitive} {#fn-payoffself-info-payoffinfo---f64 .code-header}
:::

::: docblock
Calculates the payoff value of an option based on the provided
information. [Read
more](../../pricing/trait.Payoff.html#tymethod.payoff)
:::
:::::

::: {#impl-Serialize-for-OptionType .section .impl}
[Source](../../../src/optionstratlib/model/types.rs.html#490){.src
.rightside}[§](#impl-Serialize-for-OptionType){.anchor}

### impl [Serialize](https://docs.rs/serde/1.0.219/serde/ser/trait.Serialize.html "trait serde::ser::Serialize"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-serialize-for-optiontype .code-header}
:::

:::::: impl-items
:::: {#method.serialize .section .method .trait-impl}
[Source](../../../src/optionstratlib/model/types.rs.html#490){.src
.rightside}[§](#method.serialize){.anchor}

#### fn [serialize](https://docs.rs/serde/1.0.219/serde/ser/trait.Serialize.html#tymethod.serialize){.fn}\<\_\_S\>(&self, \_\_serializer: \_\_S) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<\_\_S::[Ok](https://docs.rs/serde/1.0.219/serde/ser/trait.Serializer.html#associatedtype.Ok "type serde::ser::Serializer::Ok"){.associatedtype}, \_\_S::[Error](https://docs.rs/serde/1.0.219/serde/ser/trait.Serializer.html#associatedtype.Error "type serde::ser::Serializer::Error"){.associatedtype}\> {#fn-serialize__sself-__serializer-__s---result__sok-__serror .code-header}

::: where
where \_\_S:
[Serializer](https://docs.rs/serde/1.0.219/serde/ser/trait.Serializer.html "trait serde::ser::Serializer"){.trait},
:::
::::

::: docblock
Serialize this value into the given Serde serializer. [Read
more](https://docs.rs/serde/1.0.219/serde/ser/trait.Serialize.html#tymethod.serialize)
:::
::::::

::: {#impl-StructuralPartialEq-for-OptionType .section .impl}
[Source](../../../src/optionstratlib/model/types.rs.html#490){.src
.rightside}[§](#impl-StructuralPartialEq-for-OptionType){.anchor}

### impl [StructuralPartialEq](https://doc.rust-lang.org/1.86.0/core/marker/trait.StructuralPartialEq.html "trait core::marker::StructuralPartialEq"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-structuralpartialeq-for-optiontype .code-header}
:::
::::::::::::::::::::::::::::::::::::::

## Auto Trait Implementations[§](#synthetic-implementations){.anchor} {#synthetic-implementations .section-header}

::::::::: {#synthetic-implementations-list}
::: {#impl-Freeze-for-OptionType .section .impl}
[§](#impl-Freeze-for-OptionType){.anchor}

### impl [Freeze](https://doc.rust-lang.org/1.86.0/core/marker/trait.Freeze.html "trait core::marker::Freeze"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-freeze-for-optiontype .code-header}
:::

::: {#impl-RefUnwindSafe-for-OptionType .section .impl}
[§](#impl-RefUnwindSafe-for-OptionType){.anchor}

### impl [RefUnwindSafe](https://doc.rust-lang.org/1.86.0/core/panic/unwind_safe/trait.RefUnwindSafe.html "trait core::panic::unwind_safe::RefUnwindSafe"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-refunwindsafe-for-optiontype .code-header}
:::

::: {#impl-Send-for-OptionType .section .impl}
[§](#impl-Send-for-OptionType){.anchor}

### impl [Send](https://doc.rust-lang.org/1.86.0/core/marker/trait.Send.html "trait core::marker::Send"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-send-for-optiontype .code-header}
:::

::: {#impl-Sync-for-OptionType .section .impl}
[§](#impl-Sync-for-OptionType){.anchor}

### impl [Sync](https://doc.rust-lang.org/1.86.0/core/marker/trait.Sync.html "trait core::marker::Sync"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-sync-for-optiontype .code-header}
:::

::: {#impl-Unpin-for-OptionType .section .impl}
[§](#impl-Unpin-for-OptionType){.anchor}

### impl [Unpin](https://doc.rust-lang.org/1.86.0/core/marker/trait.Unpin.html "trait core::marker::Unpin"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-unpin-for-optiontype .code-header}
:::

::: {#impl-UnwindSafe-for-OptionType .section .impl}
[§](#impl-UnwindSafe-for-OptionType){.anchor}

### impl [UnwindSafe](https://doc.rust-lang.org/1.86.0/core/panic/unwind_safe/trait.UnwindSafe.html "trait core::panic::unwind_safe::UnwindSafe"){.trait} for [OptionType](enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum} {#impl-unwindsafe-for-optiontype .code-header}
:::
:::::::::

## Blanket Implementations[§](#blanket-implementations){.anchor} {#blanket-implementations .section-header}

:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::: {#blanket-implementations-list}
:::: {#impl-Any-for-T .section .impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/any.rs.html#138){.src
.rightside}[§](#impl-Any-for-T){.anchor}

### impl\<T\> [Any](https://doc.rust-lang.org/1.86.0/core/any/trait.Any.html "trait core::any::Any"){.trait} for T {#implt-any-for-t .code-header}

::: where
where T: \'static +
?[Sized](https://doc.rust-lang.org/1.86.0/core/marker/trait.Sized.html "trait core::marker::Sized"){.trait},
:::
::::

::::: impl-items
::: {#method.type_id .section .method .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/any.rs.html#139){.src
.rightside}[§](#method.type_id){.anchor}

#### fn [type_id](https://doc.rust-lang.org/1.86.0/core/any/trait.Any.html#tymethod.type_id){.fn}(&self) -\> [TypeId](https://doc.rust-lang.org/1.86.0/core/any/struct.TypeId.html "struct core::any::TypeId"){.struct} {#fn-type_idself---typeid .code-header}
:::

::: docblock
Gets the `TypeId` of `self`. [Read
more](https://doc.rust-lang.org/1.86.0/core/any/trait.Any.html#tymethod.type_id)
:::
:::::

:::: {#impl-Borrow%3CT%3E-for-T .section .impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/borrow.rs.html#209){.src
.rightside}[§](#impl-Borrow%3CT%3E-for-T){.anchor}

### impl\<T\> [Borrow](https://doc.rust-lang.org/1.86.0/core/borrow/trait.Borrow.html "trait core::borrow::Borrow"){.trait}\<T\> for T {#implt-borrowt-for-t .code-header}

::: where
where T:
?[Sized](https://doc.rust-lang.org/1.86.0/core/marker/trait.Sized.html "trait core::marker::Sized"){.trait},
:::
::::

::::: impl-items
::: {#method.borrow .section .method .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/borrow.rs.html#211){.src
.rightside}[§](#method.borrow){.anchor}

#### fn [borrow](https://doc.rust-lang.org/1.86.0/core/borrow/trait.Borrow.html#tymethod.borrow){.fn}(&self) -\> [&T](https://doc.rust-lang.org/1.86.0/std/primitive.reference.html){.primitive} {#fn-borrowself---t .code-header}
:::

::: docblock
Immutably borrows from an owned value. [Read
more](https://doc.rust-lang.org/1.86.0/core/borrow/trait.Borrow.html#tymethod.borrow)
:::
:::::

:::: {#impl-BorrowMut%3CT%3E-for-T .section .impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/borrow.rs.html#217){.src
.rightside}[§](#impl-BorrowMut%3CT%3E-for-T){.anchor}

### impl\<T\> [BorrowMut](https://doc.rust-lang.org/1.86.0/core/borrow/trait.BorrowMut.html "trait core::borrow::BorrowMut"){.trait}\<T\> for T {#implt-borrowmutt-for-t .code-header}

::: where
where T:
?[Sized](https://doc.rust-lang.org/1.86.0/core/marker/trait.Sized.html "trait core::marker::Sized"){.trait},
:::
::::

::::: impl-items
::: {#method.borrow_mut .section .method .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/borrow.rs.html#218){.src
.rightside}[§](#method.borrow_mut){.anchor}

#### fn [borrow_mut](https://doc.rust-lang.org/1.86.0/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut){.fn}(&mut self) -\> [&mut T](https://doc.rust-lang.org/1.86.0/std/primitive.reference.html){.primitive} {#fn-borrow_mutmut-self---mut-t .code-header}
:::

::: docblock
Mutably borrows from an owned value. [Read
more](https://doc.rust-lang.org/1.86.0/core/borrow/trait.BorrowMut.html#tymethod.borrow_mut)
:::
:::::

:::: {#impl-CloneToUninit-for-T .section .impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/clone.rs.html#273){.src
.rightside}[§](#impl-CloneToUninit-for-T){.anchor}

### impl\<T\> [CloneToUninit](https://doc.rust-lang.org/1.86.0/core/clone/trait.CloneToUninit.html "trait core::clone::CloneToUninit"){.trait} for T {#implt-clonetouninit-for-t .code-header}

::: where
where T:
[Clone](https://doc.rust-lang.org/1.86.0/core/clone/trait.Clone.html "trait core::clone::Clone"){.trait},
:::
::::

:::::: impl-items
::: {#method.clone_to_uninit .section .method .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/clone.rs.html#275){.src
.rightside}[§](#method.clone_to_uninit){.anchor}

#### unsafe fn [clone_to_uninit](https://doc.rust-lang.org/1.86.0/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit){.fn}(&self, dst: [\*mut](https://doc.rust-lang.org/1.86.0/std/primitive.pointer.html){.primitive} [u8](https://doc.rust-lang.org/1.86.0/std/primitive.u8.html){.primitive}) {#unsafe-fn-clone_to_uninitself-dst-mut-u8 .code-header}
:::

[]{.item-info}

::: {.stab .unstable}
🔬This is a nightly-only experimental API. (`clone_to_uninit`)
:::

::: docblock
Performs copy-assignment from `self` to `dst`. [Read
more](https://doc.rust-lang.org/1.86.0/core/clone/trait.CloneToUninit.html#tymethod.clone_to_uninit)
:::
::::::

::: {#impl-From%3CT%3E-for-T .section .impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/convert/mod.rs.html#767){.src
.rightside}[§](#impl-From%3CT%3E-for-T){.anchor}

### impl\<T\> [From](https://doc.rust-lang.org/1.86.0/core/convert/trait.From.html "trait core::convert::From"){.trait}\<T\> for T {#implt-fromt-for-t .code-header}
:::

::::: impl-items
::: {#method.from .section .method .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/convert/mod.rs.html#770){.src
.rightside}[§](#method.from){.anchor}

#### fn [from](https://doc.rust-lang.org/1.86.0/core/convert/trait.From.html#tymethod.from){.fn}(t: T) -\> T {#fn-fromt-t---t .code-header}
:::

::: docblock
Returns the argument unchanged.
:::
:::::

::: {#impl-Instrument-for-T .section .impl}
[§](#impl-Instrument-for-T){.anchor}

### impl\<T\> Instrument for T {#implt-instrument-for-t .code-header}
:::

::::::: impl-items
::: {#method.instrument .section .method .trait-impl}
[§](#method.instrument){.anchor}

#### fn [instrument]{.fn}(self, span: Span) -\> Instrumented\<Self\> {#fn-instrumentself-span-span---instrumentedself .code-header}
:::

::: docblock
Instruments this type with the provided \[`Span`\], returning an
`Instrumented` wrapper. Read more
:::

::: {#method.in_current_span .section .method .trait-impl}
[§](#method.in_current_span){.anchor}

#### fn [in_current_span]{.fn}(self) -\> Instrumented\<Self\> {#fn-in_current_spanself---instrumentedself .code-header}
:::

::: docblock
Instruments this type with the [current](super::Span::current())
[`Span`](crate::Span), returning an `Instrumented` wrapper. Read more
:::
:::::::

:::: {#impl-Into%3CU%3E-for-T .section .impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/convert/mod.rs.html#750-752){.src
.rightside}[§](#impl-Into%3CU%3E-for-T){.anchor}

### impl\<T, U\> [Into](https://doc.rust-lang.org/1.86.0/core/convert/trait.Into.html "trait core::convert::Into"){.trait}\<U\> for T {#implt-u-intou-for-t .code-header}

::: where
where U:
[From](https://doc.rust-lang.org/1.86.0/core/convert/trait.From.html "trait core::convert::From"){.trait}\<T\>,
:::
::::

::::: impl-items
::: {#method.into .section .method .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/convert/mod.rs.html#760){.src
.rightside}[§](#method.into){.anchor}

#### fn [into](https://doc.rust-lang.org/1.86.0/core/convert/trait.Into.html#tymethod.into){.fn}(self) -\> U {#fn-intoself---u .code-header}
:::

::: docblock
Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
[`From`](https://doc.rust-lang.org/1.86.0/core/convert/trait.From.html "trait core::convert::From")`<T> for U`
chooses to do.
:::
:::::

::: {#impl-IntoEither-for-T .section .impl}
[Source](https://docs.rs/either/1/src/either/into_either.rs.html#64){.src
.rightside}[§](#impl-IntoEither-for-T){.anchor}

### impl\<T\> [IntoEither](https://docs.rs/either/1/either/into_either/trait.IntoEither.html "trait either::into_either::IntoEither"){.trait} for T {#implt-intoeither-for-t .code-header}
:::

:::::::: impl-items
::: {#method.into_either .section .method .trait-impl}
[Source](https://docs.rs/either/1/src/either/into_either.rs.html#29){.src
.rightside}[§](#method.into_either){.anchor}

#### fn [into_either](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either){.fn}(self, into_left: [bool](https://doc.rust-lang.org/1.86.0/std/primitive.bool.html){.primitive}) -\> [Either](https://docs.rs/either/1/either/enum.Either.html "enum either::Either"){.enum}\<Self, Self\> {#fn-into_eitherself-into_left-bool---eitherself-self .code-header}
:::

::: docblock
Converts `self` into a
[`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left "variant either::Either::Left")
variant of
[`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html "enum either::Either")
if `into_left` is `true`. Converts `self` into a
[`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right "variant either::Either::Right")
variant of
[`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html "enum either::Either")
otherwise. [Read
more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either)
:::

:::: {#method.into_either_with .section .method .trait-impl}
[Source](https://docs.rs/either/1/src/either/into_either.rs.html#55-57){.src
.rightside}[§](#method.into_either_with){.anchor}

#### fn [into_either_with](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either_with){.fn}\<F\>(self, into_left: F) -\> [Either](https://docs.rs/either/1/either/enum.Either.html "enum either::Either"){.enum}\<Self, Self\> {#fn-into_either_withfself-into_left-f---eitherself-self .code-header}

::: where
where F:
[FnOnce](https://doc.rust-lang.org/1.86.0/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce"){.trait}(&Self)
-\>
[bool](https://doc.rust-lang.org/1.86.0/std/primitive.bool.html){.primitive},
:::
::::

::: docblock
Converts `self` into a
[`Left`](https://docs.rs/either/1/either/enum.Either.html#variant.Left "variant either::Either::Left")
variant of
[`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html "enum either::Either")
if `into_left(&self)` returns `true`. Converts `self` into a
[`Right`](https://docs.rs/either/1/either/enum.Either.html#variant.Right "variant either::Either::Right")
variant of
[`Either<Self, Self>`](https://docs.rs/either/1/either/enum.Either.html "enum either::Either")
otherwise. [Read
more](https://docs.rs/either/1/either/into_either/trait.IntoEither.html#method.into_either_with)
:::
::::::::

::: {#impl-Pointable-for-T .section .impl}
[§](#impl-Pointable-for-T){.anchor}

### impl\<T\> Pointable for T {#implt-pointable-for-t .code-header}
:::

::::::::::::::: impl-items
::: {#associatedconstant.ALIGN .section .associatedconstant .trait-impl}
[§](#associatedconstant.ALIGN){.anchor}

#### const [ALIGN]{.constant}: [usize](https://doc.rust-lang.org/1.86.0/std/primitive.usize.html){.primitive} {#const-align-usize .code-header}
:::

::: docblock
The alignment of pointer.
:::

::: {#associatedtype.Init .section .associatedtype .trait-impl}
[§](#associatedtype.Init){.anchor}

#### type [Init]{.associatedtype} = T {#type-init-t .code-header}
:::

::: docblock
The type for initializers.
:::

::: {#method.init .section .method .trait-impl}
[§](#method.init){.anchor}

#### unsafe fn [init]{.fn}(init: \<T as Pointable\>::Init) -\> [usize](https://doc.rust-lang.org/1.86.0/std/primitive.usize.html){.primitive} {#unsafe-fn-initinit-t-as-pointableinit---usize .code-header}
:::

::: docblock
Initializes a with the given initializer. Read more
:::

::: {#method.deref .section .method .trait-impl}
[§](#method.deref){.anchor}

#### unsafe fn [deref]{.fn}\<\'a\>(ptr: [usize](https://doc.rust-lang.org/1.86.0/std/primitive.usize.html){.primitive}) -\> [&\'a T](https://doc.rust-lang.org/1.86.0/std/primitive.reference.html){.primitive} {#unsafe-fn-derefaptr-usize---a-t .code-header}
:::

::: docblock
Dereferences the given pointer. Read more
:::

::: {#method.deref_mut .section .method .trait-impl}
[§](#method.deref_mut){.anchor}

#### unsafe fn [deref_mut]{.fn}\<\'a\>(ptr: [usize](https://doc.rust-lang.org/1.86.0/std/primitive.usize.html){.primitive}) -\> [&\'a mut T](https://doc.rust-lang.org/1.86.0/std/primitive.reference.html){.primitive} {#unsafe-fn-deref_mutaptr-usize---a-mut-t .code-header}
:::

::: docblock
Mutably dereferences the given pointer. Read more
:::

::: {#method.drop .section .method .trait-impl}
[§](#method.drop){.anchor}

#### unsafe fn [drop]{.fn}(ptr: [usize](https://doc.rust-lang.org/1.86.0/std/primitive.usize.html){.primitive}) {#unsafe-fn-dropptr-usize .code-header}
:::

::: docblock
Drops the object pointed to by the given pointer. Read more
:::
:::::::::::::::

::: {#impl-Same-for-T .section .impl}
[Source](https://docs.rs/typenum/1.18.0/src/typenum/type_operators.rs.html#34){.src
.rightside}[§](#impl-Same-for-T){.anchor}

### impl\<T\> [Same](https://docs.rs/typenum/1.18.0/typenum/type_operators/trait.Same.html "trait typenum::type_operators::Same"){.trait} for T {#implt-same-for-t .code-header}
:::

::::: impl-items
::: {#associatedtype.Output .section .associatedtype .trait-impl}
[Source](https://docs.rs/typenum/1.18.0/src/typenum/type_operators.rs.html#35){.src
.rightside}[§](#associatedtype.Output){.anchor}

#### type [Output](https://docs.rs/typenum/1.18.0/typenum/type_operators/trait.Same.html#associatedtype.Output){.associatedtype} = T {#type-output-t .code-header}
:::

::: docblock
Should always be `Self`
:::
:::::

:::: {#impl-SupersetOf%3CSS%3E-for-SP .section .impl}
[§](#impl-SupersetOf%3CSS%3E-for-SP){.anchor}

### impl\<SS, SP\> SupersetOf\<SS\> for SP {#implss-sp-supersetofss-for-sp .code-header}

::: where
where SS: SubsetOf\<SP\>,
:::
::::

::::::::::: impl-items
::: {#method.to_subset .section .method .trait-impl}
[§](#method.to_subset){.anchor}

#### fn [to_subset]{.fn}(&self) -\> [Option](https://doc.rust-lang.org/1.86.0/core/option/enum.Option.html "enum core::option::Option"){.enum}\<SS\> {#fn-to_subsetself---optionss .code-header}
:::

::: docblock
The inverse inclusion map: attempts to construct `self` from the
equivalent element of its superset. Read more
:::

::: {#method.is_in_subset .section .method .trait-impl}
[§](#method.is_in_subset){.anchor}

#### fn [is_in_subset]{.fn}(&self) -\> [bool](https://doc.rust-lang.org/1.86.0/std/primitive.bool.html){.primitive} {#fn-is_in_subsetself---bool .code-header}
:::

::: docblock
Checks if `self` is actually part of its subset `T` (and can be
converted to it).
:::

::: {#method.to_subset_unchecked .section .method .trait-impl}
[§](#method.to_subset_unchecked){.anchor}

#### fn [to_subset_unchecked]{.fn}(&self) -\> SS {#fn-to_subset_uncheckedself---ss .code-header}
:::

::: docblock
Use with care! Same as `self.to_subset` but without any property checks.
Always succeeds.
:::

::: {#method.from_subset .section .method .trait-impl}
[§](#method.from_subset){.anchor}

#### fn [from_subset]{.fn}(element: [&SS](https://doc.rust-lang.org/1.86.0/std/primitive.reference.html){.primitive}) -\> SP {#fn-from_subsetelement-ss---sp .code-header}
:::

::: docblock
The inclusion map: converts `self` to the equivalent element of its
superset.
:::
:::::::::::

:::: {#impl-ToOwned-for-T .section .impl}
[Source](https://doc.rust-lang.org/1.86.0/src/alloc/borrow.rs.html#82-84){.src
.rightside}[§](#impl-ToOwned-for-T){.anchor}

### impl\<T\> [ToOwned](https://doc.rust-lang.org/1.86.0/alloc/borrow/trait.ToOwned.html "trait alloc::borrow::ToOwned"){.trait} for T {#implt-toowned-for-t .code-header}

::: where
where T:
[Clone](https://doc.rust-lang.org/1.86.0/core/clone/trait.Clone.html "trait core::clone::Clone"){.trait},
:::
::::

::::::::: impl-items
::: {#associatedtype.Owned .section .associatedtype .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/alloc/borrow.rs.html#86){.src
.rightside}[§](#associatedtype.Owned){.anchor}

#### type [Owned](https://doc.rust-lang.org/1.86.0/alloc/borrow/trait.ToOwned.html#associatedtype.Owned){.associatedtype} = T {#type-owned-t .code-header}
:::

::: docblock
The resulting type after obtaining ownership.
:::

::: {#method.to_owned .section .method .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/alloc/borrow.rs.html#87){.src
.rightside}[§](#method.to_owned){.anchor}

#### fn [to_owned](https://doc.rust-lang.org/1.86.0/alloc/borrow/trait.ToOwned.html#tymethod.to_owned){.fn}(&self) -\> T {#fn-to_ownedself---t .code-header}
:::

::: docblock
Creates owned data from borrowed data, usually by cloning. [Read
more](https://doc.rust-lang.org/1.86.0/alloc/borrow/trait.ToOwned.html#tymethod.to_owned)
:::

::: {#method.clone_into .section .method .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/alloc/borrow.rs.html#91){.src
.rightside}[§](#method.clone_into){.anchor}

#### fn [clone_into](https://doc.rust-lang.org/1.86.0/alloc/borrow/trait.ToOwned.html#method.clone_into){.fn}(&self, target: [&mut T](https://doc.rust-lang.org/1.86.0/std/primitive.reference.html){.primitive}) {#fn-clone_intoself-target-mut-t .code-header}
:::

::: docblock
Uses borrowed data to replace owned data, usually by cloning. [Read
more](https://doc.rust-lang.org/1.86.0/alloc/borrow/trait.ToOwned.html#method.clone_into)
:::
:::::::::

:::: {#impl-ToString-for-T .section .impl}
[Source](https://doc.rust-lang.org/1.86.0/src/alloc/string.rs.html#2758){.src
.rightside}[§](#impl-ToString-for-T){.anchor}

### impl\<T\> [ToString](https://doc.rust-lang.org/1.86.0/alloc/string/trait.ToString.html "trait alloc::string::ToString"){.trait} for T {#implt-tostring-for-t .code-header}

::: where
where T:
[Display](https://doc.rust-lang.org/1.86.0/core/fmt/trait.Display.html "trait core::fmt::Display"){.trait} +
?[Sized](https://doc.rust-lang.org/1.86.0/core/marker/trait.Sized.html "trait core::marker::Sized"){.trait},
:::
::::

::::: impl-items
::: {#method.to_string .section .method .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/alloc/string.rs.html#2760){.src
.rightside}[§](#method.to_string){.anchor}

#### fn [to_string](https://doc.rust-lang.org/1.86.0/alloc/string/trait.ToString.html#tymethod.to_string){.fn}(&self) -\> [String](https://doc.rust-lang.org/1.86.0/alloc/string/struct.String.html "struct alloc::string::String"){.struct} {#fn-to_stringself---string .code-header}
:::

::: docblock
Converts the given value to a `String`. [Read
more](https://doc.rust-lang.org/1.86.0/alloc/string/trait.ToString.html#tymethod.to_string)
:::
:::::

:::: {#impl-TryFrom%3CU%3E-for-T .section .impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/convert/mod.rs.html#807-809){.src
.rightside}[§](#impl-TryFrom%3CU%3E-for-T){.anchor}

### impl\<T, U\> [TryFrom](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryFrom.html "trait core::convert::TryFrom"){.trait}\<U\> for T {#implt-u-tryfromu-for-t .code-header}

::: where
where U:
[Into](https://doc.rust-lang.org/1.86.0/core/convert/trait.Into.html "trait core::convert::Into"){.trait}\<T\>,
:::
::::

::::::: impl-items
::: {#associatedtype.Error-1 .section .associatedtype .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/convert/mod.rs.html#811){.src
.rightside}[§](#associatedtype.Error-1){.anchor}

#### type [Error](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryFrom.html#associatedtype.Error){.associatedtype} = [Infallible](https://doc.rust-lang.org/1.86.0/core/convert/enum.Infallible.html "enum core::convert::Infallible"){.enum} {#type-error-infallible .code-header}
:::

::: docblock
The type returned in the event of a conversion error.
:::

::: {#method.try_from .section .method .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/convert/mod.rs.html#814){.src
.rightside}[§](#method.try_from){.anchor}

#### fn [try_from](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryFrom.html#tymethod.try_from){.fn}(value: U) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<T, \<T as [TryFrom](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryFrom.html "trait core::convert::TryFrom"){.trait}\<U\>\>::[Error](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error"){.associatedtype}\> {#fn-try_fromvalue-u---resultt-t-as-tryfromuerror .code-header}
:::

::: docblock
Performs the conversion.
:::
:::::::

:::: {#impl-TryInto%3CU%3E-for-T .section .impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/convert/mod.rs.html#792-794){.src
.rightside}[§](#impl-TryInto%3CU%3E-for-T){.anchor}

### impl\<T, U\> [TryInto](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryInto.html "trait core::convert::TryInto"){.trait}\<U\> for T {#implt-u-tryintou-for-t .code-header}

::: where
where U:
[TryFrom](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryFrom.html "trait core::convert::TryFrom"){.trait}\<T\>,
:::
::::

::::::: impl-items
::: {#associatedtype.Error .section .associatedtype .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/convert/mod.rs.html#796){.src
.rightside}[§](#associatedtype.Error){.anchor}

#### type [Error](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryInto.html#associatedtype.Error){.associatedtype} = \<U as [TryFrom](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryFrom.html "trait core::convert::TryFrom"){.trait}\<T\>\>::[Error](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error"){.associatedtype} {#type-error-u-as-tryfromterror .code-header}
:::

::: docblock
The type returned in the event of a conversion error.
:::

::: {#method.try_into .section .method .trait-impl}
[Source](https://doc.rust-lang.org/1.86.0/src/core/convert/mod.rs.html#799){.src
.rightside}[§](#method.try_into){.anchor}

#### fn [try_into](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryInto.html#tymethod.try_into){.fn}(self) -\> [Result](https://doc.rust-lang.org/1.86.0/core/result/enum.Result.html "enum core::result::Result"){.enum}\<U, \<U as [TryFrom](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryFrom.html "trait core::convert::TryFrom"){.trait}\<T\>\>::[Error](https://doc.rust-lang.org/1.86.0/core/convert/trait.TryFrom.html#associatedtype.Error "type core::convert::TryFrom::Error"){.associatedtype}\> {#fn-try_intoself---resultu-u-as-tryfromterror .code-header}
:::

::: docblock
Performs the conversion.
:::
:::::::

:::: {#impl-VZip%3CV%3E-for-T .section .impl}
[§](#impl-VZip%3CV%3E-for-T){.anchor}

### impl\<V, T\> VZip\<V\> for T {#implv-t-vzipv-for-t .code-header}

::: where
where V: MultiLane\<T\>,
:::
::::

:::: impl-items
::: {#method.vzip .section .method .trait-impl}
[§](#method.vzip){.anchor}

#### fn [vzip]{.fn}(self) -\> V {#fn-vzipself---v .code-header}
:::
::::

::: {#impl-WithSubscriber-for-T .section .impl}
[§](#impl-WithSubscriber-for-T){.anchor}

### impl\<T\> WithSubscriber for T {#implt-withsubscriber-for-t .code-header}
:::

:::::::: impl-items
:::: {#method.with_subscriber .section .method .trait-impl}
[§](#method.with_subscriber){.anchor}

#### fn [with_subscriber]{.fn}\<S\>(self, subscriber: S) -\> WithDispatch\<Self\> {#fn-with_subscribersself-subscriber-s---withdispatchself .code-header}

::: where
where S:
[Into](https://doc.rust-lang.org/1.86.0/core/convert/trait.Into.html "trait core::convert::Into"){.trait}\<Dispatch\>,
:::
::::

::: docblock
Attaches the provided [`Subscriber`](super::Subscriber) to this type,
returning a \[`WithDispatch`\] wrapper. Read more
:::

::: {#method.with_current_subscriber .section .method .trait-impl}
[§](#method.with_current_subscriber){.anchor}

#### fn [with_current_subscriber]{.fn}(self) -\> WithDispatch\<Self\> {#fn-with_current_subscriberself---withdispatchself .code-header}
:::

::: docblock
Attaches the current
[default](crate::dispatcher#setting-the-default-subscriber)
[`Subscriber`](super::Subscriber) to this type, returning a
\[`WithDispatch`\] wrapper. Read more
:::
::::::::

:::: {#impl-DeserializeOwned-for-T .section .impl}
[Source](https://docs.rs/serde/1.0.219/src/serde/de/mod.rs.html#614){.src
.rightside}[§](#impl-DeserializeOwned-for-T){.anchor}

### impl\<T\> [DeserializeOwned](https://docs.rs/serde/1.0.219/serde/de/trait.DeserializeOwned.html "trait serde::de::DeserializeOwned"){.trait} for T {#implt-deserializeowned-for-t .code-header}

::: where
where T: for\<\'de\>
[Deserialize](https://docs.rs/serde/1.0.219/serde/de/trait.Deserialize.html "trait serde::de::Deserialize"){.trait}\<\'de\>,
:::
::::

:::: {#impl-Scalar-for-T .section .impl}
[Source](https://docs.rs/nalgebra/0.25.0/src/nalgebra/base/scalar.rs.html#8){.src
.rightside}[§](#impl-Scalar-for-T){.anchor}

### impl\<T\> [Scalar](https://docs.rs/nalgebra/0.25.0/nalgebra/base/scalar/trait.Scalar.html "trait nalgebra::base::scalar::Scalar"){.trait} for T {#implt-scalar-for-t .code-header}

::: where
where T: \'static +
[Clone](https://doc.rust-lang.org/1.86.0/core/clone/trait.Clone.html "trait core::clone::Clone"){.trait} +
[PartialEq](https://doc.rust-lang.org/1.86.0/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq"){.trait} +
[Debug](https://doc.rust-lang.org/1.86.0/core/fmt/trait.Debug.html "trait core::fmt::Debug"){.trait},
:::
::::
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
