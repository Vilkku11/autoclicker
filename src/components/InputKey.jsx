import { invoke } from "@tauri-apps/api";

const InputKey = (props) => {
  const setKeyBind = async () => {
    props.setKeys("Insert keybind");
    await invoke("set_key_bind").then((keyList) => {
      let keybind = "";
      for (const key of keyList) {
        if (keybind != "") {
          keybind = keybind + " + " + key;
        } else {
          keybind = keybind + key;
        }
        props.setKeys(keybind);
      }
    });
  };

  return (
    <div>
      <button onClick={setKeyBind}>{props.keys}</button>
    </div>
  );
};

export default InputKey;
