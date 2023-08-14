import { invoke } from "@tauri-apps/api/tauri";

const Cursor = () => {
  const moveCursor = async (state) => {
    await invoke("cursor", { state });
  };

  return (
    <div>
      <button onClick={() => moveCursor("square")}>square</button>
      <button onClick={() => moveCursor("random")}>random</button>
    </div>
  );
};
export default Cursor;
