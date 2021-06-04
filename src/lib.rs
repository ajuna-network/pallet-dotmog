// DOT Mog, Susbstrate Gamification Project with C# .NET Standard & Unity3D
// Copyright (C) 2020-2021 DOT Mog Team, darkfriend77 & metastar77
//
// DOT Mog is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License.
// DOT Mog is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	ensure,
	traits::{
		Get, Randomness, Currency, ReservableCurrency, ExistenceRequirement, WithdrawReasons, OnUnbalanced
	},
	dispatch::DispatchError,
};
use frame_system::{ensure_signed};
use sp_runtime::{SaturatedConversion, traits::{Hash, TrailingZeroInput, Zero, AccountIdConversion}};
use sp_std::vec::{Vec};

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// Implementations of some helper traits passed into runtime modules as associated types.
pub mod general;
use general::{Pricing, Breeding, BreedType, Generation, RarityType, FeeType};

pub mod game_event;
use game_event::{GameEventType};

pub mod game_config;
use game_config::{GameConfig};

const MAX_AUCTIONS_PER_BLOCK: usize = 2;
const MAX_EVENTS_PER_BLOCK: usize = 10;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct MogwaiStruct<Hash, BlockNumber, Balance, RarityType> {
	id: Hash,
	dna: Hash,
	genesis: BlockNumber,
	price: Balance,
	gen: u32,
	rarity: RarityType,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct MogwaiBios<Hash, BlockNumber, Balance> {
	mogwai_id: Hash,
	state: u32,
	metaxy: Vec<[u8;16]>,
	intrinsic: Balance,
	level: u8,
	phases: Vec<BlockNumber>,
	adaptations: Vec<Hash>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct GameEvent<Hash, BlockNumber, GameEventType> {
	id: Hash,
	begin: BlockNumber,
	duration: u16,
	event_type: GameEventType,
	hashes: Vec<Hash>,
	value: u64,
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Auction<Hash, Balance, BlockNumber, AccountId> {
	mogwai_id: Hash,
	mogwai_owner: AccountId,
	expiry: BlockNumber,
	min_bid: Balance,
	high_bid: Balance,
	high_bidder: AccountId,
}

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
	};
	use frame_system::pallet_prelude::*;
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// The units in which we record balances.
		type Currency: ReservableCurrency<Self::AccountId>;
		/// Something that provides randomness in the runtime.
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		/// Handler for price payments.
		type PricePayment: OnUnbalanced<NegativeImbalanceOf<Self>>;
		// Weight information for extrinsics in this pallet.
		//type WeightInfo: WeightInfo;
	}
	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	/// The `AccountId` of the dot mog founder.
	#[pallet::storage]
	#[pallet::getter(fn key)]
	pub(super) type Key<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

	/// A map of the current configuration of an account.
	#[pallet::storage]
	#[pallet::getter(fn account_config)]
	pub type AccountConfig<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Option<Vec<u8>>, ValueQuery>;

	/// A map of mogwais accessible by the mogwai hash.
	#[pallet::storage]
	#[pallet::getter(fn mogwai)]
	pub type Mogwais<T: Config> = StorageMap<_, Identity, T::Hash, MogwaiStruct<T::Hash, T::BlockNumber, BalanceOf<T>, RarityType>, ValueQuery>;
	/// A map of mogwai bios accessible by the mogwai hash.
	#[pallet::storage]
	#[pallet::getter(fn mogwai_bios)]
	pub type MogwaisBios<T: Config> = StorageMap<_, Identity, T::Hash,MogwaiBios<T::Hash, T::BlockNumber, BalanceOf<T>>, ValueQuery>;
	/// A map of mogwai owners accessible by the mogwai hash.
	#[pallet::storage]
	#[pallet::getter(fn owner_of)]
	pub type MogwaiOwner<T: Config> = StorageMap<_, Identity, T::Hash,Option<T::AccountId>, ValueQuery>;

	/// A map of all existing mogwais accessible by the index. 
	#[pallet::storage]
	#[pallet::getter(fn mogwai_by_index)]
	pub type AllMogwaisArray<T: Config> = StorageMap<_, Blake2_128Concat, u64, T::Hash, ValueQuery>;
	/// A count over all existing mogwais in the system.
	#[pallet::storage]
	#[pallet::getter(fn all_mogwais_count)]
	pub type AllMogwaisCount<T: Config> = StorageValue<_, u64>;
	/// A map of the index of the mogwai accessible by the mogwai hash.
	#[pallet::storage]
	pub type AllMogwaisIndex<T: Config> = StorageMap<_, Identity, T::Hash, u64, ValueQuery>;
		
	/// A map of all mogwai hashes associated with an account.
	#[pallet::storage]
	#[pallet::getter(fn mogwai_of_owner_by_index)]
	pub type OwnedMogwaisArray<T: Config> = StorageMap<_, Blake2_128Concat, (T::AccountId, u64), T::Hash, ValueQuery>;
	/// A count over all existing mogwais owned by one account.
	#[pallet::storage]
	#[pallet::getter(fn owned_mogwais_count)]
	pub type OwnedMogwaisCount<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;
	/// A map of the owned mogwais index accessible by the mogwai hash.
	#[pallet::storage]
	pub type OwnedMogwaisIndex<T: Config> = StorageMap<_, Identity, T::Hash, u64, ValueQuery>;

	/// A map of mogwai auctions accessible by the mogwai hash.
	#[pallet::storage]
	#[pallet::getter(fn auction_of)]
	pub type MogwaiAuction<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Option<Auction<T::Hash, BalanceOf<T>, T::BlockNumber, T::AccountId>>, ValueQuery>;
	/// A vec of mogwai auctions accessible by the expiry block number.
	#[pallet::storage]
	#[pallet::getter(fn auctions_expire_at)]
	pub type Auctions<T: Config> = StorageMap<_, Blake2_128Concat, T::BlockNumber, Vec<Auction<T::Hash, BalanceOf<T>, T::BlockNumber, T::AccountId>>, ValueQuery>;
	/// Current auction period max limit.
	//#[pallet::storage]
	//#[pallet::getter(fn auction_period_limit)]
	//pub type AuctionPeriodLimit<T: Config> = StorageValue<_, T::BlockNumber = (1000 as u32).into(), ValueQuery>;
					
		
	/// A map of bids accessible by account id and mogwai hash.
	#[pallet::storage]
	#[pallet::getter(fn bid_of)]
	pub type Bids<T: Config> = StorageMap<_, Blake2_128Concat, (T::Hash, T::AccountId), BalanceOf<T>, ValueQuery>;
	/// A vec of accounts accessible by mogwai hash.
	#[pallet::storage]
	#[pallet::getter(fn bid_accounts)]
	pub type BidAccounts<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Vec<T::AccountId>, ValueQuery>;
	/// A map of game events accessible by the game event id (hash).
	#[pallet::storage]
	#[pallet::getter(fn game_events)]
	pub type GameEvents<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, GameEvent<T::Hash, T::BlockNumber, GameEventType>, ValueQuery>;


//		/// A map of all existing game events accessible by the index. 
//		AllGameEventsArray get(fn game_event_by_index): map hasher(blake2_128_concat) u64 => T::Hash;
//		/// A count over all existing game events in the system.
//		AllGameEventsCount get(fn all_game_events_count): u64;
//		/// A map of the index of the game events accessible by the game event id (hash).
//		AllGameEventsIndex: map hasher(identity) T::Hash => u64;
//
//		/// A map of all game event ids (hash) associated with an game event type (indexed).
//		GameEventsArray get(fn game_event_of_type_by_index): map hasher(blake2_128_concat) (GameEventType, u64) => T::Hash;
//		/// A count over all existing game events of one particular game event type.
//		GameEventsCount get(fn game_event_of_type_count): map hasher(blake2_128_concat) GameEventType => u64;
//		/// A map of the game event type index accessible by the game event id (hash).
//		GameEventsIndex: map hasher(identity) T::Hash => u64;
//
//		/// A vec of game event ids (hash) accessible by the triggering block number.
//		GameEventsAtBlock get(fn game_events_at_block): map hasher(blake2_128_concat) T::BlockNumber => Vec<T::Hash>;	
//
//		/// A vec of game event ids (hash) accessible by the corresponding mogwai.
//		GameEventsOfMogwai get(fn game_events_of_mogwai): map hasher(identity) T::Hash => Vec<T::Hash>;
//
//		/// The nonce used for randomness.
//		Nonce: u64 = 0;


	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub key: T::AccountId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { key: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			<Key<T>>::put(&self.key);
		}
	}

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
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

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
