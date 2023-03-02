//! different views mods for each redmaple tree

use std::fmt::Display;

/// `ViewMode` trait offers a generic way of handling different view of the data
pub trait ViewMode {
    /// this type identifier allows for the item to be used in a way that can be used to serialize
    /// stuff later on. `Display` trait here is used to ensure that whatever type you use. it would
    /// be able to be displayed in a string format.
    ///
    /// So if you want to return an enum variant, you should also implement `Display` for that enum
    /// as well.
    type Identifier: Sized + Display;

    /// Returns a sized item that can be used in order to identify the view type.
    ///
    /// I suggest the use of an enum. The enum can be the same as the parent enum (if you are
    /// using an enum as the parent parent ).
    fn get(&self) -> &Self::Identifier;
}
