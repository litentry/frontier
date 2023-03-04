use pallet_evmless::{Precompile, PrecompileHandle, PrecompileResult, PrecompileSet};
use sp_core::{H160, U256};
use sp_std::marker::PhantomData;
use sp_runtime::traits::Dispatchable;
use frame_support::dispatch::{PostDispatchInfo, GetDispatchInfo};

use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use pallet_evmless_precompile_fungibles::{AssetIdParameterOf, AssetIdOf, BalanceOf, Fungibles};

use precompile_utils::EvmData;

pub struct FrontierPrecompiles<R>(PhantomData<R>);

impl<R> FrontierPrecompiles<R>
where
	R: pallet_evmless::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}
	pub fn used_addresses() -> [H160; 8] {
		[
			hash(1),
			hash(2),
			hash(3),
			hash(4),
			hash(5),
			hash(1024),
			hash(1025),
			hash(1337),
		]
	}
}
impl<R> PrecompileSet for FrontierPrecompiles<R>
where
	R: pallet_evmless::Config  + pallet_assets::Config,
	AssetIdParameterOf<R>: From<u32>,
	AssetIdOf<R>: From<u32>,
	BalanceOf<R>: EvmData + Into<U256>,
	<R as frame_system::Config>::AccountId: From<H160>,
	//<<R as frame_system::Config>::Lookup as StaticLookup>::Source: From<H160>,
	R::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	R::RuntimeCall: From<pallet_assets::Call<R>>,
	<R::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<R::AccountId>>,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(handle)),
			a if a == hash(2) => Some(Sha256::execute(handle)),
			a if a == hash(3) => Some(Ripemd160::execute(handle)),
			a if a == hash(4) => Some(Identity::execute(handle)),
			a if a == hash(5) => Some(Modexp::execute(handle)),
			// Non-Frontier specific nor Ethereum precompiles :
			a if a == hash(1024) => Some(Sha3FIPS256::execute(handle)),
			a if a == hash(1025) => Some(ECRecoverPublicKey::execute(handle)),
			// EVMless
			a if a == hash(1337) => Some(Fungibles::<R>::execute(handle)),

			_ => None,
		}
	}

	fn is_precompile(&self, address: H160) -> bool {
		Self::used_addresses().contains(&address)
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}
