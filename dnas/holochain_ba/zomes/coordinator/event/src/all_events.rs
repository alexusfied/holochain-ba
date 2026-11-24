use event_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn get_all_events() -> ExternResult<Vec<Link>> {
    let path = Path::from("all_events");
    get_links(
        LinkQuery::try_new(path.path_entry_hash()?, LinkTypes::AllEvents)?,
        GetStrategy::default(),
    )
}
