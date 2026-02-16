use crate::modules::{aliases, casting, literals};

pub fn types() {
    casting::casting();
    literals::literals();
    aliases::alias();
}