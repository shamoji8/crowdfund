#![cfg_attr(not(feature = "std"), no_std)]

// use frame_support::inherent::Vec;
use scale_info::prelude::vec;
use frame_support::pallet_prelude::*;
use frame_support::traits::{Currency, ReservableCurrency};
use frame_system::pallet_prelude::*;
//use pallet_fund_raising::{Role, Status};
//use scale_info::TypeInfo;

//use serde::{Deserialize, Serialize};

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: ReservableCurrency<Self::AccountId>;

		type Fee: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

	#[pallet::storage]
	#[pallet::getter(fn score_storage)]
	pub type ScoreStorage<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, i32, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn amount)]
	// initail value : 0
	pub type Amount<T> = StorageValue<_, i32>;

	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub amount_num: i32,
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self { amount_num: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			<Amount<T>>::put(&self.amount_num);
		}
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ScoreSet(T::AccountId, i32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		AlreadySet,
		NotEnoughAmount,
		InvalidAccount,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn set_score(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			ensure!(T::Currency::total_balance(&who) >= T::Fee::get(), Error::<T>::NotEnoughAmount);

			match <ScoreStorage<T>>::try_get(&who) {
				Err(_) => {
					<ScoreStorage<T>>::insert(&who, 100);
				},
				Ok(_) => Err(Error::<T>::AlreadySet)?,
			}

			Self::deposit_event(Event::ScoreSet(who, 100));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn evaluation(origin: OriginFor<T>, account: AccountIdOf<T>, mut person_score: i32) -> DispatchResult {
			let mut score = Self::score_storage(account).ok_or(Error::<T>::InvalidAccount)?;

			person_score += 1;

			// 1: -10, 2: -3, 3: -1, 4: 1, 5: 3, 6: 10
			let psc = vec![-10, -3, -1, 1, 3, 10];


			let calc = (score + psc[person_score as usize]) + (psc[person_score as usize] * score) / 1000;

			Ok(())
		}
	}
}
