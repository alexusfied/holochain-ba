use event_integrity::*;
use hdk::prelude::*;
use holochain::sweettest::{SweetConductor, SweetZome};

const FULL_EVENT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../../../test_data/full_event.json"
));

pub async fn sample_event(_conductor: &SweetConductor, _zome: &SweetZome) -> Event {
    serde_json::from_str(FULL_EVENT_JSON).expect("full_event.json does not match Event")
}

pub async fn create_event(conductor: &SweetConductor, zome: &SweetZome) -> Record {
    conductor
        .call(&zome, "create_event", sample_event(conductor, zome).await)
        .await
}
