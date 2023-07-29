import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Cps from "./components/Cps";

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
      <button onClick={testButton}>test</button>
      <Cps />
    </div>
  );
}

export default App;
