/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/12/24
******************************************************************************/

//! # Greeks Error Module
//!
//! This module provides error handling for Greek calculations and equations in option pricing.
//! It defines error types for various mathematical calculations and validations used in
//! financial derivatives analysis.
//!
//! ## Error Types
//!
//! ### Greeks Error (`GreeksError`)
//! Main error enum that encompasses:
//! * Calculation errors in Greek values
//! * Input validation errors
//! * Mathematical operation errors
//! * Boundary condition errors
//!
//! ### Mathematical Error (`MathErrorKind`)
//! Handles specific mathematical errors:
//! * Division by zero
//! * Overflow conditions
//! * Invalid domain errors
//! * Convergence failures
//!
//! ### Input Validation Error (`InputErrorKind`)
//! Manages validation of input parameters:
//! * Invalid volatility values
//! * Invalid time values
//! * Invalid price values
//! * Invalid rate values

use crate::Positive;
use crate::error::VolatilityError;
use crate::error::decimal;
use std::error::Error;
use std::fmt;

/// Represents errors that can occur during options Greek calculations.
///
/// This enum encapsulates the various types of errors that might arise during
/// the calculation of option Greeks (delta, gamma, theta, vega, rho) and related
/// financial computations. It provides a structured approach to error handling
/// by categorizing errors based on their nature and source.
///
/// `GreeksError` serves as the primary error type for the Greek calculation system,
/// allowing for precise error reporting and handling across different calculation stages.
#[derive(Debug)]
pub enum GreeksError {
    /// Errors related to mathematical calculations such as division by zero,
    /// overflow, domain errors, or convergence failures.
    MathError(MathErrorKind),

    /// Errors related to input validation, including invalid volatility, time,
    /// price, interest rate, or strike price values.
    InputError(InputErrorKind),

    /// Errors specific to the calculation of individual Greeks (delta, gamma,
    /// theta, vega, rho) or other option-related computations.
    CalculationError(CalculationErrorKind),

    /// Errors originating from standard Rust error types, wrapped as strings
    /// for consistent error handling.
    StdError(String),
}

/// Represents various types of mathematical errors that can occur during calculations.
///
/// This enum provides specific error categories for mathematical operations, allowing
/// precise error handling and informative reporting. Each variant contains detailed
/// information about the specific error condition that occurred.
///
/// `MathErrorKind` serves as part of the error handling system for numerical calculations,
/// particularly in financial and statistical contexts where numerical stability and
/// precision are critical.
///
/// # Variants
///
/// * `DivisionByZero` - Represents the fundamental error of attempting to divide by zero,
///   which is mathematically undefined.
///
/// * `Overflow` - Represents errors when calculations exceed the numerical limits of
///   the underlying data type, typically with very large numbers or exponential operations.
///
/// * `InvalidDomain` - Represents errors when a mathematical function is evaluated
///   outside its valid domain, containing both the problematic value and reason.
///
/// * `ConvergenceFailure` - Represents errors when an iterative algorithm fails to
///   converge to a solution within specified parameters.
///
/// # Usage
///
/// This error type is typically used in financial models, statistical calculations,
/// and numerical algorithms where precise error identification is essential for
/// debugging and proper error handling.
///
/// ```rust
/// use optionstratlib::error::greeks::MathErrorKind;
///
/// fn calculate_square_root(value: f64) -> Result<f64, MathErrorKind> {
///     if value < 0.0 {
///         return Err(MathErrorKind::InvalidDomain {
///             value,
///             reason: "Cannot calculate square root of a negative number".to_string()
///         });
///     }
///     Ok(value.sqrt())
/// }
/// ```
#[derive(Debug)]
pub enum MathErrorKind {
    /// Error that occurs when attempting to divide by zero.
    ///
    /// This is a fundamental mathematical error that must be caught to prevent undefined behavior.
    /// In numerical calculations, division by zero is undefined and will cause program crashes
    /// if not properly handled.
    DivisionByZero,

