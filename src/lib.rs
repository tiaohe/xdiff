mod config;
pub mod cli;
mod req;
mod utils;

pub use config::{DiffConfig, DiffProfile, ResponseProfile};
pub use req::RequestProfile;
pub use utils::diff_text;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExtraArgs {
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}