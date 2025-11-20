use super::sample_data;
use crate::models::{Contact, Message};

pub struct MemoryContactsRepo;

impl MemoryContactsRepo {
    pub fn new() -> Self {
        Self
    }
}

impl MemoryContactsRepo {
    pub fn get_all(&self) -> Vec<Contact> {
        sample_data::create_sample_contacts()
    }
}

pub struct MemorySessionsRepo;

impl MemorySessionsRepo {
    pub fn new() -> Self {
        Self
    }

    pub fn get_messages(&self, contact: &Contact) -> Vec<Message> {
        sample_data::create_sample_messages(contact)
    }
}
