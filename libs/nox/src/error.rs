use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("xla error {0}")]
    Xla(#[from] xla::Error),
    #[error("vmap axis len must be same as input len")]
    WrongAxisLen,
    #[error("vmap arguments must be batchable")]
    UnbatchableArgument,
    #[error("vmap requires at least one argument")]
    VmapArgsEmpty,
    #[error("vmap requires in axis length to equal arguments length")]
    VmapInAxisMismatch,
}
