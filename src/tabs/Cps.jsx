import { invoke } from "@tauri-apps/api/tauri";
import "./Cps.css";
const Cps = () => {
  const click = async () => {
    await invoke("click");
  };

  return (
    <div>
      <label>Click per second</label>
      <input type="number" min="0" max="5"></input>
      <button onClick={click}>Start</button>
    </div>
  );
};

export default Cps;
