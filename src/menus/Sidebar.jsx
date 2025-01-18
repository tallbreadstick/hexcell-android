import MenuItem from "../components/MenuItem";
import BurgerMenu from "./../assets/burger_menu.png";
import ConversionIcon from "./../assets/conversion.svg";
import ArithmeticIcon from "./../assets/arithmetic.svg";
import ComplementIcon from "./../assets/complement.svg";
import BatchIcon from "./../assets/batch.svg";
import { onMount, onCleanup, createSignal } from "solid-js";

export function openSidebar() {
    const sidebar = document.getElementById("sidebar");
    sidebar.classList.remove("sidebar-close");
    setOpen(true);
}

export function closeSidebar() {
    const sidebar = document.getElementById("sidebar");
    sidebar.classList.add("sidebar-close");
    setOpen(false);
}

const [isOpen, setOpen] = createSignal(false);

function Sidebar() {

    function handleClickOff(e) {
        const sidebar = document.getElementById("sidebar");
        if (e.target.classList.contains("burger-menu")) return;
        if (!sidebar.contains(e.target) && isOpen()) {
            closeSidebar();
        }
    }

    onMount(() => {
        closeSidebar();
        document.addEventListener("click", handleClickOff);
    });

    onCleanup(() => {
        document.removeEventListener("click", handleClickOff);
    });

    return (
        <div id="sidebar" class="sidebar-close">
            <div class="burger-menu screen-fade-in" onClick={closeSidebar}>
                <img src={BurgerMenu} height="30px" width="30px" />
            </div>
            <MenuItem icon={ConversionIcon} title="Conversion" menu="CONVERSION" />
            <MenuItem icon={ArithmeticIcon} title="Arithmetic" menu="ARITHMETIC" />
            <MenuItem icon={ComplementIcon} title="Complement" menu="COMPLEMENT" />
            <hr />
            <MenuItem icon={BatchIcon} title="Batch Handler" menu="BATCH" />
        </div>
    );

}

export default Sidebar;