use event_integrity::*;
use hdk::prelude::*;
use holochain::sweettest::{SweetConductor, SweetZome};

pub async fn sample_event(conductor: &SweetConductor, zome: &SweetZome) -> Event {
    Event {
        event_identifier: Default::default(),
    }
}

pub async fn create_event(conductor: &SweetConductor, zome: &SweetZome) -> Record {
    conductor
        .call(&zome, "create_event", sample_event(conductor, zome).await)
        .await
}
