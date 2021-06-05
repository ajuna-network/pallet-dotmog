// This file is part of Substrate.

// Copyright (C) 2020-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Test utilities

use super::*;
use crate as pallet_dotmog;

use frame_support::{
	parameter_types, ord_parameter_types,
	traits::{OnInitialize, OnFinalize, TestRandomness},
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use frame_system::EnsureSignedBy;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		DotMogModule: pallet_dotmog::{Pallet, Call, Storage, Event<T>, Config<T>},
	}
);

parameter_types! {
	pub const CandidateDeposit: u64 = 25;
	pub const WrongSideDeduction: u64 = 2;
	pub const MaxStrikes: u32 = 2;
	pub const RotationPeriod: u64 = 4;
	pub const PeriodSpend: u64 = 1000;
	pub const MaxLockDuration: u64 = 100;
	pub const ChallengePeriod: u64 = 8;
	pub const BlockHashCount: u64 = 250;
	pub const ExistentialDeposit: u64 = 1;

	pub const PalletId: PalletId = PalletId(*b"py/dtmog");
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}

ord_parameter_types! {
	pub const FounderSetAccount: u128 = 1;
	pub const SuspensionJudgementSetAccount: u128 = 2;
}

impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Call = Call;
	type Hashing = BlakeTwo256;
	type AccountId = u128;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl Config for Test {
	type PalletId = DotMogPalletId;
	type Event = Event;
	type Currency = pallet_balances::Module<Self>;
	type Randomness = TestRandomness;
	type PricePayment = ();
	//type Scheduler = Scheduler;
	//type PalletsOrigin = OriginCaller;
}
