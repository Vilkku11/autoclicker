import { invoke } from "@tauri-apps/api/tauri";
import "./Cps.css";
import { useState } from "react";
const Cps = () => {
  const [cps, setCps] = useState(0);

  const click = async (event) => {
    event.preventDefault();
    console.log(cps);
    await invoke("click", { cps: cps });
  };

  const setKeyBind = async () => {
    await invoke("set_key_bind");
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
      <button onClick={setKeyBind}></button>
    </div>
  );
};

export default Cps;
