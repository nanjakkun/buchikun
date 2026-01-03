pub mod infer_conjugation_type;
pub use infer_conjugation_type::{ConjugationType, VerbError, infer_conjugation_type};

pub mod conjugate;
pub use conjugate::get_irrealis_form;
