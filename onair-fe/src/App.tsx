import React, { useState, useEffect, useRef } from "react";
import Pane from "./Pane";
import State from "./State";

const SERVER = window.location.host;
const ENDPOINT = `ws://${SERVER}/subscribe`;

interface Update {
  onair: boolean;
}

function App() {
  const [state, setState] = useState(State.Offline);
  const received = useRef(true);
  const [alive, setAlive] = useState(true);

  const connect = () => {
    console.log(`connectiong ${ENDPOINT}`);
    const ws = new WebSocket(ENDPOINT);

    ws.onopen = () => {
      console.log("connected");
    };

    ws.onmessage = (event) => {
      const update: Update = JSON.parse(event.data);
      console.log(`updated ${update.onair}`);
      const s = update.onair ? State.OnAir : State.Standby;
      const state = alive ? s : State.Offline;
      received.current = true;
      setState(state);
      setAlive(true);
    };

    ws.onerror = (err) => {
      console.log("connection error", err);
    };

    ws.onclose = () => {
      console.log("disconnected");

      setTimeout(function() {
        connect();
      }, 1000);
    };
  };

  const checkLiveness = () => {
    console.log(`check liveness ${received.current}`);
    if (!received.current) {
      console.log("no updates, not alive");
      setAlive(false);
      setState(State.Offline);
    }
    received.current = false;
  };

  useEffect(() => {
    connect();

    const interval = setInterval(checkLiveness, 60000);

    return () => clearInterval(interval);
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  return <Pane state={state} />;
}

export default App;
