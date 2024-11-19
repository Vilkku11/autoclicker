import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

import InputKey from "../components/InputKey";

const Cursor = () => {
  const [keys, setKeys] = useState("LShift + A");
  const moveCursor = async (state) => {
    let data = [state, keys];

    await invoke("cursor", { data });
  };

  return (
    <div>
      <button onClick={() => moveCursor("square")}>square</button>
      <button onClick={() => moveCursor("random")}>random</button>
      <label>Set keybind to stop:</label>
      <InputKey keys={keys} setKeys={setKeys} />
    </div>
  );
};
export default Cursor;
