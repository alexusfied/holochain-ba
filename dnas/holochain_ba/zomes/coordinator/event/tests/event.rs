use holochain::prelude::*;
use holochain::sweettest::*;
use std::path::Path;

mod common;
use common::*;

#[tokio::test(flavor = "multi_thread")]
async fn create_event() {
    // Create a conductor with the standard config
    let mut conductor = SweetConductor::from_standard_config().await;
    let dna_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/holochain_ba.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let app = conductor.setup_app("test-app", &[dna_file]).await.unwrap();
    let zome = app.cells()[0].zome("event");

    let event = sample_event(&conductor, &zome).await;

    // Agent creates a Event
    let _: Record = conductor.call(&zome, "create_event", event.clone()).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn create_and_read_event() {
    // Create conductors with the standard config
    let mut conductors = SweetConductorBatch::from_standard_config_rendezvous(2).await;
    let dna_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../workdir/holochain_ba.dna");
    let dna_file = SweetDnaFile::from_bundle(&dna_path).await.unwrap();
    let apps = conductors.setup_app("test-app", &[dna_file]).await.unwrap();
    let cells = apps.cells_flattened();
    let alice_conductor = conductors.get(0).unwrap();
    let alice_zome = cells[0].zome("event");
    let bob_conductor = conductors.get(1).unwrap();
    let bob_zome = cells[1].zome("event");

    let event = sample_event(&alice_conductor, &alice_zome).await;

    // Alice creates a Event
    let record: Record = alice_conductor
        .call(&alice_zome, "create_event", event.clone())
        .await;

    // Wait for the created entry to be propagated to the other node.
    await_consistency(&cells).await.unwrap();

    // Bob gets the created Event
    let propagated_record: Record = bob_conductor
        .call(
            &bob_zome,
            "get_event",
            record.signed_action.hashed.hash.clone(),
        )
        .await;
    assert_eq!(record, propagated_record);
}
