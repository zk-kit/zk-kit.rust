use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ImtError {
    #[error("Too many leaves: got {got}, max for arity {arity} and depth {depth} is max {max}")]
    TooManyLeaves {
        got: usize,
        arity: usize,
        depth: usize,
        max: usize,
    },
    #[error("The tree is full")]
    TreeFull,
    #[error("The leaf does not exist in this tree")]
    NotContained,
}
