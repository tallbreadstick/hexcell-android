import { createSignal, onMount } from "solid-js";
import { openSidebar } from "../menus/Sidebar"; 
import Sidebar from "../menus/Sidebar";
import BurgerMenu from "./../assets/burger_menu.png";
import Conversion from "../menus/Conversion";
import Arithmetic from "../menus/Arithmetic";
import Complement from "../menus/Complement";
import BatchHandler from "../menus/BatchHandler";

const [menu, setMenu] = createSignal("none");

export async function selectMenu(menuTitle) {
    await setMenu(menuTitle);
}

function Main() {

    return (
        <div id="main-screen">
            <Sidebar />
            <div id="nav-bar">
                <div class="burger-menu screen-fade-in" onClick={openSidebar}>
                    <img src={BurgerMenu} height="30px" width="30px" />
                </div>
                <label>HexCell Calculator © 2024</label>
            </div>
            <div id="main-body">
                {() => {
                    switch (menu()) {
                        case "CONVERSION":
                            return <Conversion />;
                        case "ARITHMETIC":
                            return <Arithmetic />;
                        case "COMPLEMENT":
                            return <Complement />;
                        case "BATCH":
                            return <BatchHandler />;
                        default:
                            return;
                    }
                }}
            </div>
        </div>
    );

}

export default Main;