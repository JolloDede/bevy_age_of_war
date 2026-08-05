#[derive(Clone, Copy, PartialEq)]
pub enum GameUnit {
    Meele,
    Ranged,
    Tank,
    Super,
}

#[derive(Clone, Copy, PartialEq)]
pub enum QueueUnit {
    None,
    Meele,
    Ranged,
    Tank,
    Super,
}
