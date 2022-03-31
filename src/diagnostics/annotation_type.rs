/// Different types of annotations, usually only distinguishable by Color
pub enum AnnotationType {
    /// Error messages that should only be generated by unrecoverable errors
    ///
    /// ```text
    /// Something has gone terribly wrong!
    /// ```
    Error,

    /// Messages that indicate flaws in the code, but are not critical
    ///
    /// ```text
    /// This statement does nothing!
    /// ```
    Warning,

    /// Information about parts of the code related to errors/warnings
    ///
    /// ```text
    /// ^-- the error occured because this was false
    /// ```
    Info,

    /// Additional information about why the error occured
    ///
    /// ```text
    /// =note: Leuchtkraft does not support syntax errors!
    /// ```
    Note,

    /// Tips on how to fix any of the above
    ///
    /// ```text
    /// ^-- try removing this implication
    /// ```
    Help,
}