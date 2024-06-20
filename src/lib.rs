mod config;
pub mod cli;
mod req;

pub use config::{DiffConfig, DiffProfile, ResponseProfile};
pub use req::RequestProfile;


#[derive(Debug, Clone)]
pub struct ExtraArgs {
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,
}