#![cfg_attr(not(feature = "std"), no_std)]

pub mod aabb2;
pub mod iaabb2;

pub use aabb2::{aabb2, Aabb2};
pub use iaabb2::{iaabb2, IAabb2};
