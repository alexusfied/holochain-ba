import { useContext } from "react";

import "./app.css";
import { ClientContext } from "./ClientContext";
import AllEvents from "./holochain_ba/event/AllEvents";

const App = () => {
  const { error, loading } = useContext(ClientContext);
  return (
    <>
      <h1>Holochain-BA by Alexander Fiedler</h1>
      <div>
        <div className="card">
          {loading ? "connecting..." : (error ? error.message : "Client is connected.")}
        </div>
        <AllEvents />
      </div>
    </>
  );
};

export default App;
