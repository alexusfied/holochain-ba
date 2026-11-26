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

