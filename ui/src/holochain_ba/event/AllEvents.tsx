import { HolochainError, Link, SignalCb } from "@holochain/client";
import { FC, useCallback, useContext, useEffect, useState } from "react";

import { ClientContext } from "../../ClientContext";
import EventDetail from "./EventDetail";
import type { EventSignal } from "./types";

const AllEvents: FC = () => {
  const { client } = useContext(ClientContext);
  const [hashes, setHashes] = useState<Uint8Array[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<HolochainError | undefined>();

  const fetchEvents = useCallback(async () => {
    setLoading(true);
    try {
      const links: Link[] = await client?.callZome({
        role_name: "holochain_ba",
        zome_name: "event",
        fn_name: "get_all_events",
      });
      if (links?.length) {
        setHashes(links.map((l) => l.target));
      }
    } catch (e) {
      setError(e as HolochainError);
    } finally {
      setLoading(false);
    }
  }, [client]);

  const handleSignal: SignalCb = useCallback((signal) => {
    if (signal.type !== "app") return;
    if (signal.value.zome_name !== "event") return;
    const payload = signal.value.payload as EventSignal;
    if (payload.type !== "EntryCreated") return;
    if (payload.app_entry.type !== "Event") return;
    setHashes((prevHashes) => [...prevHashes, payload.action.hashed.hash]);
  }, [setHashes]);

  useEffect(() => {
    fetchEvents();
    client?.on("signal", handleSignal);
  }, [client, handleSignal, fetchEvents]);

  if (loading) {
    return <progress />;
  }

  return (
    <div>
      {error ? <div className="alert">Error fetching the events: {error.message}</div> : hashes.length > 0
        ? (
          <div>
            {hashes.map((hash, i) => <EventDetail key={i} eventHash={hash} onEventDeleted={fetchEvents} />)}
          </div>
        )
        : <div className="alert">No events found.</div>}
    </div>
  );
};

export default AllEvents;
