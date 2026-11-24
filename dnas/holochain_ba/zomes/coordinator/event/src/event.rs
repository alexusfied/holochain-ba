use event_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn create_event(event: Event) -> ExternResult<Record> {
    let event_hash = create_entry(&EntryTypes::Event(event.clone()))?;
    let record = get(event_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Could not find the newly created Event".to_string())
    ))?;
    let path = Path::from("all_events");
    create_link(
        path.path_entry_hash()?,
        event_hash.clone(),
        LinkTypes::AllEvents,
        (),
    )?;
    Ok(record)
}

#[hdk_extern]
pub fn get_event(event_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(event_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed get details response".to_string()
        ))),
    }
}
