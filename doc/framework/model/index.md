:::::::: width-limiter
::::::: {#main-content .section .content}
:::: main-heading
::: rustdoc-breadcrumbs
[optionstratlib](../index.html)
:::

# Module modelCopy item path

[[Source](../../src/optionstratlib/model/mod.rs.html#7-129){.src}
]{.sub-heading}
::::

Expand description

:::: docblock
- `model` - Core data structures and models for options and derivatives.

Defines the fundamental data types and structures used throughout the
library, including option contract representations, position tracking,
and market data models. Serves as the foundation for all other modules.

## [§](#model-module){.doc-anchor}Model Module

This module provides core data structures and implementations for
financial options modeling. It includes fundamental components for
option pricing, position management, and type definitions.

### [§](#core-components){.doc-anchor}Core Components

- `option` - Implementation of the core Options structure and related
  functionality
- `position` - Management of financial positions and their properties
- `types` - Essential type definitions and enums
- `utils` - Utility functions for model operations and calculations
- `format` - Display and Debug implementations for model types
- `profit_range` - Calculations for profit/loss ranges

### [§](#key-features){.doc-anchor}Key Features

#### [§](#options){.doc-anchor}Options

Comprehensive implementation of financial options including:

- Multiple option types (European, American, Asian, etc.)
- Greeks calculation (Delta, Gamma, Theta, etc.)
- Option pricing using various models
- Position management and profit/loss calculations

#### [§](#position-management){.doc-anchor}Position Management

Tools for managing financial positions:

- Position tracking
- Cost basis calculations
- Profit/Loss analysis
- Break-even calculations
- Fee management

#### [§](#type-system){.doc-anchor}Type System

Robust type definitions ensuring type safety:

- `Positive` for non-negative numbers
- `ExpirationDate` handling
- Option styles and types
- Side (Long/Short) definitions

#### [§](#formatting){.doc-anchor}Formatting

Comprehensive formatting support:

- Display trait implementations for readable output
- Debug trait implementations for detailed inspection
- Consistent formatting across all types
- Custom format implementations for complex types

#### [§](#profitloss-analysis){.doc-anchor}Profit/Loss Analysis

Tools for analyzing potential outcomes:

- Profit range calculations
- Break-even point determination
- Probability calculations for price ranges
- Risk/reward analysis

### [§](#example-usage){.doc-anchor}Example Usage

::: example-wrap
``` {.rust .rust-example-rendered}
use rust_decimal_macros::dec;
use tracing::info;
use optionstratlib::Options;
use optionstratlib::model::types::{ExpirationDate, OptionStyle, OptionType, Side};
use optionstratlib::pos;
use optionstratlib::Positive;

let option = Options::new(
    OptionType::European,
    Side::Long,
    "AAPL".to_string(),
    pos!(100.0),
    ExpirationDate::Days(pos!(30.0)),
    pos!(0.2),
    pos!(1.0),
    pos!(105.0),
    dec!(0.05),
    OptionStyle::Call,
    pos!(0.01),
    None,
);

info!("Option Details: {}", option);
info!("Debug View: {:?}", option);
```
:::
::::

## Re-exports[§](#reexports){.anchor} {#reexports .section-header}

`pub use option::`[`Options`](option/struct.Options.html "struct optionstratlib::model::option::Options"){.struct}`;`

`pub use position::`[`Position`](position/struct.Position.html "struct optionstratlib::model::position::Position"){.struct}`;`

`pub use types::`[`ExpirationDate`](types/enum.ExpirationDate.html "enum optionstratlib::model::types::ExpirationDate"){.enum}`;`

`pub use types::`[`OptionStyle`](types/enum.OptionStyle.html "enum optionstratlib::model::types::OptionStyle"){.enum}`;`

`pub use types::`[`OptionType`](types/enum.OptionType.html "enum optionstratlib::model::types::OptionType"){.enum}`;`

`pub use types::`[`Side`](types/enum.Side.html "enum optionstratlib::model::types::Side"){.enum}`;`

## Modules[§](#modules){.anchor} {#modules .section-header}

[decimal](decimal/index.html "mod optionstratlib::model::decimal"){.mod}
:   Core utilities for handling decimal numbers in financial
    calculations.

[option](option/index.html "mod optionstratlib::model::option"){.mod}
:   Components for options contract modeling and analysis, including
    Greeks and pricing models.

[position](position/index.html "mod optionstratlib::model::position"){.mod}
:   Definitions and utilities for managing trading positions, including
    risk metrics and exposure tracking.

[positive](positive/index.html "mod optionstratlib::model::positive"){.mod}
:   Types and utilities for working with strictly positive numerical
    values in financial contexts.

[types](types/index.html "mod optionstratlib::model::types"){.mod}
:   Common type definitions used throughout the options strategy
    library.

[utils](utils/index.html "mod optionstratlib::model::utils"){.mod}
:   Utility functions supporting various operations across the library.

## Structs[§](#structs){.anchor} {#structs .section-header}

[ProfitLossRange](struct.ProfitLossRange.html "struct optionstratlib::model::ProfitLossRange"){.struct}
:   Represents a price range where a strategy is profitable

## Enums[§](#enums){.anchor} {#enums .section-header}

[BasicAxisTypes](enum.BasicAxisTypes.html "enum optionstratlib::model::BasicAxisTypes"){.enum}
:   Represents the basic axis types used in financial option analysis
    and visualization.
:::::::
::::::::
