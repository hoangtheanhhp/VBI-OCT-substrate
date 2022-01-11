#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_runtime::RuntimeDebug;
	use codec::{Decode, Encode};
	#[cfg(feature = "std")]
	use serde::{Serialize, Deserialize};
	use frame_system::{ensure_signed};
	use scale_info::TypeInfo;


	use pallet_utils::{Content, PostId
	};
	/// Information about a post's owner, its' related space, content, and visibility.
	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(bounds(), skip_type_params(T))]
	pub struct Post {

		/// Unique sequential identifier of a post. Examples of post ids: `1`, `2`, `3`, and so on.
		pub id: PostId,

		pub content: Content,
	}
	/// Post extension provides specific information necessary for different kinds 
	/// of posts such as regular posts, comments, and shared posts.
	#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "std", serde(untagged))]
	pub enum PostExtension {
		RegularPost,
		NFT
	}

	impl Default for PostExtension {
		fn default() -> Self {
			PostExtension::RegularPost
		}
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn next_post_id)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type NextPostId<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn post_by_id)]
	pub(super) type PostById<T: Config> = StorageMap<_, Blake2_128Concat, PostId, Option<Post>, ValueQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		PostCreated(T::AccountId, PostId),
        PostDeleted(T::AccountId, PostId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {

		 /// Post was not found by id.
		 PostNotFound,
		 /// An account is not a post owner.
		 NotAPostOwner,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(0)]
		pub fn create_post(
		  origin: OriginFor<T>,
		  extension: PostExtension,
		  content: Content
		) -> DispatchResult {
		  let creator = ensure_signed(origin)?;
	
		//   let new_post_id = Self::next_post_id();
		//   let new_post: Post<T> = Post::new(new_post_id, creator.clone(), extension, content.clone());
	
		//   PostById::insert(new_post_id, new_post);
		//   NextPostId::mutate(|n| { *n += 1; });
	
		  Self::deposit_event(Event::PostCreated(creator, 1));
		  Ok(())
		}
	}
}
