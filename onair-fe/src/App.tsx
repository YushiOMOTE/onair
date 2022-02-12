import React, { useState, useEffect, useRef } from "react";
import Pane from "./Pane";
import State from "./State";

const SERVER = process.env.SERVER || "127.0.0.1:8080";
const ENDPOINT = `ws://${SERVER}/subscribe`;

interface Update {
  onair: boolean;
}

function App() {
  const [state, setState] = useState(State.Offline);
  const received = useRef(true);
  const [timedout, setTimedout] = useState(false);

  useEffect(() => {
    console.log(`connectiong ${ENDPOINT}`);
    const ws = new WebSocket(ENDPOINT);

    ws.onopen = () => {
      console.log("connected");
    };

    ws.onmessage = (event) => {
      const update: Update = JSON.parse(event.data);
      console.log(`updated ${update.onair}`);
      const s = update.onair ? State.OnAir : State.Standby;
      const state = timedout ? State.Offline : s;
      received.current = true;
      setState(state);
      setTimedout(false);
    };

    ws.onclose = () => {
      console.log("disconnected");
    };

    const interval = setInterval(() => {
      console.log(`tick ${received.current}`);
      if (!received.current) {
        console.log("timed out");
        setTimedout(true);
        setState(State.Offline);
      }
      received.current = false;
    }, 60000);

    return () => clearInterval(interval);
  }, []);

  return <Pane state={state} />;
}

export default App;
