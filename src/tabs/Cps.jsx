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
    </div>
  );
};

export default Cps;
