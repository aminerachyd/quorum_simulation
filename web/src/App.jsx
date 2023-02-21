import "./App.css";
import {
  Button,
  ChakraProvider,
  Container,
  Flex,
  Input,
  Table,
  TableContainer,
  Tbody,
  Text,
  Th,
  Thead,
  Tr,
} from "@chakra-ui/react";
import { useState } from "react";
import ServerInfo from "./components/ServerInfo";
import config from "./config.json";

function App() {
  const [inputData, setInputData] = useState(0);
  const [state, setState] = useState({
    currentValue: null,
    servers: [],
    loadingWrite: false,
    loadingRead: false,
  });

  const ENDPOINT =
    process.env.REACT_APP_SERVER_ENDPOINT ||
    config.SERVER_ENDPOINT ||
    "http://localhost:8080/quorum";

  const readQuorum = () => {
    setState({ ...state, loadingRead: true });
    fetch(ENDPOINT, {
      method: "GET",
    })
      .then((res) => res.json())
      .then((res) => {
        console.log(res);
        setState({
          ...state,
          loadingRead: false,
          servers: res.servers_state,
          currentValue: res.response,
        });
      });
  };

  const writeQuorum = () => {
    setState({ ...state, loadingWrite: true });
    fetch(`${ENDPOINT}/${inputData}`, {
      method: "POST",
    })
      .then((res) => res.json())
      .then((res) => {
        console.log(res);
        setState({
          ...state,
          loadingWrite: false,
        });
      });
  };

  return (
    <ChakraProvider>
      <Container width={1000} margin={2}>
        {state.servers !== [] && (
          <>
            <div className="App">
              <Text fontSize="5xl" align={"start"}>
                Quorum simulation
              </Text>
            </div>
            <Flex mt={3} mb={3} w={1000}>
              <Button
                onClick={readQuorum}
                isDisabled={state.loadingRead}
                colorScheme={"green"}
                mr={3}
              >
                Fetch data
              </Button>
              <Button
                mr={3}
                onClick={writeQuorum}
                isDisabled={state.loadingWrite}
                colorScheme={"red"}
              >
                Write data
              </Button>
              <Input
                w={150}
                mr={3}
                type={"number"}
                onChange={(event) => {
                  setInputData(event.target.value);
                }}
              ></Input>
              <Text alignSelf="center">
                Current value : {state.currentValue}
              </Text>
            </Flex>

            <TableContainer overflowX={"visible"} w={1000}>
              <Table size="sm">
                <Thead>
                  <Tr>
                    <Th>Server ID</Th>
                    <Th>Current value</Th>
                    <Th>Last timestamp</Th>
                  </Tr>
                </Thead>
                <Tbody>
                  {state.servers.map((server) => (
                    <ServerInfo server={server}></ServerInfo>
                  ))}
                </Tbody>
              </Table>
            </TableContainer>
          </>
        )}
      </Container>
    </ChakraProvider>
  );
}

export default App;