    /// Error that occurs when a calculation exceeds the numerical limits of the data type.
    ///
    /// This typically happens with very large numbers or during exponential operations.
    /// Overflow errors can lead to incorrect results and should be caught to maintain
    /// calculation integrity.
    Overflow,

    /// Error that occurs when a function is evaluated outside its valid domain.
    ///
    /// # Fields
    /// * `value` - The input value that caused the domain error
    /// * `reason` - A descriptive explanation of why the value is invalid
    ///
    /// Domain errors are common in mathematical functions like logarithms, square roots,
    /// and trigonometric functions where certain input values are not allowed.
    InvalidDomain {
        /// The value that was outside the valid domain
        value: f64,
        /// Detailed explanation of why the value is invalid for the operation
        reason: String,
    },

    /// Error that occurs when an iterative algorithm fails to converge to a solution.
    ///
    /// # Fields
    /// * `iterations` - The number of iterations performed before failure
    /// * `tolerance` - The convergence tolerance that was not satisfied
    ///
    /// Convergence failures typically occur in numerical methods like Newton-Raphson,
    /// implied volatility calculations, or other root-finding algorithms.
    ConvergenceFailure {
        /// Number of iterations attempted before giving up
        iterations: usize,
        /// The tolerance threshold that wasn't met during convergence
        tolerance: f64,
    },
}

/// Represents different types of input validation errors that can occur during financial calculations.
///
/// This enum encapsulates various error conditions related to the validation of input parameters
/// in financial models, particularly for options pricing and risk analysis. Each variant contains
/// detailed information about the invalid input, including both the problematic value and a reason
/// explaining why it was rejected.
///
/// # Variants
///
/// * `InvalidVolatility` - Represents errors related to improper volatility values
///   such as negative values or unreasonably large inputs that would cause calculation issues.
///
/// * `InvalidTime` - Represents errors related to time inputs (typically time to expiration)
///   that are outside acceptable bounds or otherwise unsuitable for financial calculations.
///
/// * `InvalidPrice` - Represents errors related to price inputs (like underlying asset prices)
///   that are negative or otherwise invalid for the calculation context.
///
/// * `InvalidRate` - Represents errors related to interest rate values that are outside
///   acceptable bounds for the specific financial modeling context.
///
/// * `InvalidStrike` - Represents errors related to strike price inputs that are malformed,
///   out of bounds, or otherwise unsuitable for options calculations.
///
/// # Usage
///
/// These error kinds are typically used within higher-level error types to provide specific
/// information about validation failures, enabling precise error handling and informative
/// error messages for users.
#[derive(Debug)]
pub enum InputErrorKind {
    /// Error indicating an invalid volatility input.
    ///
    /// This error occurs when a volatility value is outside acceptable bounds
    /// (typically negative values or unreasonably large values) or otherwise invalid
    /// for the calculation being performed.
    InvalidVolatility {
        /// The invalid volatility value that was provided
        value: f64,
        /// Detailed explanation of why the volatility value is invalid
        reason: String,
    },

    /// Error indicating an invalid time input.
    ///
    /// This error occurs when a time value (typically representing time to expiration)
    /// is outside acceptable bounds or otherwise invalid for the calculation being performed.
    InvalidTime {
        /// The invalid time value that was provided (as a Positive type)
        value: Positive,
        /// Detailed explanation of why the time value is invalid
        reason: String,
    },

    /// Error indicating an invalid price input.
    ///
    /// This error occurs when a price value (such as an underlying asset price)
    /// is outside acceptable bounds (typically negative values) or otherwise
    /// invalid for the calculation being performed.
    InvalidPrice {
        /// The invalid price value that was provided
        value: f64,
        /// Detailed explanation of why the price value is invalid
        reason: String,
    },

    /// Error indicating an invalid interest rate input.
    ///
    /// This error occurs when an interest rate value is outside acceptable bounds
    /// or otherwise invalid for the calculation being performed.
    InvalidRate {
        /// The invalid interest rate value that was provided
        value: f64,
        /// Detailed explanation of why the rate value is invalid
        reason: String,
    },

