//! commands hold the formal message that goes through the system

use whirlybird::journey::entity::entry::Entry;

//
// # type declaration
//

#[derive(Debug, Clone)]
pub enum Change {
    CreateNewEntry(Entry),
}
