#![cfg_attr(not(feature = "std"), no_std)]

/// A module for proof of existence
pub use pallet::*;
use codec::{Encode, Decode};


#[cfg(test)]
mod tests;

#[cfg(feature = "std")]
use sp_std::{
    prelude::*,
};
use sp_std::vec::Vec;
use scale_info::TypeInfo;


pub type PostId = u64;


#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo, Debug)]
pub enum Content {
    /// No content.
    None,
    /// A raw vector of bytes.
    Raw(Vec<u8>),
    /// IPFS CID v0 of content.
    #[allow(clippy::upper_case_acronyms)]
    IPFS(Vec<u8>),
    /// Hypercore protocol (former DAT) id of content.
    Hyper(Vec<u8>),
}

impl From<Content> for Vec<u8> {
    fn from(content: Content) -> Vec<u8> {
        match content {
            Content::None => Vec::new(),
            Content::Raw(vec_u8) => vec_u8,
            Content::IPFS(vec_u8) => vec_u8,
            Content::Hyper(vec_u8) => vec_u8,
        }
    }
}

impl Default for Content {
    fn default() -> Self {
        Self::None
    }
}

impl Content {
    pub fn is_none(&self) -> bool {
        self == &Self::None
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn is_ipfs(&self) -> bool {
        matches!(self, Self::IPFS(_))
    }
}

/// Information about a post's owner, its' related space, content, and visibility.
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo, Debug)]
#[scale_info(bounds(), skip_type_params(T))]
pub struct Post {

    /// Unique sequential identifier of a post. Examples of post ids: `1`, `2`, `3`, and so on.
    pub id: PostId,

    pub content: Content,
}

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*
    };
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type ClassData: Parameter + Member;
        
        #[pallet::constant]
        type MaximumClaimLength: Get<u32>;
        type MinimumClaimLength: Get<u32>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::ClassData,
        (T::AccountId, T::BlockNumber)
    >;

    #[pallet::event]
    //#[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, T::ClassData),
        ClaimRevoked(T::AccountId, T::ClassData),
        ClaimTransferred(T::AccountId, T::AccountId, T::ClassData),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyExist,
        ClaimNotExist,
        NotClaimOwner,
        DestinationIsClaimOwner,
        ClaimTooBig,
        ClaimTooSmall,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: T::ClassData
        ) -> DispatchResultWithPostInfo {
            // ensure!(claim.len() <= T::MaximumClaimLength::get() as usize, Error::<T>::ClaimTooBig);
            // ensure!(claim.len() >= T::MinimumClaimLength::get() as usize, Error::<T>::ClaimTooSmall);
            let sender = ensure_signed(origin)?;
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
            Proofs::<T>::insert(
                &claim,
                (sender.clone(), <frame_system::Pallet::<T>>::block_number()),
            );

            Self::deposit_event(Event::ClaimCreated(sender, claim));
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: T::ClassData
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner == sender, Error::<T>::NotClaimOwner);
            Proofs::<T>::remove(&claim);
            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            destination: T::AccountId,
            claim: T::ClassData
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
            ensure!(owner == sender, Error::<T>::NotClaimOwner);
            ensure!(owner != destination, Error::<T>::DestinationIsClaimOwner);
            Proofs::<T>::remove(&claim);
            Proofs::<T>::insert(
                &claim,
                (destination.clone(), <frame_system::Pallet::<T>>::block_number()),
            );
            Self::deposit_event(Event::ClaimTransferred(sender, destination, claim));
            Ok(().into())
        }
    }

}
