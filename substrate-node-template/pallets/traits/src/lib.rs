#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::{DispatchError, DispatchResult};

use pallet_utils::{SpaceId, User};

pub mod moderation;


pub trait SpaceFollowsProvider {
  type AccountId;

  fn is_space_follower(account: Self::AccountId, space_id: SpaceId) -> bool;
}