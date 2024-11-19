import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

import InputKey from "../components/InputKey";

const Hold = () => {
  const [keys, setKeys] = useState("LShift + A");

  const [keyToHold, setKeyToHold] = useState("LShift");

  const startHold = async () => {
    let data = [keyToHold, keys];

    await invoke("hold", { data: data });
  };

  return (
    <div>
      <label>hold</label>
      <button onClick={startHold}>hold</button>
      <label>Set keybind to stop:</label>
      <InputKey keys={keys} setKeys={setKeys} />
      <label>Set key to hold:</label>
      <InputKey keys={keyToHold} setKeys={setKeyToHold} />
    </div>
  );
};

export default Hold;
