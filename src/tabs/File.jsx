import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

const File = () => {
  const [filePath, setFilePath] = useState("No file");

  const handlePath = async () => {
    await invoke("get_file_path").then((path) => {
      setFilePath(path);
    });
  };

  return (
    <div>
      <h1>file</h1>
      <button onClick={handlePath}>Choose file</button>
      <p>{filePath}</p>
    </div>
  );
};

export default File;
