import { invoke } from "@tauri-apps/api/core";

function Complement() {

    async function convertDown() {
        const fields = document.querySelectorAll("#complement input[type='text']");
        if (!fields[0].value) return;
        await invoke("process_complement", {
            binaryStr: fields[0].value
        })
        .then(ok => {
            fields[2].value = ok;
        })
        .catch(err => {
            console.error(err);
        });
        processBinaryDecimal();
        processComplementDecimal();
    }

    async function convertUp() {
        const fields = document.querySelectorAll("#complement input[type='text']");
        if (!fields[2].value) return;
        await invoke("process_binary", {
            complementStr: fields[2].value
        })
        .then(ok => {
            fields[0].value = ok;
        })
        .catch(err => {
            console.error(err);
        });
        processBinaryDecimal();
        processComplementDecimal();
    }

    function processBinaryDecimal() {
        const fields = document.querySelectorAll("#complement input[type='text']");
        invoke("process_binary_decimal", {
            binaryStr: fields[0].value
        })
        .then(ok => {
            fields[1].value = ok;
        })
        .catch(err => {
            console.error(err);
        });
    }

    function processComplementDecimal() {
        const fields = document.querySelectorAll("#complement input[type='text']");
        invoke("process_complement_decimal", {
            complementStr: fields[2].value
        })
        .then(ok => {
            fields[3].value = ok;
        })
        .catch(err => {
            console.error(err);
        });
    }

    function clearAll() {
        const fields = document.querySelectorAll("#complement input[type='text']");
        fields.forEach(field => {
            field.value = "";
        });
    }

    function clampInputs() {
        const fields = document.querySelectorAll("#complement .input-field");
        fields.forEach(async (field) => {
            await invoke("clamp_input", {
                input: field.value,
                baseStr: "2"
            })
            .then(ok => {
                field.value = ok;
            })
            .catch(err => {
                console.error(err);
            });
        });
    }

    return (
        <div id="complement" class="menu-container expand-fill">
            <div class="menu-row">
                <input type="text" class="input-field" autoComplete="false" onBlur={clampInputs} />
                <input type="text" class="context-field" readOnly />
            </div>
            <div class="menu-row">
                <button onClick={convertDown}>Convert ▼</button>
                <button onClick={convertUp}>Convert ▲</button>
                <button onClick={clearAll}>Clear</button>
            </div>
            <div class="menu-row">
                <input type="text" class="input-field" autoComplete="false" onBlur={clampInputs} />
                <input type="text" class="context-field" readOnly />
            </div>
        </div>
    );

}

export default Complement;