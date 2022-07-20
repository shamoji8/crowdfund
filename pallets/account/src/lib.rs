#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::inherent::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	//use pallet_fund_raising::{Role, Status};
	use scale_info::TypeInfo;

	use serde::{Deserialize, Serialize};

	// https://docs.substrate.io/reference/how-to-guides/basics/configure-genesis-state/
	#[derive(Encode, Decode, Ord, PartialOrd, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Role {
		Organization,
		SysMan,
		User,
		Contributer,
	}

	impl Default for Role {
		fn default() -> Self {
			Self::User
		}
	}

	#[derive(Encode, Decode, Ord, PartialOrd, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Status {
		Active,
		Revoked,
		Deactivated,
	}

	impl Default for Status {
		fn default() -> Self {
			Self::Active
		}
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(bounds(), skip_type_params(T))]
	pub struct Account<T: Config> {
		id: T::AccountId,
		role: Role,
		status: Status,
		metadata: Vec<u8>,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn account_storage)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type AccountStorage<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, Account<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account_role)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type AccountRole<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Role, OptionQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub sysman_accountmap: Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { sysman_accountmap: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for a in &self.sysman_accountmap {
				let _a = a.clone();
				let account = Account::<T> {
					id: _a,
					role: Role::SysMan,
					status: Status::Active,
					metadata: Vec::new(),
				};
				<AccountStorage<T>>::insert(a, account);
				<AccountRole<T>>::insert(a, Role::SysMan);
			}
		}
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AccountRegisted,
		AccountUpdated(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Account is already Registered
		AlreadyRegistered,
		/// Account is not Registered
		AccountNotRegistered,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000)]
		pub fn register(origin: OriginFor<T>, role: Role, metadata: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			match <AccountStorage<T>>::try_get(&who) {
				Err(_) => {
					<AccountStorage<T>>::insert(
						&who,
						Account {
							id: who.clone(),
							role: role.clone(),
							status: Status::Active,
							metadata,
						},
					);
					<AccountRole<T>>::insert(who, role.clone());
				},
				Ok(_) => Err(Error::<T>::AlreadyRegistered)?,
			}
			// Return a successful DispatchResultWithPostInfo
			Self::deposit_event(Event::AccountRegisted);
			Ok(())
		}
	}
}
