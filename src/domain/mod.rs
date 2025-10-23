pub mod repos;
pub mod events;

// Re-export existing domain models so callers can migrate to this namespace gradually
pub use crate::models::{ChatSession, Contact, Message, ToolbarItem};
