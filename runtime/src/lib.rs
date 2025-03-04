// This file is part of Darwinia.
//
// Copyright (C) 2018-2021 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

pub mod constants {
	// --- substrate ---
	use sp_staking::SessionIndex;
	// --- darwinia ---
	use crate::*;

	pub const NANO: Balance = 1;
	pub const MICRO: Balance = 1_000 * NANO;
	pub const MILLI: Balance = 1_000 * MICRO;
	pub const COIN: Balance = 1_000 * MILLI;

	pub const CAP: Balance = 10_000_000_000 * COIN;
	pub const TOTAL_POWER: Power = 1_000_000_000;

	pub const MILLISECS_PER_BLOCK: Moment = 6000;
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
	pub const BLOCKS_PER_SESSION: BlockNumber = 10 * MINUTES;
	pub const SESSIONS_PER_ERA: SessionIndex = 6;

	// Time is measured by number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = 60 * MINUTES;
	pub const DAYS: BlockNumber = 24 * HOURS;

	// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 20 * COIN + (bytes as Balance) * 100 * MICRO
	}
}

pub mod wasm {
	//! Make the WASM binary available.

	#[cfg(all(feature = "std", any(target_arch = "x86_64", target_arch = "x86")))]
	include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

	#[cfg(all(
		feature = "std",
		not(feature = "crab"),
		not(any(target_arch = "x86_64", target_arch = "x86"))
	))]
	pub const WASM_BINARY: &[u8] = include_bytes!("../wasm/darwinia_pc2_runtime.compact.wasm");
	#[cfg(all(
		feature = "std",
		feature = "crab",
		not(any(target_arch = "x86_64", target_arch = "x86"))
	))]
	pub const WASM_BINARY: &[u8] = include_bytes!("../wasm/darwinia_crab_pc2_runtime.compact.wasm");
	#[cfg(all(
		feature = "std",
		not(feature = "crab"),
		not(any(target_arch = "x86_64", target_arch = "x86"))
	))]
	pub const WASM_BINARY_BLOATY: &[u8] = include_bytes!("../wasm/darwinia_pc2_runtime.wasm");
	#[cfg(all(
		feature = "std",
		feature = "crab",
		not(any(target_arch = "x86_64", target_arch = "x86"))
	))]
	pub const WASM_BINARY_BLOATY: &[u8] = include_bytes!("../wasm/darwinia_crab_pc2_runtime.wasm");

	#[cfg(feature = "std")]
	/// Wasm binary unwrapped. If built with `BUILD_DUMMY_WASM_BINARY`, the function panics.
	pub fn wasm_binary_unwrap() -> &'static [u8] {
		#[cfg(all(feature = "std", any(target_arch = "x86_64", target_arch = "x86")))]
		return WASM_BINARY.expect(
			"Development wasm binary is not available. This means the client is \
							built with `BUILD_DUMMY_WASM_BINARY` flag and it is only usable for \
							production chains. Please rebuild with the flag disabled.",
		);
		#[cfg(all(feature = "std", not(any(target_arch = "x86_64", target_arch = "x86"))))]
		return WASM_BINARY;
	}
}

pub mod system;
pub use system::*;

pub mod timestamp;
pub use timestamp::*;

pub mod balances;
pub use balances::*;

pub mod transaction_payment;
pub use transaction_payment::*;

pub mod democracy;
pub use democracy::*;

pub mod collective;
pub use collective::*;

pub mod elections_phragmen;
pub use elections_phragmen::*;

pub mod membership;
pub use membership::*;

pub mod treasury;
pub use treasury::*;

pub mod header_mmr;
pub use header_mmr::*;

pub mod sudo;
pub use sudo::*;

pub mod utility;
pub use utility::*;

pub mod identity;
pub use identity::*;

pub mod scheduler;
pub use scheduler::*;

pub mod proxy;
pub use proxy::*;

pub mod multisig;
pub use multisig::*;

pub mod ethereum_relay;
pub use ethereum_relay::*;

pub mod ethereum_backing;
pub use ethereum_backing::*;

pub mod relayer_game;
pub use relayer_game::*;

pub mod relay_authorities;
pub use relay_authorities::*;

pub mod parachain_system;
pub use parachain_system::*;

pub mod parachain_info_;
pub use parachain_info_::*;

pub mod xcm_handler;
pub use xcm_handler::*;

// --- darwinia ---
use constants::*;
use darwinia_pc2_primitives::*;
pub use wasm::*;

// --- substrate ---
use frame_support::traits::Randomness;
use sp_api::impl_runtime_apis;
use sp_core::OpaqueMetadata;
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::Block as BlockT,
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, MultiAddress,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPallets,
>;

type Ring = Balances;

