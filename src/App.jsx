import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { writeText, readText } from "@tauri-apps/api/clipboard";
import { listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";
import { useRef } from "react";
import deleteIcon from "./assets/delete.svg";
import greenRecord from "./assets/green-record.svg";
import record from "./assets/record.svg";
import reverse from "./assets/reverse.svg";
import pin from "./assets/pin.svg";
import greenPin from "./assets/green-pin.svg";

const SIDE = { TOP: "TOP", BOTTOM: "BOTTOM" };

function App() {
  const [stack, setStack] = useState([]);
  const [stackEnterSide, setStackEnterSide] = useState(SIDE.TOP);
  const [pinnedOnTop, setPinnedOnTop] = useState(false);
  const [preventPop, setPreventPop] = useState(false);
  const [recording, setRecording] = useState(true);
  const unlisten = useRef({ pasteListener: () => {}, copyListener: () => {} });

  useEffect(() => {
    const listenKeypress = async () => {
      unlisten.current.pasteListener = await listen("pasted", async () => {
        if (preventPop) return;
        setStack((p) => {
          if (p.length === 0) return [];
          if (p.length > 1) {
            const { text } = p[1];
            writeText(text);
          }
          return p.slice(1);
        });
      });

      unlisten.current.copyListener = await listen("copied", async () => {
        const clipboardText = await readText();
        if (!clipboardText.length) return;
        const newElement = { id: crypto.randomUUID(), text: clipboardText };
        if (stackEnterSide === SIDE.BOTTOM) setStack((p) => [...p, newElement]);
        else setStack((p) => [newElement, ...p]);
      });
    };
    invoke("start_listen");
    listenKeypress();

    return () => {
      invoke("stop_listen");
      unlisten?.current?.pasteListener();
      unlisten?.current?.copyListener();
    };
  }, []);

  const handlePinUnpin = async () => {
    if (pinnedOnTop) {
      setPinnedOnTop(false);
      await appWindow.setAlwaysOnTop(false);
    } else {
      setPinnedOnTop(true);
      await appWindow.setAlwaysOnTop(true);
    }
  };

  const handleRecordToggle = () => {
    if (recording) {
      setRecording(false);
      invoke("stop_listen");
    } else {
      setRecording(true);
      invoke("start_listen");
    }
  };
  const handleReverse = () => {
    setStack((p) => {
      if (!p.length) return;
      const { text } = p.at(-1);
      writeText(text);
      return [...p].reverse();
    });
  };
  return (
    <>
      <div className="container">
        <button onClick={handleRecordToggle}>
          <img src={recording ? greenRecord : record} />
        </button>
        <button onClick={handlePinUnpin}>
          <img src={pinnedOnTop ? greenPin : pin} />
        </button>
        <button onClick={handleReverse}>
          <img style={{ transform: "rotate(90deg)" }} src={reverse} />
        </button>
        <button onClick={() => setStack([])}>
          <img src={deleteIcon} />
        </button>
      </div>
      {stack?.map((t) => (
        <div className="text" key={t.id}>
          {t.text}
        </div>
      ))}
    </>
  );
}

export default App;
