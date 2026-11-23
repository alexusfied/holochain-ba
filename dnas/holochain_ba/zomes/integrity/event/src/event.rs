use hdi::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct EventIdentifier {
    event_identifier_type: String,
    event_identifier_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct EventDetailInformation {
    event_detail: String,
    event_detail_extension: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct EventOutcomeDetail {
    event_outcome_detail_note: String,
    event_outcome_detail_extension: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct EventOutcomeInformation {
    event_outcome: String,
    event_outcome_detail: EventOutcomeDetail
}

#[derive(Clone, PartialEq)]
#[hdk_entry_helper]
pub struct Event {
    pub event_identifier: EventIdentifier,
    pub event_type: String,
    pub event_date_time: String,
    #[serde(default)]
    pub event_detail_information: Vec<EventDetailInformation>,
    #[serde(default)]
    pub event_outcome_information: Vec<EventOutcomeInformation>,
    #[serde(default)]
    pub linking_agent_identifier: Vec<String>,
    #[serde(default)]
    pub linking_object_identifier: Vec<String>,
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
