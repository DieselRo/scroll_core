// ===============================
// src/state_manager.rs
// ===============================
#![allow(dead_code)]
#![allow(unused_imports)]

use chrono::Utc;
use log::info;

use crate::scroll::Scroll;
use crate::schema::ScrollStatus;

pub fn transition(scroll: &mut Scroll, new_status: ScrollStatus) {
    let old_status = scroll.status.clone();
    scroll.status = new_status;
    scroll.origin.last_modified = Utc::now();

    info!(
        "State transition for '{}': {:?} -> {:?} at {:?} — {}",
        scroll.title,
        old_status,
        scroll.status,
        scroll.origin.last_modified,
        describe_status(scroll.status.clone())
    );
}

pub fn describe_status(status: ScrollStatus) -> &'static str {
    match status {
        ScrollStatus::Emergent => "Becoming—its essence begins to cohere.",
        ScrollStatus::Draft => "Dormant—yet full of unwritten possibility.",
        ScrollStatus::Active => "Stirring—its voice prepares to echo.",
        ScrollStatus::Sealed => "Closed—its truth hidden, but preserved.",
        ScrollStatus::Archived => "Laid to rest in the Vault of Threads.",
        ScrollStatus::Latent => "Suspended—awaiting catalyst or consequence.",
    }
}

pub fn is_valid_transition(current: &ScrollStatus, next: &ScrollStatus) -> bool {
    use ScrollStatus::*;
    match (current, next) {
        (Draft, Active) => true,
        (Active, Sealed) => true,
        (Sealed, Archived) => true,
        (Emergent, Draft) => true,
        (Latent, Emergent) => true,
        (_, _) if current == next => true,
        _ => false,
    }
}

pub fn try_transition(scroll: &mut Scroll, next_status: ScrollStatus) -> Result<(), String> {
    if !is_valid_transition(&scroll.status, &next_status) {
        return Err(format!(
            "Invalid state transition: {:?} -> {:?}",
            scroll.status, next_status
        ));
    }

    if matches!((&scroll.status, &next_status), (ScrollStatus::Draft, ScrollStatus::Active)) {
        scroll.validate()?;
    }

    transition(scroll, next_status);
    Ok(())
}
