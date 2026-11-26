use hdi::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventIdentifier {
    pub event_identifier_type: String,
    pub event_identifier_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventDetailInformation {
    #[serde(default)]
    pub event_detail: Option<String>,
    #[serde(default)]
    pub event_detail_extension: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventOutcomeDetail {
    #[serde(default)]
    pub event_outcome_detail_note: Option<String>,
    #[serde(default)]
    pub event_outcome_detail_extension: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EventOutcomeInformation {
    #[serde(default)]
    pub event_outcome: Option<String>,
    #[serde(default)]
    pub event_outcome_detail: Vec<EventOutcomeDetail>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LinkingAgentIdentifier {
    pub linking_agent_identifier_type: String,
    pub linking_agent_identifier_value: String,
    #[serde(default)]
    pub linking_agent_role: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LinkingObjectIdentifier {
    pub linking_object_identifier_type: String,
    pub linking_object_identifier_value: String,
    #[serde(default)]
    pub linking_object_role: Vec<String>,
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
    pub linking_agent_identifier: Vec<LinkingAgentIdentifier>,
    #[serde(default)]
    pub linking_object_identifier: Vec<LinkingObjectIdentifier>,
}

fn validate_event_identifier(id_type: &str, id_value: &str) -> bool {
    id_type == "UUID" && uuid::Uuid::try_parse(id_value).is_ok()
}

pub fn validate_create_event(
    _action: EntryCreationAction,
    _event: Event,
) -> ExternResult<ValidateCallbackResult> {
    if !validate_event_identifier(&_event.event_identifier.event_identifier_type, &_event.event_identifier.event_identifier_value) {
        return Ok(ValidateCallbackResult::Invalid(
            "Invalid event identifier: The type has to be UUID and the value a valid UUID".to_string(),
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
