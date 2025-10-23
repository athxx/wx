use crate::domain::{Contact, Message};

pub trait ContactsRepo: Send + Sync {
    fn get_all(&self) -> Vec<Contact>;
}

pub trait SessionsRepo: Send + Sync {
    fn get_messages(&self, contact: &Contact) -> Vec<Message>;
}
