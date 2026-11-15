use std::fmt;

// Purpose: To get a better format when printing numbers
pub struct NewVec<T>(pub Vec<T>);

impl<T: fmt::Display> fmt::Display for NewVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{s}")
    }
}
