import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, Record } from "@holochain/client";
import { FC, useContext, useEffect, useState } from "react";

import { ClientContext } from "../../ClientContext";
import type { Event } from "./types";

const CreateEvent: FC<CreateEventProps> = ({ onEventCreated, eventIdentifier }) => {
  const { client } = useContext(ClientContext);
  const [isEventValid, setIsEventValid] = useState(false);

  const createEvent = async () => {
    const eventEntry: Event = {
      event_identifier: eventIdentifier!,
    };
    try {
      const record = await client?.callZome({
        role_name: "holochain_ba",
        zome_name: "event",
        fn_name: "create_event",
        payload: eventEntry,
      });
      onEventCreated && onEventCreated(record.signed_action.hashed.hash);
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    setIsEventValid(true);
  }, []);

  return (
    <div>
      <h3>Create Event</h3>
      <button disabled={!isEventValid} onClick={() => createEvent()}>
        Create Event
      </button>
    </div>
  );
};

interface CreateEventProps {
  onEventCreated?: (hash?: Uint8Array) => void;
  eventIdentifier: string;
}

export default CreateEvent;
