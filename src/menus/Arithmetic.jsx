import { invoke } from "@tauri-apps/api/core";

function Arithmetic() {

    function processArithmetic() {
        const inputs = document.querySelectorAll("#arithmetic .input-field");
        const base = document.querySelector("#arithmetic .base-field");
        const operator = document.querySelector("#arithmetic-operator");
        invoke("process_arithmetic", {
            input1Str: inputs[0].value,
            input2Str: inputs[1].value,
            baseStr: base.value,
            operation: operator.value
        })
            .then(ok => {
                inputs[2].value = ok;
            })
            .catch(err => {
                console.error(err);
            });
    }

    function clampBase() {
        const base = document.querySelector("#arithmetic .base-field");
        invoke("clamp_base", {
            baseStr: base.value
        })
            .then(ok => {
                base.value = ok;
            })
            .catch(err => {
                console.error(err);
            });
    }

    function clampInputs() {
        const inputs = document.querySelectorAll("#arithmetic .input-field");
        const base = document.querySelector("#arithmetic .base-field");
        inputs.forEach(input => {
            invoke("clamp_input", {
                input: input.value,
                baseStr: base.value
            })
            .then(ok => {
                input.value = ok;
            })
            .catch(err => {
                console.error(err);
            });
        });
    }

    function validateInput() {
        const inputs = document.querySelectorAll("#arithmetic .input-field");
        inputs.forEach(input => {
            invoke("input_valid", {
                input: input.value
            })
            .then(ok => {
                if (!ok) {
                    input.value = "";
                }
            })
            .catch(err => {
                console.error(err);
                input.value = "";
            });
        });
    }

    return (
        <div id="arithmetic" class="menu-container expand-fill">
            <div class="menu-row">
                <input type="text" class="base-field" autoComplete="false" onBlur={() => { clampBase(); clampInputs() }} />
                <input type="text" class="input-field" autoComplete="false" onBlur={() => { clampInputs(); validateInput(); }} />
            </div>
            <div class="menu-row">
                <select id="arithmetic-operator">
                    <option value="add">+</option>
                    <option value="subtract">-</option>
                    <option value="multiply">×</option>
                    <option value="divide">÷</option>
                </select>
                <input type="text" class="input-field" autoComplete="false" onBlur={() => { clampInputs(); validateInput(); }} />
            </div>
            <hr />
            <div class="menu-row">
                <button onClick={processArithmetic}>=</button>
                <input type="text" class="input-field" readOnly />
            </div>
        </div>
    );

}

export default Arithmetic;