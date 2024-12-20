import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

import Cps from "./tabs/Cps";
import Cursor from "./tabs/Cursor";
import Hold from "./tabs/Hold";

//react tabs test
import { Tab, Tabs, TabList, TabPanel } from "react-tabs";
//import "react-tabs/style/react-tabs.css";

function App() {
  return (
    <div>
      <h1>Autoclicker!</h1>
      <Tabs>
        <TabList>
          <Tab>Click</Tab>
          <Tab>Hold</Tab>
          <Tab>Cursor</Tab>
        </TabList>

        <TabPanel>
          <Cps />
        </TabPanel>
        <TabPanel>
          <Hold />
        </TabPanel>
        <TabPanel>
          <Cursor />
        </TabPanel>
      </Tabs>
    </div>
  );
}

export default App;
