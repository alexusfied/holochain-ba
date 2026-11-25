use event_integrity::*;
use hdk::prelude::*;
use holochain::sweettest::{SweetConductor, SweetZome};

pub async fn sample_event(conductor: &SweetConductor, zome: &SweetZome) -> Event {
    Event {
        event_identifier: EventIdentifier {
            event_identifier_type: "UUID".to_string(),
            event_identifier_value: "4cdea8b3-94ed-4ffd-a16f-9e253c420897".to_string()
        },
        event_type: "creation".to_string(),
        event_date_time: "2026-10-29T14:32:05Z".to_string(),
        event_detail_information: vec![
            EventDetailInformation {
                event_detail: "Some detail information".to_string(),
                event_detail_extension: "Some detail extension".to_string()
            }
        ],
        event_outcome_information: vec![
            EventOutcomeInformation {
                event_outcome: "success".to_string(),
                event_outcome_detail: EventOutcomeDetail {
                    event_outcome_detail_note: "Some description of the result of the event".to_string(),
                    event_outcome_detail_extension: "Some outcome detail extension".to_string()
                }
            }
        ],
        linking_agent_identifier: vec![
            LinkingAgentIdentifier {
                linking_agent_identifier_type: "UUID".to_string(),
                linking_agent_identifier_value: "980d0c0a-00cd-44a2-bde5-5ab9d97c7b2a".to_string(),
                linking_agent_role: "executing program".to_string()
            }
        ],
        linking_object_identifier: vec![
            LinkingObjectIdentifier {
                linking_object_identifier_type: "UUID".to_string(),
                linking_object_identifier_value: "7adcbbeb-1e09-455d-884a-2a5a63aee42b".to_string(),
                linking_object_role: "outcome".to_string()
            }
        ]
    }
}

pub async fn create_event(conductor: &SweetConductor, zome: &SweetZome) -> Record {
    conductor
        .call(&zome, "create_event", sample_event(conductor, zome).await)
        .await
}
