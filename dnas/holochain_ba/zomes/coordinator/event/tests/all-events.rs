use holochain::prelude::*;
use holochain::sweettest::*;
use std::path::Path;

mod common;
use common::*;

#[tokio::test(flavor = "multi_thread")]
async fn create_a_event_and_get_all_events() {
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

    // Bob gets all events
    let collection_output: Vec<Link> = bob_conductor.call(&bob_zome, "get_all_events", ()).await;
    assert_eq!(collection_output.len(), 0);

    // Alice creates a Event
    let create_record: Record = create_event(&alice_conductor, &alice_zome).await;

    // Wait for the created entry to be propagated to the other node.
    await_consistency(&cells).await.unwrap();

    // Bob gets all events again
    let collection_output: Vec<Link> = bob_conductor.call(&bob_zome, "get_all_events", ()).await;
    assert_eq!(collection_output.len(), 1);
    assert_eq!(
        collection_output[0].target,
        create_record.signed_action.hashed.hash.clone().into()
    );
}
