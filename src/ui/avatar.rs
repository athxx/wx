use std::sync::atomic::{AtomicUsize, Ordering};

static AVATAR_SEQ: AtomicUsize = AtomicUsize::new(0);

const AVA_SVGS: &[&str] = &[
    "ava/afro.svg",
    "ava/angry-.svg",
    "ava/angry-_1.svg",
    "ava/angry-_2.svg",
    "ava/arrogant.svg",
    "ava/baby-.svg",
    "ava/baby.svg",
    "ava/bully.svg",
    "ava/businessman.svg",
    "ava/cheeky-.svg",
    "ava/confused-.svg",
    "ava/crying-.svg",
    "ava/dazed.svg",
    "ava/dead-.svg",
    "ava/dead-_1.svg",
    "ava/desperate-.svg",
    "ava/desperate.svg",
    "ava/dissapointment.svg",
    "ava/drunk.svg",
    "ava/evil.svg",
    "ava/gangster.svg",
    "ava/geek.svg",
    "ava/gentleman-.svg",
];

pub fn avatar_for_key(key: &str) -> &'static str {
    let h = gpui::hash(&key.to_string()) as usize;
    let ix = h % AVA_SVGS.len();
    AVA_SVGS[ix]
}

#[allow(dead_code)]
pub fn random_avatar_path() -> &'static str {
    let ix = AVATAR_SEQ.fetch_add(1, Ordering::Relaxed) % AVA_SVGS.len();
    AVA_SVGS[ix]
}