    /// Error indicating an invalid strike price input.
    ///
    /// This error occurs when a strike price value is outside acceptable bounds,
    /// in an incorrect format, or otherwise invalid for the calculation being performed.
    InvalidStrike {
        /// The invalid strike value that was provided (as a String)
        value: String,
        /// Detailed explanation of why the strike value is invalid
        reason: String,
    },
}

/// Represents specific error types that can occur during financial derivative calculations.
///
/// This enum categorizes errors that happen during the calculation of option Greeks and other
/// financial metrics. Each variant provides detailed context about what went wrong during
/// the specific calculation, allowing for precise error handling and debugging.
///
/// The enum is designed to be used within a broader error handling system for options pricing
/// and financial calculations, providing specific error types for different aspects of
/// the derivatives pricing process.
#[derive(Debug)]
pub enum CalculationErrorKind {
    /// Error in delta calculation
    ///
    /// Delta measures the rate of change of the option price with respect to changes
    /// in the underlying asset's price.
    DeltaError {
        /// Detailed description of what caused the delta calculation to fail
        reason: String,
    },
    /// Error in gamma calculation
    ///
    /// Gamma measures the rate of change of delta with respect to changes in the
    /// underlying asset's price.
    GammaError {
        /// Detailed description of what caused the gamma calculation to fail
        reason: String,
    },
    /// Error in theta calculation
    ///
    /// Theta measures the rate of decay of the option's value over time, often
    /// referred to as time decay.
    ThetaError {
        /// Detailed description of what caused the theta calculation to fail
        reason: String,
    },
    /// Error in vega calculation
    ///
    /// Vega measures the sensitivity of the option price to changes in the
    /// underlying asset's volatility.
    VegaError {
        /// Detailed description of what caused the vega calculation to fail
        reason: String,
    },
    /// Error in rho calculation
    ///
    /// Rho measures the sensitivity of the option price to changes in the
    /// risk-free interest rate.
    RhoError {
        /// Detailed description of what caused the rho calculation to fail
        reason: String,
    },
    /// Error originating from decimal operations
    ///
    /// Wraps a decimal library error that occurred during option calculations,
    /// typically related to precision, arithmetic operations, or invalid values.
    DecimalError {
        /// The underlying decimal calculation error
        error: decimal::DecimalError,
    },
}

impl fmt::Display for GreeksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GreeksError::MathError(err) => write!(f, "Mathematical error: {}", err),
            GreeksError::InputError(err) => write!(f, "Input validation error: {}", err),
            GreeksError::CalculationError(err) => write!(f, "Greek calculation error: {}", err),
            GreeksError::StdError(msg) => write!(f, "Standard error: {}", msg),
        }
    }
}

impl fmt::Display for MathErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathErrorKind::DivisionByZero => write!(f, "Division by zero"),
            MathErrorKind::Overflow => write!(f, "Numerical overflow"),
            MathErrorKind::InvalidDomain { value, reason } => {
                write!(f, "Invalid domain value {}: {}", value, reason)
            }
            MathErrorKind::ConvergenceFailure {
                iterations,
                tolerance,
            } => {
                write!(
                    f,
                    "Failed to converge after {} iterations (tolerance: {})",
                    iterations, tolerance
                )
            }
        }
    }
}

impl fmt::Display for InputErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputErrorKind::InvalidVolatility { value, reason } => {
                write!(f, "Invalid volatility {}: {}", value, reason)
            }
            InputErrorKind::InvalidTime { value, reason } => {
                write!(f, "Invalid time value {}: {}", value, reason)
            }
            InputErrorKind::InvalidPrice { value, reason } => {
                write!(f, "Invalid price {}: {}", value, reason)
            }
            InputErrorKind::InvalidRate { value, reason } => {
                write!(f, "Invalid rate {}: {}", value, reason)
            }
            InputErrorKind::InvalidStrike { value, reason } => {
                write!(f, "Invalid strike price {}: {}", value, reason)
            }
        }
    }
}

