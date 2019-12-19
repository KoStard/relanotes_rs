#[derive(Debug)]
pub enum RelanotesValidationRejection {
    TechnicalError(String),
    TryingToMutateOtherSubgroup { current: i32, checking: i32 },
    EmptyName,
    LinkedToItself(i32),
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
        match self {
            RelanotesValidationRejection::TechnicalError(e) => write!(f, "Technical Error {}", e),
            RelanotesValidationRejection::TryingToMutateOtherSubgroup { current: _, checking } => write!(f, "Trying to mutate other subgroup ({})", checking),
            RelanotesValidationRejection::EmptyName => write!(f, "The name is empty"),
            RelanotesValidationRejection::LinkedToItself(id) => write!(f, "Linked to itself ({})", id),
            RelanotesValidationRejection::DuplicateRegularNode(e) => write!(f, "Duplicate regular node ({})", e),
            RelanotesValidationRejection::StickyNoteWithoutOwner => write!(f, "Sticky note without an owner"),
            RelanotesValidationRejection::InvalidStickyNoteOwner => write!(f, "Invalid sticky note owner"),
            RelanotesValidationRejection::DuplicateStickyNote(e) => write!(f, "Duplicate sticky note ({})", e),
            RelanotesValidationRejection::InheritedNodeWithoutOwner => write!(f, "Inherited node without an owner"),
            RelanotesValidationRejection::InvalidInheritedNodeOwner => write!(f, "Invalid inherited node owner"),
            RelanotesValidationRejection::DuplicateInheritedNode(e) => write!(f, "Duplicate inherited node ({})", e),
            RelanotesValidationRejection::SymLinkWithName => write!(f, "SymLink with a name"),
            RelanotesValidationRejection::SymLinkWithDescription => write!(f, "SymLink with a description"),
            RelanotesValidationRejection::SymLinkWithoutOwner => write!(f, "SymLink without an owner"),
            RelanotesValidationRejection::InvalidSymLinkOwner => write!(f, "Invalid SymLink owner"),
            RelanotesValidationRejection::SymLinkToSameSubgroup => write!(f, "SymLink targetting to the same group"),
        }
    }
}

impl std::error::Error for RelanotesValidationRejection {
}
