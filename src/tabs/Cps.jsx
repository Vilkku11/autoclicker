import { invoke } from "@tauri-apps/api/tauri";
import "./Cps.css";
import { useState } from "react";
import InputKey from "../components/InputKey";

const Cps = () => {
  const [cps, setCps] = useState(0);
  const [keys, setKeys] = useState("LShift + A");

  const click = async (event) => {
    event.preventDefault();
    console.log(cps);

    const data = [String(cps), keys];

    await invoke("click", { data: data });
  };

  return (
    <div>
      <label>Click per second</label>
      <form onSubmit={click}>
        <input
          id="cps"
          value={cps}
          type="number"
          min="1"
          max="5000"
          onChange={(event) => {
            setCps(event.target.value);
          }}
        ></input>
        <button>Start</button>
      </form>
      <label>Set keybind to stop:</label>
      <InputKey keys={keys} setKeys={setKeys} />
    </div>
  );
};

export default Cps;
