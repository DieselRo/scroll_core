// artifact.rs – Abstract Writable Artifact Interface
//======================================================

pub trait WritableArtifact {
    fn to_string_representation(&self) -> String;
    fn file_extension(&self) -> &'static str;
}