/// This runtime version.
#[cfg(not(feature = "crab"))]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("Darwinia PC2"),
	impl_name: create_runtime_str!("Darwinia PC2"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
};
#[cfg(feature = "crab")]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("Darwinia Crab PC2"),
	impl_name: create_runtime_str!("Darwinia Crab PC2"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
};

impl_opaque_keys! {
	pub struct SessionKeys {}
}

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

frame_support::construct_runtime! {
	pub enum Runtime
	where
		Block = Block,
		NodeBlock = OpaqueBlock,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		// Basic stuff; balances is uncallable initially.
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>} = 0,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Call, Storage} = 1,

		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 2,
		Balances: darwinia_balances::<Instance0>::{Pallet, Call, Storage, Config<T>, Event<T>} = 3,
		Kton: darwinia_balances::<Instance1>::{Pallet, Call, Storage, Config<T>, Event<T>} = 4,
		TransactionPayment: pallet_transaction_payment::{Pallet, Storage} = 5,

		// Governance stuff; uncallable initially.
		Democracy: darwinia_democracy::{Pallet, Call, Storage, Config, Event<T>} = 6,
		Council: pallet_collective::<Instance0>::{Pallet, Call, Storage, Origin<T>, Config<T>, Event<T>} = 7,
		TechnicalCommittee: pallet_collective::<Instance1>::{Pallet, Call, Storage, Origin<T>, Config<T>, Event<T>} = 8,
		ElectionsPhragmen: darwinia_elections_phragmen::{Pallet, Call, Storage, Config<T>, Event<T>} = 9,
		TechnicalMembership: pallet_membership::<Instance0>::{Pallet, Call, Storage, Config<T>, Event<T>} = 10,
		Treasury: darwinia_treasury::{Pallet, Call, Storage, Event<T>} = 11,
		HeaderMMR: darwinia_header_mmr::{Pallet, Call, Storage} = 12,

		Sudo: pallet_sudo::{Pallet, Call, Storage, Config<T>, Event<T>} = 13,

		// Claims. Usable initially.
		// Claims: darwinia_claims::{Pallet, Call, Storage, Config, Event<T>, ValidateUnsigned} = 14,

		// Vesting. Usable initially, but removed once all vesting is finished.
		// Vesting: darwinia_vesting::{Pallet, Call, Storage, Event<T>, Config<T>} = 15,

		// Utility module.
		Utility: pallet_utility::{Pallet, Call, Event} = 16,

		// Less simple identity module.
		Identity: pallet_identity::{Pallet, Call, Storage, Event<T>} = 17,

		// Society module.
		// Society: pallet_society::{Pallet, Call, Storage, Event<T>} = 18,

		// Social recovery module.
		// Recovery: pallet_recovery::{Pallet, Call, Storage, Event<T>} = 19,

		// System scheduler.
		Scheduler: pallet_scheduler::{Pallet, Call, Storage, Event<T>} = 20,

		// Proxy module. Late addition.
		Proxy: pallet_proxy::{Pallet, Call, Storage, Event<T>} = 21,

		// Multisig module. Late addition.
		Multisig: pallet_multisig::{Pallet, Call, Storage, Event<T>} = 22,

		// CrabIssuing: darwinia_crab_issuing::{Pallet, Call, Storage, Config, Event<T>} = 23,
		// CrabBacking: darwinia_crab_backing::{Pallet, Storage, Config<T>} = 24,

		EthereumRelay: darwinia_ethereum_relay::{Pallet, Call, Storage, Config<T>, Event<T>} = 25,
		EthereumBacking: darwinia_ethereum_backing::{Pallet, Call, Storage, Config<T>, Event<T>} = 26,
		EthereumRelayerGame: darwinia_relayer_game::<Instance0>::{Pallet, Storage} = 27,
		EthereumRelayAuthorities: darwinia_relay_authorities::<Instance0>::{Pallet, Call, Storage, Config<T>, Event<T>} = 28,

		// TronBacking: darwinia_tron_backing::{Pallet, Storage, Config<T>} = 29,

		ParachainSystem: cumulus_pallet_parachain_system::{Pallet, Call, Storage, Inherent, Event} = 30,
		ParachainInfo: parachain_info::{Pallet, Storage, Config} = 31,
		XcmHandler: cumulus_pallet_xcm_handler::{Pallet, Call, Event<T>, Origin} = 32,
	}
}

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			Runtime::metadata().into()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(
			extrinsic: <Block as BlockT>::Extrinsic,
		) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: sp_inherents::InherentData) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			RandomnessCollectiveFlip::random_seed().0
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, sp_core::crypto::KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}

		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}
	}
}

cumulus_pallet_parachain_system::register_validate_block!(Runtime, Executive);
