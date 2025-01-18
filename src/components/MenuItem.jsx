import { selectMenu } from "../screens/Main";
import { closeSidebar } from "../menus/Sidebar";

function MenuItem(props) {

    function closeAndSelect(menu) {
        selectMenu(menu);
        closeSidebar();
    }

    return (
        <div class="menu-item" onClick={() => closeAndSelect(props.menu)}>
            <img src={props.icon} width="30px" height="30px" />
            <label>{props.title}</label>
        </div>
    );

}

export default MenuItem;