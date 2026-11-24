import type {
  ActionHash,
  AgentPubKey,
  Create,
  CreateLink,
  Delete,
  DeleteLink,
  DnaHash,
  EntryHash,
  ExternalHash,
  Record,
  SignedActionHashed,
  Update,
} from "@holochain/client";

export type EventSignal = {
  type: "EntryCreated";
  action: SignedActionHashed<Create>;
  app_entry: EntryTypes;
} | {
  type: "EntryUpdated";
  action: SignedActionHashed<Update>;
  app_entry: EntryTypes;
  original_app_entry: EntryTypes;
} | {
  type: "EntryDeleted";
  action: SignedActionHashed<Delete>;
  original_app_entry: EntryTypes;
} | {
  type: "LinkCreated";
  action: SignedActionHashed<CreateLink>;
  link_type: string;
} | {
  type: "LinkDeleted";
  action: SignedActionHashed<DeleteLink>;
  link_type: string;
};

/* dprint-ignore-start */
export type EntryTypes =
 | ({  type: 'Event'; } & Event);
/* dprint-ignore-end */

interface EventIdentifier {
  event_identifier_type: string,
  event_identifier_value: string
}

interface EventDetailInformation {
  event_detail: string,
  event_detail_extension: string
}

interface EventOutcomeDetail {
  event_outcome_detail_note: string,
  event_outcome_detail_extension: string
}

interface EventOutcomeInformation {
  event_outcome: string,
  event_outcome_detail: EventOutcomeDetail 
}

interface LinkingAgentIdentifier {
  linking_agent_identifier_type: string,
  linking_agent_identifier_value: string,
  linking_agent_role: string
}

interface LinkingObjectIdentifier {
  linking_object_identifier_type: string,
  linking_object_identifier_value: string,
  linking_object_role: string
}

export interface Event {
  event_identifier: EventIdentifier,
  event_type: string,
  event_date_time: string,
  event_detail_information: EventDetailInformation[],
  event_outcome_information: EventOutcomeInformation[],
  linking_agent_identifier: LinkingAgentIdentifier[],
  linking_object_identifier: LinkingObjectIdentifier[]

}
