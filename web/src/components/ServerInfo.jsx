import { Td, Tr, useHighlight } from "@chakra-ui/react";
import { useEffect, useState } from "react";

const ServerInfo = ({ server }) => {
  const [state, setState] = useState({
    id: null,
    currentValue: null,
    timestamp: null,
  });

  useEffect(() => {
    setState({
      id: server[0],
      currentValue: server[1],
      timestamp: server[2] && server[2].replace("Instant", ""),
    });
  }, [server]);

  let serverId = state.id;
  let currentValue = state.currentValue;
  let timestamp = state.timestamp;

  return (
    server && (
      <Tr>
        <Td>{serverId}</Td>
        <Td>{currentValue}</Td>
        <Td>{timestamp}</Td>
      </Tr>
    )
  );
};

export default ServerInfo;
