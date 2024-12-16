import { invoke } from "@tauri-apps/api/core";
import { createSignal, from, onCleanup } from "solid-js";
import { getExportData } from "./BatchHandler";

const [tableData, setTableData] = createSignal();

export function handleExportReceived() {
    const data = getExportData();
    const fields = document.querySelectorAll("#conversion input[type='text']");
    console.log(fields);
    fields[0].value = data.input;
    fields[1].value = data.base1;
    fields[2].value = data.output;
    fields[3].value = data.base2;
    processConversion();
}

async function processConversion() {
    const inputs = document.querySelectorAll("#conversion .input-field");
    const bases = document.querySelectorAll("#conversion .base-field");
    await invoke("process_conversion", {
        input: inputs[0].value,
        base1Str: bases[0].value,
        base2Str: bases[1].value
    })
        .then(ok => {
            inputs[1].value = ok;
        })
        .catch(err => {
            console.error(err);
        });
    await processConversionSteps();
    constructConversionTable();
}

async function processConversionSteps() {
    const inputs = document.querySelectorAll("#conversion .input-field");
    const bases = document.querySelectorAll("#conversion .base-field");
    await invoke("process_steps", {
        input: inputs[0].value,
        base1Str: bases[0].value,
        base2Str: bases[1].value
    })
    .then(ok => {
        setTableData(JSON.parse(ok));
    })
    .catch(err => {
        console.error(err);
    });
}

async function constructConversionTable() {
    const toStepsTable = document.querySelector("#to-table tbody");
    const fromStepsTable = document.querySelector("#from-table tbody");
    if (tableData()) {
        const inputs = document.querySelectorAll("#conversion .input-field");
        const bases = document.querySelectorAll("#conversion .base-field");
        if (toStepsTable) {
            toStepsTable.innerHTML = "";
            tableData().to_steps.forEach(step => {
                const row = document.createElement("tr");
                row.innerHTML = `<td>${step.digit} * ${step.base}<sup>${step.place}</sup></td><td>= ${step.res}</td>`;
                toStepsTable.appendChild(row);
            });
            await invoke("process_conversion", {
                input: inputs[0].value,
                base1Str: bases[0].value,
                base2Str: "10"
            })
            .then(ok => {
                const resRow = document.createElement("tr");
                resRow.innerHTML = `<td colspan='2'>Result: ${ok}</td>`;
                toStepsTable.appendChild(resRow);
            })
            .catch(err => {
                console.error(err);
            });
        }
        if (fromStepsTable) {
            fromStepsTable.innerHTML = "";
            tableData().from_steps.forEach(step => {
                const row = document.createElement("tr");
                row.innerHTML = `<td>${step.num} ${step.op} ${step.base}</td><td>= ${step.res}</td><td>r${step.rem}</td>`;
                fromStepsTable.appendChild(row);
            });
            const resRow = document.createElement("tr");
            resRow.innerHTML = `<td colspan='3'>Result: ${inputs[1].value}</td>`;
            fromStepsTable.appendChild(resRow);
        }
    }
}

function Conversion() {

    const [steps, setSteps] = createSignal("TO_STEPS");

    async function swap(field1, field2) {
        const temp = field1.value;
        field1.value = field2.value;
        field2.value = temp;
    }

    async function processSwap() {
        const inputs = document.querySelectorAll("#conversion .input-field");
        const bases = document.querySelectorAll("#conversion .base-field");
        await swap(inputs[0], inputs[1]);
        await swap(bases[0], bases[1]);
        await processConversionSteps();
        constructConversionTable();
    }

    function processClear() {
        const fields = document.querySelectorAll("#conversion input[type='text']");
        const toStepsTable = document.querySelector("#to-table tbody");
        const fromStepsTable = document.querySelector("#from-table tbody");
        fields.forEach(field => {
            field.value = "";
        });
        setTableData(null);
        if (toStepsTable) toStepsTable.innerHTML = "";
        if (fromStepsTable) fromStepsTable.innerHTML = "";
    }

    function clampBase() {
        const bases = document.querySelectorAll("#conversion .base-field");
        bases.forEach(base => {
            invoke("clamp_base", {
                baseStr: base.value
            })
                .then(ok => {
                    base.value = ok;
                })
                .catch(err => {
                    console.error(err);
                });
        });
    }

    function clampInput() {
        const inputs = document.querySelectorAll("#conversion .input-field");
        const bases = document.querySelectorAll("#conversion .base-field");
        invoke("clamp_input", {
            input: inputs[0].value,
            baseStr: bases[0].value
        })
            .then(ok => {
                inputs[0].value = ok;
            })
            .catch(err => {
                console.error(err);
            });
    }

    function validateInput() {
        const inputs = document.querySelectorAll("#conversion .input-field");
        invoke("input_valid", {
            input: inputs[0].value
        })
            .then(ok => {
                if (!ok) {
                    inputs[0].value = "";
                }
            })
            .catch(err => {
                console.error(err);
                inputs[0].value = "";
            });
    }

    function selectTab(id, tab) {
        setSteps(tab);
        const tabButtons = document.querySelectorAll("#conversion .conversion-tab");
        tabButtons.forEach(button => {
            if (button.id !== id) {
                button.classList.remove("active-tab");
            } else {
                button.classList.add("active-tab");
            }
        });
        constructConversionTable();
    }

    onCleanup(() => {
        setTableData(null);
    });

    return (
        <div id="conversion" class="menu-container expand-fill">
            <div class="menu-row">
                <input type="text" class="input-field" autoComplete="off" onBlur={() => { validateInput(); clampInput(); }} />
                <input type="text" class="base-field" inputMode="numeric" autoComplete="off" onBlur={() => { clampBase(); clampInput(); }} />
            </div>
            <div class="menu-row menu-controls">
                <button onClick={processConversion} onTouchEnd={processConversion}>Convert</button>
                <button onClick={processSwap} onTouchEnd={processSwap}>Swap</button>
                <button onClick={processClear} onTouchEnd={processClear}>Clear</button>
            </div>
            <div class="menu-row">
                <input type="text" class="input-field" readOnly />
                <input type="text" class="base-field" inputMode="numeric" autoComplete="off" onBlur={clampBase} />
            </div>
            <hr />
            <div class="conversion-table expand-fill">
                <div id="conversion-table-header">
                    <div id="to-steps" class="conversion-tab active-tab" onClick={() => {selectTab("to-steps", "TO_STEPS")}}>
                        <label>To Decimal Steps</label>
                    </div>
                    <div id="from-steps" class="conversion-tab" onClick={() => {selectTab("from-steps", "FROM_STEPS")}}>
                        <label>From Decimal Steps</label>
                    </div>
                </div>
                <div id="conversion-table-container">
                    {() => {
                        switch (steps()) {
                            case "TO_STEPS":
                                return <table id="to-table" class="conversion-table-body">
                                    <tbody></tbody>
                                </table>;
                            case "FROM_STEPS":
                                return <table id="from-table" class="conversion-table-body">
                                    <tbody></tbody>
                                </table>;
                            default:
                                return;
                        }
                    }}
                </div>
            </div>
        </div>
    );

}

export default Conversion;