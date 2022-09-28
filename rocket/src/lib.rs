mod error;
mod fairing;

// FIXME: Do this differently!
// mod config {
//     use std::env;
//
//     use rocket::config::Environment;
//
//     lazy_static! {
//         pub static ref ROCKET_ENV: Environment = {
//             match env::var("ROCKET_ENV").ok() {
//                 Some(value) => value.parse().unwrap_or(Environment::Development),
//                 None => Environment::Development,
//             }
//         };
//     }
// }

pub mod request;
pub mod response;

pub use self::{fairing::JsonApiFairing, request::*, response::*};
