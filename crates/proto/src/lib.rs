#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![cfg(not(doctest))]

#[cfg(feature = "pulumi")]
pub mod pulumi;
#[cfg(feature = "pulumi_gestalt")]
pub mod pulumi_gestalt;
