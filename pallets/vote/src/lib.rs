#![cfg_attr(not(feature = "std"), no_std)]

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
	use frame_support::pallet_prelude::{*, ValueQuery};
	use frame_system::pallet_prelude::*;

	use frame_support::traits::{Currency, ExistenceRequirement, WithdrawReasons};

	use sp_runtime::traits::Zero;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// type Currency: Currency<Self::AccountId>;

		// type PostDeposit: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Simple index for identifying a fund.
	pub type IdeaIndex = u32;

	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	// type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
	// type IdeaOf<T> = Idea<AccountIdOf<T>, BalanceOf<T>, <T as frame_system::Config>::BlockNumber>;

	// #[derive(Encode, Decode, Default, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	// #[cfg_attr(feature = "std", derive(Debug))]
	// pub struct Idea<AccountId, Balance, BlockNumber> {
	// 	/// 
	// 	inventor: AccountId,
	// 	/// The amount of deposit placed
	// 	pre_deposit: Balance,
	// 	/// The total amount raised
	// 	voted_number: u32,
	// 	/// Block number after which funding must have succeeded
	// 	end: BlockNumber,
	// 	/// Upper bound on `raised`
	// 	goal: Balance,
	// }

	// #[pallet::storage]
	// #[pallet::getter(fn ideas)]
	// pub type Ideas<T> = StorageMap<_, Blake2_128Concat, IdeaIndex, IdeaOf<T>, ValueQuery>;

	// #[pallet::storage]
	// #[pallet::getter(fn idea_count)]
	// pub type IdeaCount<T> = StorageValue<_, IdeaIndex, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		// #[pallet::weight(10_000)]
		// pub fn post(origin: OriginFor<T>, end: T::BlockNumber, goal: BalanceOf<T>) -> DispatchResult {
		// 	// Check that the extrinsic was signed and get the signer.
		// 	let drafter = ensure_signed(origin)?;

		// 	// get pre deposit
		// 	let pre_deposit = T::PostDeposit::get();

		// 	// withdraw pre deposit
		// 	let imb = T::Currency::withdraw(
		// 		&drafter,
		// 		pre_deposit,
		// 		WithdrawReasons::TRANSFER,
		// 		ExistenceRequirement::AllowDeath,
		// 	)?;

		// 	let index = <IdeaCount<T>>::get();

		// 	<IdeaCount<T>>::put(index.saturating_add(1));

		// 	// transfer fee to another pool
		// 	T::Currency::resolve_creating(&Self::fund_account_id(index), imb);

		// 	// register idea
		// 	<Ideas<T>>::insert(
		// 		index,
		// 		Idea { inventor: drafter, pre_deposit, voted_number: Zero::zero(), end, goal },
		// 	);

		// 	// Emit an event.
		// 	Self::deposit_event(Event::SomethingStored(something, who));
		// 	// Return a successful DispatchResultWithPostInfo
		// 	Ok(())
		// }

		// /// An example dispatchable that may throw a custom error.
		// #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		// pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	// Read a value from storage.
		// 	match <Something<T>>::get() {
		// 		// Return an error if the value has not been set.
		// 		None => return Err(Error::<T>::NoneValue.into()),
		// 		Some(old) => {
		// 			// Increment the value read from storage; will error in the event of overflow.
		// 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
		// 			// Update the value in storage with the incremented result.
		// 			<Something<T>>::put(new);
		// 			Ok(())
		// 		},
		// 	}
		// }
	}
}
