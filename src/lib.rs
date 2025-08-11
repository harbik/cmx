//#![allow(dead_code, unused_imports)]
#![doc = include_str!("../README.md")]

/*
  Copyright 2021, Harbers Bik LLC

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*/

pub mod profile;
pub mod header;
pub mod signatures;
pub mod tag;
pub mod error;
pub mod language;

pub use error::Error;
use num::Zero;


/// Rounds a floating-point value to the specified precision.
/// For example, round_to_precision(1.23456, 100.0) returns 1.23.
///
/// # Arguments
/// * `value` - The value to round.
/// * `precision` - The precision
///
/// # Returns
/// The rounded value.
pub(crate) fn round_to_precision(value: f64, precision: i32) -> f64 {
    let multiplier = 10f64.powi(precision);
    (value * multiplier).round() / multiplier
}

// Add this helper function
// Make the helper function generic
pub(crate) fn is_zero<T: Zero>(n: &T) -> bool {
    n.is_zero()
}