impl fmt::Display for CalculationErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculationErrorKind::DeltaError { reason } => {
                write!(f, "Delta calculation error: {}", reason)
            }
            CalculationErrorKind::GammaError { reason } => {
                write!(f, "Gamma calculation error: {}", reason)
            }
            CalculationErrorKind::ThetaError { reason } => {
                write!(f, "Theta calculation error: {}", reason)
            }
            CalculationErrorKind::VegaError { reason } => {
                write!(f, "Vega calculation error: {}", reason)
            }
            CalculationErrorKind::RhoError { reason } => {
                write!(f, "Rho calculation error: {}", reason)
            }
            CalculationErrorKind::DecimalError { error } => write!(f, "Decimal error: {}", error),
        }
    }
}

impl Error for GreeksError {}

/// Type alias for Results returned from Greek calculation functions.
///
/// This alias wraps the standard Rust `Result` type to provide a specialized
/// result type for Greek calculations, using `GreeksError` as the error type.
///
/// # Type Parameters
///
/// * `T` - The success value type that will be returned when operations succeed.
///
/// # Related Types
///
/// This type alias is part of the error handling system for Greek calculations
/// and works with the `GreeksError` enum which provides detailed error information.
///
/// # Usage Context
///
/// Typically used in functions that calculate option Greeks (delta, gamma, theta,
/// vega, rho) and other financial metrics where specialized error handling for
/// mathematical and input validation errors is needed.
pub type GreeksResult<T> = Result<T, GreeksError>;

/// Implementation of factory methods for creating specific `GreeksError` instances.
///
/// This implementation provides convenient constructor methods for creating different types
/// of errors that can occur during options Greek calculations. These methods make error creation
/// more concise and readable in the codebase, while ensuring consistent error formatting.
impl GreeksError {
    /// Creates an error for invalid volatility values.
    ///
    /// Use this method when a volatility input is outside acceptable bounds or otherwise
    /// unsuitable for calculations. Common cases include negative values or unreasonably
    /// large volatilities.
    ///
    /// # Parameters
    /// * `value` - The invalid volatility value that triggered the error
    /// * `reason` - An explanation of why the volatility value is invalid
    ///
    /// # Returns
    /// A `GreeksError::InputError` with `InvalidVolatility` kind
    pub fn invalid_volatility(value: f64, reason: &str) -> Self {
        GreeksError::InputError(InputErrorKind::InvalidVolatility {
            value,
            reason: reason.to_string(),
        })
    }

    /// Creates an error for invalid time values.
    ///
    /// Use this method when a time input (typically representing time to expiration)
    /// is outside acceptable bounds or otherwise invalid for calculations.
    ///
    /// # Parameters
    /// * `value` - The invalid time value (as a `Positive` type) that triggered the error
    /// * `reason` - An explanation of why the time value is invalid
    ///
    /// # Returns
    /// A `GreeksError::InputError` with `InvalidTime` kind
    pub fn invalid_time(value: Positive, reason: &str) -> Self {
        GreeksError::InputError(InputErrorKind::InvalidTime {
            value,
            reason: reason.to_string(),
        })
    }

    /// Creates an error for delta calculation failures.
    ///
    /// Use this method when a calculation of the delta Greek value fails for any reason.
    /// Delta measures the rate of change of option price with respect to changes in the
    /// underlying asset price.
    ///
    /// # Parameters
    /// * `reason` - A detailed explanation of what caused the delta calculation to fail
    ///
    /// # Returns
    /// A `GreeksError::CalculationError` with `DeltaError` kind
    pub fn delta_error(reason: &str) -> Self {
        GreeksError::CalculationError(CalculationErrorKind::DeltaError {
            reason: reason.to_string(),
        })
    }
}

