import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

import Cps from "./tabs/Cps";
import Cursor from "./tabs/Cursor";

//react tabs test
import { Tab, Tabs, TabList, TabPanel } from "react-tabs";
//import "react-tabs/style/react-tabs.css";

function App() {
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  const testButton = async () => {
    await invoke("test");
  };

  return (
    <div>
      <h1>Autoclicker!</h1>
      <Tabs>
        <TabList>
          <Tab>Clicker</Tab>
          <Tab>Cursor</Tab>
        </TabList>

        <TabPanel>
          <Cps />
        </TabPanel>
        <TabPanel>
          <Cursor />
        </TabPanel>
      </Tabs>
    </div>
  );
}

export default App;
