pub mod suggest_conjugation_type;
pub use suggest_conjugation_type::{ConjugationType, VerbError, suggest_conjugation_type};

pub mod conjugate;
pub use conjugate::get_irrealis_form;
