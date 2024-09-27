#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::Application;
use linera_sdk::{
    base::WithServiceAbi,
    views::{View, ViewStorageContext},
    Service, ServiceRuntime,
};

pub struct ApplicationService {
    state: Application,
    runtime: ServiceRuntime<Self>,
}

linera_sdk::service!(ApplicationService);

impl WithServiceAbi for ApplicationService {
    type Abi = zk_airdrop_demo::ApplicationAbi;
}

impl Service for ApplicationService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Application::load(ViewStorageContext::from(runtime.key_value_store()))
            .await
            .expect("Failed to load state");
        ApplicationService { state, runtime }
    }

    async fn handle_query(&self, _query: Self::Query) -> Self::QueryResponse {
        panic!("Queries not supported by application");
    }
}
