import MenuItem from "../components/MenuItem";
import BurgerMenu from "./../assets/burger_menu.png";
import ConversionIcon from "./../assets/conversion.svg";
import ArithmeticIcon from "./../assets/arithmetic.svg";
import ComplementIcon from "./../assets/complement.svg";
import BatchIcon from "./../assets/batch.svg";

export function openSidebar() {
    const sidebar = document.getElementById("sidebar");
    sidebar.classList.remove("sidebar-close");
}

export function closeSidebar() {
    const sidebar = document.getElementById("sidebar");
    sidebar.classList.add("sidebar-close");
}

function Sidebar() {

    return (
        <div id="sidebar" class="sidebar-close">
            <div class="burger-menu screen-fade-in" onClick={closeSidebar} onTouchEnd={closeSidebar}>
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