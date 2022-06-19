// RGB Standard Library: high-level API to RGB smart contracts.
// Written in 2019-2022 by
//     Dr. Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// To the extent possible under law, the author(s) have dedicated all copyright
// and related and neighboring rights to this software to the public domain
// worldwide. This software is distributed without any warranty.
//
// You should have received a copy of the MIT License along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

mod transfer;
mod id;

pub use id::ConsignmentId;
pub use transfer::{
    AnchoredBundles, ExtensionList, StateTransfer, TransferEndpoints, RGB_TRANSFER_VERSION,
};
