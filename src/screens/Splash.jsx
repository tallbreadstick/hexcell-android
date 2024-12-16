import Logo from "./../assets/hexcell-tauri.png";

function Splash() {

    return (
        <div id="splash-screen" class="screen-fade-in expand-fill">
            <div id="splash-logo">
                <img src={Logo} height="120px" width="120px" />
                <label>HexCell Calculator</label>
            </div>
        </div>
    );

}

export default Splash;