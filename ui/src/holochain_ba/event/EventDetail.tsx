import { HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { FC, useCallback, useContext, useEffect, useState } from "react";

import { ClientContext } from "../../ClientContext";
import type { Event } from "./types";

const EventDetail: FC<EventDetailProps> = ({ eventHash, onEventDeleted }) => {
  const { client } = useContext(ClientContext);
  const [record, setRecord] = useState<Record | undefined>(undefined);
  const [event, setEvent] = useState<Event | undefined>(undefined);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<HolochainError | undefined>();

  const fetchEvent = useCallback(async () => {
    setLoading(true);
    setRecord(undefined);
    try {
      const result = await client?.callZome({
        role_name: "holochain_ba",
        zome_name: "event",
        fn_name: "get_event",
        payload: eventHash,
      });
      setRecord(result);
      setLoading(false);
    } catch (e) {
      setError(e as HolochainError);
    } finally {
      setLoading(false);
    }
  }, [client, eventHash]);

  useEffect(() => {
    if (!eventHash) {
      throw new Error(`The eventHash prop is required for this component`);
    }
    fetchEvent();
  }, [fetchEvent, eventHash]);

  useEffect(() => {
    if (!record) return;
    setEvent(decode((record.entry as any).Present.entry) as Event);
  }, [record]);

  if (loading) {
    return <progress />;
  }

  if (error) {
    return <div className="alert">Error: {error.message}</div>;
  }

  return (
    <div>
      {record
        ? (
          <section>
            <div>
            </div>
          </section>
        )
        : <div className="alert">The requested event was not found.</div>}
    </div>
  );
};

interface EventDetailProps {
  eventHash: Uint8Array;
  onEventDeleted?: (eventHash: Uint8Array) => void;
}

export default EventDetail;
