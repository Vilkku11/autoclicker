import { invoke } from "@tauri-apps/api/tauri";
import "./Cps.css";
import { useState } from "react";
const Cps = () => {
  const [cps, setCps] = useState(0);
  const [keys, setKeys] = useState("LShift + A");
  const [list, setList] = useState();

  const click = async (event) => {
    event.preventDefault();
    console.log(cps);

    const data = [String(cps), keys];

    await invoke("click", { data: data });
  };

  const setKeyBind = async () => {
    await invoke("set_key_bind").then((list) => {
      setList(list);
      let keybind = "";
      for (const key of list) {
        if (keybind != "") {
          keybind = keybind + " + " + key;
        } else {
          keybind = keybind + key;
        }
      }
      setKeys(keybind);
    });
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
      <label>Keybind to stop:</label>
      <button onClick={setKeyBind}>{keys}</button>
    </div>
  );
};

export default Cps;
