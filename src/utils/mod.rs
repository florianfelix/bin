mod error;
mod wrapper;

pub mod prelude {
    pub use super::error::{Error, Result};
    pub use super::wrapper::W;
}
