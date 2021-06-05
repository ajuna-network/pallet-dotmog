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

//! Tests for the module.

use super::*;
use mock::*;

use frame_support::{assert_ok, assert_noop};
use sp_runtime::traits::BadOrigin;
use sp_core::blake2_256;

#[test]
fn it_works_for_default_value() {
	//new_test_ext().execute_with(|| {
	//	// Dispatch a signed extrinsic.
	//	assert_ok!(DotMogModule::do_something(Origin::signed(1), 42));
	//	// Read pallet storage and assert an expected result.
	//	assert_eq!(DotMogModule::something(), Some(42));
	//});
}

#[test]
fn correct_error_for_none_value() {
	//new_test_ext().execute_with(|| {
	//	// Ensure the expected error is thrown when no value is present.
	//	assert_noop!(
	//		DotMogModule::cause_error(Origin::signed(1)),
	//		Error::<Test>::NoneValue
	//	);
	//});
}