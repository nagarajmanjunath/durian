use crate::v1;
pub use jsonrpc_core::{Compatibility, Error, MetaIoHandler};
use std::collections::HashSet;
use std::str::FromStr;
use ethereum_types::{Address, H160, H256, U256, U512};
use jsonrpc_core::futures::{future, Future};
use jsonrpc_core::types::Value;
use jsonrpc_core::{BoxFuture, Result};
use v1::helpers::errors;
use v1::traits::TransactionRPC;
use v1::types::TransactionRequest;
use v1::impls::TransactionRPCImpl;
use v1::metadata::Metadata;
use blockchain::blockchain::Blockchain;


#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Api {
	/// Transaction methods
	Transaction

}

#[derive(Debug, PartialEq, Clone)]
pub enum ApiSet {
	List(HashSet<Api>),
}

impl Default for ApiSet {
	fn default() -> Self {
		ApiSet::List(vec![Api::Transaction].into_iter().collect())
	}
}


impl ApiSet {
	pub fn list_apis(&self) -> HashSet<Api> {
		match *self {
			ApiSet::List(ref apis) => apis.clone(),
		}
	}
}

pub fn setup_rpc(mut handler: MetaIoHandler<Metadata>, apis: ApiSet,bc:Blockchain) -> MetaIoHandler<Metadata> {
	for api in apis.list_apis() {
		match api {
			Api::Transaction => handler.extend_with(TransactionRPCImpl::new(bc.clone()).to_delegate()),
		}
	}

	handler
}