/// Implements conversion from `decimal::DecimalError` to `GreeksError`.
///
/// This implementation allows decimal calculation errors to be automatically converted
/// into the appropriate `GreeksError` variant, simplifying error handling when working
/// with decimal operations in financial calculations.
///
impl From<decimal::DecimalError> for GreeksError {
    fn from(error: decimal::DecimalError) -> Self {
        GreeksError::CalculationError(CalculationErrorKind::DecimalError { error })
    }
}

/// Implements conversion from `VolatilityError` to `GreeksError`.
///
/// This implementation allows volatility-related errors to be automatically converted
/// into appropriate `InputErrorKind::InvalidVolatility` errors, providing consistent
/// error handling for invalid volatility values.
///
impl From<VolatilityError> for GreeksError {
    fn from(error: VolatilityError) -> Self {
        GreeksError::InputError(InputErrorKind::InvalidVolatility {
            value: 0.0,
            reason: error.to_string(),
        })
    }
}

/// Implements conversion from `Box<dyn Error>` to `GreeksError`.
///
/// This implementation serves as a catch-all for converting any type that implements
/// the standard Error trait into a `GreeksError`. This is useful for integrating with
/// libraries or functions that return standard error types.
///
impl From<Box<dyn Error>> for GreeksError {
    fn from(error: Box<dyn Error>) -> Self {
        GreeksError::StdError(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_invalid_volatility_error_creation() {
        let error = GreeksError::invalid_volatility(-0.5, "Volatility cannot be negative");
        match error {
            GreeksError::InputError(InputErrorKind::InvalidVolatility { value, reason }) => {
                assert_eq!(value, -0.5);
                assert_eq!(reason, "Volatility cannot be negative");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]

    fn test_delta_error_creation() {
        let error = GreeksError::delta_error("Failed to calculate delta");
        match error {
            GreeksError::CalculationError(CalculationErrorKind::DeltaError { reason }) => {
                assert_eq!(reason, "Failed to calculate delta");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]

    fn test_math_error_display() {
        let error = GreeksError::MathError(MathErrorKind::DivisionByZero);
        assert_eq!(error.to_string(), "Mathematical error: Division by zero");

        let error = GreeksError::MathError(MathErrorKind::InvalidDomain {
            value: 1.5,
            reason: "Value out of range".to_string(),
        });
        assert_eq!(
            error.to_string(),
            "Mathematical error: Invalid domain value 1.5: Value out of range"
        );
    }

    #[test]

    fn test_input_error_display() {
        let error = GreeksError::InputError(InputErrorKind::InvalidPrice {
            value: -100.0,
            reason: "Price cannot be negative".to_string(),
        });
        assert_eq!(
            error.to_string(),
            "Input validation error: Invalid price -100: Price cannot be negative"
        );

        let error = GreeksError::InputError(InputErrorKind::InvalidRate {
            value: 2.5,
            reason: "Rate must be between 0 and 1".to_string(),
        });
        assert_eq!(
            error.to_string(),
            "Input validation error: Invalid rate 2.5: Rate must be between 0 and 1"
        );
    }

    #[test]

    fn test_calculation_error_display() {
        let error = GreeksError::CalculationError(CalculationErrorKind::GammaError {
            reason: "Invalid input parameters".to_string(),
        });
        assert_eq!(
            error.to_string(),
            "Greek calculation error: Gamma calculation error: Invalid input parameters"
        );

        let error = GreeksError::CalculationError(CalculationErrorKind::VegaError {
            reason: "Calculation overflow".to_string(),
        });
        assert_eq!(
            error.to_string(),
            "Greek calculation error: Vega calculation error: Calculation overflow"
        );
    }

    #[test]

    fn test_convergence_failure_display() {
        let error = GreeksError::MathError(MathErrorKind::ConvergenceFailure {
            iterations: 1000,
            tolerance: 0.0001,
        });
        assert_eq!(
            error.to_string(),
            "Mathematical error: Failed to converge after 1000 iterations (tolerance: 0.0001)"
        );
    }

    #[test]

    fn test_result_type() {
        fn test_function() -> GreeksResult<f64> {
            Err(GreeksError::delta_error("Test error"))
        }

        let result = test_function();
        assert!(result.is_err());
        match result {
            Err(GreeksError::CalculationError(CalculationErrorKind::DeltaError { reason })) => {
                assert_eq!(reason, "Test error");
            }
            _ => panic!("Wrong error type"),
        }
    }

    #[test]

    fn test_error_trait_implementation() {
        let error = GreeksError::delta_error("Test error");
        let _error_trait_object: &dyn Error = &error;
        // If this compiles, it means Error trait is implemented correctly
    }

    #[test]

    fn test_debug_implementation() {
        let error = GreeksError::delta_error("Test error");
        let debug_string = format!("{:?}", error);
        assert!(debug_string.contains("DeltaError"));
        assert!(debug_string.contains("Test error"));
    }
}

#[cfg(test)]
mod tests_extended {
    use super::*;
    use crate::error::decimal::DecimalError::InvalidPrecision;
    use crate::error::greeks::CalculationErrorKind::DecimalError;
    use crate::pos;

    #[test]
    fn test_greeks_error_std_error() {
        let error = GreeksError::StdError("An error occurred".to_string());
        assert_eq!(format!("{}", error), "Standard error: An error occurred");
    }

    #[test]
    fn test_math_error_overflow() {
        let error = MathErrorKind::Overflow;
        assert_eq!(format!("{}", error), "Numerical overflow");
    }

    #[test]
    fn test_input_error_invalid_volatility() {
        let error = InputErrorKind::InvalidVolatility {
            value: 0.5,
            reason: "Out of bounds".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Invalid volatility 0.5: Out of bounds"
        );
    }

    #[test]
    fn test_calculation_error_delta() {
        let error = CalculationErrorKind::DeltaError {
            reason: "Unable to compute delta".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Delta calculation error: Unable to compute delta"
        );
    }

    #[test]
    fn test_calculation_error_theta() {
        let error = CalculationErrorKind::ThetaError {
            reason: "Negative time decay".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Theta calculation error: Negative time decay"
        );
    }

    #[test]
    fn test_calculation_error_rho() {
        let error = CalculationErrorKind::RhoError {
            reason: "Interest rate too high".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Rho calculation error: Interest rate too high"
        );
    }

    #[test]
    fn test_calculation_error_decimal() {
        let error = DecimalError {
            error: InvalidPrecision {
                precision: 0,
                reason: "Precision error".to_string(),
            },
        };
        assert_eq!(
            format!("{}", error),
            "Decimal error: Invalid decimal precision 0: Precision error"
        );
    }

    #[test]
    fn test_invalid_time_constructor() {
        let error = GreeksError::invalid_time(pos!(5.0), "Time must be positive");
        assert_eq!(
            format!("{}", error),
            "Input validation error: Invalid time value 5: Time must be positive"
        );
    }

    #[test]
    fn test_decimal_error_conversion() {
        let decimal_error = InvalidPrecision {
            precision: 0,
            reason: "Precision lost".to_string(),
        };

        let error: GreeksError = decimal_error.into();

        match error {
            GreeksError::CalculationError(DecimalError { error }) => {
                assert!(error.to_string().contains("Precision lost"));
            }
            _ => panic!("Wrong error variant"),
        }
    }

    #[test]
    fn test_implied_volatility_error_conversion() {
        let iv_error = VolatilityError::ZeroVega;
        let error: GreeksError = iv_error.into();
        assert_eq!(
            format!("{}", error),
            "Input validation error: Invalid volatility 0: Vega is zero, cannot calculate implied volatility"
        );
    }

    #[test]
    fn test_boxed_error_conversion() {
        let boxed_error: Box<dyn Error> = Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Some IO error",
        ));
        let error: GreeksError = boxed_error.into();
        assert_eq!(format!("{}", error), "Standard error: Some IO error");
    }
}
