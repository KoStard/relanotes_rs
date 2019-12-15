#[derive(Debug)]
pub enum RelanotesValidationRejection {
    TechnicalError(String),
    TryingToMutateOtherSubgroup { current: i32, checking: i32 },
    EmptyName,
    DuplicateRegularNode(String),
    StickyNoteWithoutOwner,
    InvalidStickyNoteOwner,
    DuplicateStickyNote(String),
    InheritedNodeWithoutOwner,
    InvalidInheritedNodeOwner,
    DuplicateInheritedNode(String),
    SymLinkWithName,
    SymLinkWithDescription,
    SymLinkWithoutOwner,
    InvalidSymLinkOwner,
    SymLinkToSameSubgroup,
}

impl std::fmt::Display for RelanotesValidationRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {}
    }
}

impl std::error::Error for RelanotesValidationRejection {
    fn description(&self) -> &str {
        match self {}
    }
}
