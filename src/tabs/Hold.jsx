import { invoke } from "@tauri-apps/api/tauri";

const Hold = () => {
  const startHold = async () => {
    await invoke("hold");
  };

  return (
    <div>
      <label>hold</label>
      <button onClick={startHold}>hold</button>
    </div>
  );
};

export default Hold;
