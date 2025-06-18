// artifact.rs – Abstract Writable Artifact Interface
//======================================================
#![allow(dead_code)]
#![allow(unused_imports)]


pub trait WritableArtifact {
    fn to_string_representation(&self) -> String;
    fn file_extension(&self) -> &'static str;
}
