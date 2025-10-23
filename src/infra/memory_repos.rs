use crate::domain::repos::{ContactsRepo, SessionsRepo};
use crate::domain::{Contact, Message};
use crate::data::sample_data;

pub struct MemoryContactsRepo;

impl MemoryContactsRepo {
    pub fn new() -> Self {
        Self
    }
}

impl ContactsRepo for MemoryContactsRepo {
    fn get_all(&self) -> Vec<Contact> {
        sample_data::create_sample_contacts()
    }
}

pub struct MemorySessionsRepo;

impl MemorySessionsRepo {
    pub fn new() -> Self {
        Self
    }
}

impl SessionsRepo for MemorySessionsRepo {
    fn get_messages(&self, contact: &Contact) -> Vec<Message> {
        sample_data::create_sample_messages(contact)
    }
}
