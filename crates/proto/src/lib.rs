#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![cfg(not(doctest))]

#[cfg(feature = "pulumi")]
pub mod pulumi;

#[cfg(feature = "language_server")]
pub mod language_server;
