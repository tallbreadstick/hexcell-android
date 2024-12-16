import { createSignal, onMount, createEffect } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import "./global.css";
import Splash from "./screens/Splash";
import Main from "./screens/Main";

function App() {

    const [screen, setScreen] = createSignal("SPLASH_SCREEN");

    onMount(() => {
        setTimeout(() => setScreen("MAIN_SCREEN"), 1200);
    });

    return (
        <div id="app" class="expand-fill">
            {() => {
                switch (screen()) {
                    case "SPLASH_SCREEN":
                        return <Splash />
                    case "MAIN_SCREEN":
                        return <Main />
                    default:
                        return <label>Error</label>
                }
            }}
        </div>
    );

}

export default App;
