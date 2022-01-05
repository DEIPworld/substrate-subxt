// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
    SignedExtra,
    StorageEntry,
};
use codec::{
    Codec,
    Encode,
    EncodeLike,
};
use core::fmt::Debug;
use sp_runtime::traits::{
    AtLeast32Bit,
    Extrinsic,
    Hash,
    Header,
    MaybeSerializeDeserialize,
    Member,
    Verify,
};

/// Runtime types.
pub trait Config: Clone + Sized + Send + Sync + 'static {
    /// Account index (aka nonce) type. This stores the number of previous
    /// transactions associated with a sender account.
    type Index: Parameter + Member + Default + AtLeast32Bit + Copy + scale_info::TypeInfo;

    /// The block number type used by the runtime.
    type BlockNumber: Parameter
        + Member
        + Default
        + Copy
        + core::hash::Hash
        + core::str::FromStr;

    /// The output of the `Hashing` function.
    type Hash: Parameter
        + Member
        + MaybeSerializeDeserialize
        + Ord
        + Default
        + Copy
        + std::hash::Hash
        + AsRef<[u8]>
        + AsMut<[u8]>
        + scale_info::TypeInfo;

    /// The hashing system (algorithm) being used in the runtime (e.g. Blake2).
    type Hashing: Hash<Output = Self::Hash>;

    /// The user account identifier type for the runtime.
    type AccountId: Parameter + Member;

    /// The address type. This instead of `<frame_system::Trait::Lookup as StaticLookup>::Source`.
    type Address: Codec + Clone + PartialEq;

    /// The block header.
    type Header: Parameter
        + Header<Number = Self::BlockNumber, Hash = Self::Hash>
        + serde::de::DeserializeOwned;

    /// Signature type.
    type Signature: Verify + Encode + Send + Sync + 'static;

    /// Extrinsic type within blocks.
    type Extrinsic: Parameter + Extrinsic + Debug + MaybeSerializeDeserialize;
}

/// Parameter trait copied from `substrate::frame_support`
pub trait Parameter: Codec + EncodeLike + Clone + Eq + Debug {}
impl<T> Parameter for T where T: Codec + EncodeLike + Clone + Eq + Debug {}

/// Trait to fetch data about an account.
///
/// Should be implemented on a type implementing `StorageEntry`,
/// usually generated by the `subxt` macro.
pub trait AccountData<T: Config>: StorageEntry {
    /// Create a new storage entry key from the account id.
    fn storage_entry(account_id: T::AccountId) -> Self;
    /// Get the nonce from the storage entry value.
    fn nonce(result: &<Self as StorageEntry>::Value) -> T::Index;
}

/// Trait to configure the extra data for an extrinsic.
pub trait ExtrinsicExtraData<T: Config> {
    /// The type of the [`StorageEntry`] which can be used to retrieve an account nonce.
    type AccountData: AccountData<T>;
    /// The type of extra data and additional signed data to be included in a transaction.
    type Extra: SignedExtra<T> + Send + Sync + 'static;
}
