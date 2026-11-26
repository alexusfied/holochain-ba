use hdi::prelude::*;
use crate::*;

fn same_uuid(a: &str, b: &str) -> bool {
    match (uuid::Uuid::try_parse(a), uuid::Uuid::try_parse(b)) {
        (Ok(x), Ok(y)) => x == y,
        _ => false
    }
}

fn validate_event_identifier_format(id_type: &str, id_value: &str) -> bool {
    id_type == "UUID" && uuid::Uuid::try_parse(id_value).is_ok()
}

fn validate_event_identifier_unique(action: &EntryCreationAction, id_value: &str) -> ExternResult<bool> {
    let result: Vec<RegisterAgentActivity> = must_get_agent_activity(action.author().clone(), ChainFilter::new(action.prev_action().clone()))?;

    let event_entry_def = &EntryType::App(UnitEntryTypes::Event.try_into()?);
    let entry_actions = result
        .iter()
        .filter(|o| o.action.hashed.content.entry_type() == Some(event_entry_def));


    for action in entry_actions {
        if let Some(entry_hash) = action.action.hashed.content.entry_hash() {
            let prev: Event = must_get_entry(entry_hash.clone())?.content.try_into()?;
            if same_uuid(&prev.event_identifier.event_identifier_value, id_value) { return Ok(false); }
        }
    }

    return Ok(true)
}

fn validate_event_type(event_type: &str) -> bool {
    if !matches!(event_type, "ingestion" | "migration" | "validation") {
        return false;
    }

    true
}

fn validate_event_outcome(event_outcome: &str) -> bool {
    if !matches!(event_outcome, "success" | "failure") {
        return false;
    }

    true
}

pub fn validate_create_event(
    action: EntryCreationAction,
    event: Event,
) -> ExternResult<ValidateCallbackResult> {
    if !validate_event_type(&event.event_type) {
        return Ok(ValidateCallbackResult::Invalid(
            "Invalid event type: Has to be either ingestion, migration or validation".to_string()
        ));
    }

    for outcome_info in event.event_outcome_information {
        if let Some(event_outcome) = &outcome_info.event_outcome {
            if !validate_event_outcome(event_outcome) {
                return Ok(ValidateCallbackResult::Invalid(
                    "Invalid event outcome: Has to be either success or failure".to_string()
                ));
            }
        }

    }


    if !validate_event_identifier_format(&event.event_identifier.event_identifier_type, &event.event_identifier.event_identifier_value) {
        return Ok(ValidateCallbackResult::Invalid(
            "Invalid event identifier: The type has to be UUID and the value a valid UUID".to_string(),
        )); 
    } 

    if !validate_event_identifier_unique(&action, &event.event_identifier.event_identifier_value)? {
        return Ok(ValidateCallbackResult::Invalid(
            "Event identifier is already used in the chain".to_string(),
        ));
    }

    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_update_event(
    _action: Update,
    _event: Event,
    _original_action: EntryCreationAction,
    _original_event: Event,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Events cannot be updated".to_string(),
    ))
}

pub fn validate_delete_event(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_event: Event,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Events cannot be deleted".to_string(),
    ))
}

pub fn validate_create_link_all_events(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash =
        target_address
            .into_action_hash()
            .ok_or(wasm_error!(WasmErrorInner::Guest(
                "No action hash associated with link".to_string()
            )))?;
    let record = must_get_valid_record(action_hash)?;
    let _event: crate::Event = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(
            "Linked action must reference an entry".to_string()
        )))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_all_events(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(
        "Event links cannot be deleted".to_string(),
    ))
}
