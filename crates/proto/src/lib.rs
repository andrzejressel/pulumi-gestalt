#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![cfg(not(doctest))]

#[cfg(feature = "connectivity")]
pub mod full;
#[cfg(feature = "pulumi_gestalt")]
pub mod pulumi_gestalt;