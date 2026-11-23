use hdi::prelude::*;

struct EventIdentifier {
    event_identifier_type: String,
    event_identifier_value: String,
}

struct EventDetailInformation {
    event_detail: String,
    event_detail_extension: String,
}

struct EventOutcomeDetail {
    event_outcome_detail_note: String,
    event_outcome_detail_extension: String,
}

struct EventOutcomeInformation {
    event_outcome: String,
    event_outcome_detail: EventOutcomeDetail
}

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct Event {
    pub event_identifier: EventIdentifier,
    pub event_type: String,
    pub event_date_time: Timestamp,
    pub event_detail_information: EventDetailInformation,
    pub event_outcome_information: EventOutcomeInformation,
    pub linking_agent_identifier: String,
    pub linking_object_identifier: String,
}

pub fn validate_create_event(
    _action: EntryCreationAction,
    _event: Event,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
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
