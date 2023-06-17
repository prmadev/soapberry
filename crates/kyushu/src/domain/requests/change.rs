//! commands hold the formal message that goes through the system

use whirlybird::journey::entity::maple::Maple;

//
// # type declaration
//

#[derive(Debug, Clone)]
pub enum Change {
    CreateNewMaple(Maple),
}
