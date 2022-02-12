import React, { useState, useEffect } from "react";

const ENDPOINT = "ws://127.0.0.1:8080/subscribe";

function App() {
  const [state, setState] = useState("unknown");

  useEffect(() => {
    console.log("connecting");
    const ws = new WebSocket(ENDPOINT);

    ws.onopen = () => {
      console.log("connected");
    };

    ws.onmessage = (event) => {
      console.log(event.data);
      setState(event.data.toString());
    };

    ws.onclose = () => {
      console.log("disconnected");
    };
  });

  console.log("debug");

  return <div>Hello {state}</div>;
}

export default App;
