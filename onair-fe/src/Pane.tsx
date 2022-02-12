import State from "./State";
import styled from "styled-components";

interface Props {
  state: State;
}

const MAP = {
  [State.Standby]: { title: "WORKING", color: "green" },
  [State.Offline]: { title: "OFFLINE", color: "gray" },
  [State.OnAir]: { title: "ON AIR", color: "red" },
};

function Pane(props: Props) {
  const { title, color } = MAP[props.state];
  return <Div style={{ backgroundColor: color }}>{title}</Div>;
}

const Div = styled.div`
  color: white;
  width: 100%;
  font-family: monospace;
  font-size: 20vw;
`;

export default Pane;
