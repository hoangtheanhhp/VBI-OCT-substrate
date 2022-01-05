#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::*, pallet_prelude::*, traits::Currency};
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_runtime::SaturatedConversion;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub enum Zodiac {
		Aquarius,
		Pisces,
		Aries,
		Taurus,
		Gemini,
		Cancer,
		Leo,
		Virgo,
		Libra,
		Scorpio,
		Sagittarius,
		Capricorn,
	}

	impl Default for Zodiac {
		fn default() -> Self { Zodiac::Aquarius }
	}


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MyCurrency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn account_zodiac)]
	//Account's zodiac storage
	pub type AccountZodiac<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Zodiac, ValueQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SetZodiacSucceed(T::AccountId),
		TotalIssuance(u32),
		FreeBalance(u32)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ZodiacWasSet,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000)]
		pub fn set_zodiac(origin: OriginFor<T>, zodiac: Zodiac) -> DispatchResult {
			let who = ensure_signed(origin)?;
			if AccountZodiac::<T>::contains_key(&who) {
				Err(Error::<T>::ZodiacWasSet)?
			} else {
				AccountZodiac::<T>::insert(&who, zodiac);
				Self::deposit_event(Event::<T>::SetZodiacSucceed(who));
			}
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		#[pallet::weight(10_000)]
		pub fn get_balance_info(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let total_issuance = T::MyCurrency::total_issuance();
			let value = total_issuance.saturated_into::<u32>();
			let free_balance = T::MyCurrency::free_balance(&who);
			let my_balance = free_balance.saturated_into::<u32>();
			Self::deposit_event(Event::<T>::TotalIssuance(value));
			Self::deposit_event(Event::<T>::FreeBalance(my_balance));
			Ok(())
		}
	}
}
