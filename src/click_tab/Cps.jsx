import { useState } from "react";
import "./Cps.css";
const Cps = () => {
  const [start, setStart] = useState("Start");

  return (
    <div>
      <label>Click per second</label>
      <input type="number" min="0" max="5"></input>
      <button
        onClick={() => {
          if (start == "Start") {
            setStart("Stop");
          } else {
            setStart("Start");
          }
        }}
      >
        {start}
      </button>
    </div>
  );
};

export default Cps;
