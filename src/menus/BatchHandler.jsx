import { createSignal, onCleanup } from "solid-js";
import { createStore } from "solid-js/store";
import { invoke } from "@tauri-apps/api/core";
import { selectMenu } from "../screens/Main";
import { handleExportReceived } from "./Conversion";

const [conversionList, setConversionList] = createStore([]);
const [arithmeticList, setArithmeticList] = createStore([]);
const [complementList, setComplementList] = createStore([]);

const [tempData, setTempData] = createSignal(null);

export function getExportData() {
    return tempData();
}

async function exportConversion(index) {
    setTempData({
        input: conversionList[index].input,
        base1: conversionList[index].base1,
        base2: conversionList[index].base2,
        output: conversionList[index].output
    });
    await selectMenu("CONVERSION");
    handleExportReceived();
}

function BatchHandler() {

    const [batchMenu, setBatchMenu] = createSignal("CONVERSION");

    // long press hell

    const [pressing, setPressing] = createSignal(false);
    const [pressTimer, setPressTimer] = createSignal(null);

    function handlePointerDown(e, index) {
        e.preventDefault();
        setPressing(true);
        setPressTimer(setTimeout(() => {
            if (pressing()) {
                exportConversion(index);
            }
        }, 800));
    }

    function handlePointerUp() {
        setPressing(false);
        clearTimeout(pressTimer());
    }

    function handlePointerCancel() {
        setPressing(false);
        clearTimeout(pressTimer());
    }

    // tab selection

    function selectTab(id, name) {
        const tabButtons = document.querySelectorAll("#batch-header .batch-tab-head");
        switch (batchMenu()) {
            case "CONVERSION":
                saveConversionFields();
                break;
            case "ARITHMETIC":
                saveArithmeticFields();
                break;
            default:
                break;
        }
        setBatchMenu(name);
        tabButtons.forEach(button => {
            if (button.id !== id) {
                button.classList.remove("active-tab");
            } else {
                button.classList.add("active-tab");
            }
        });
    }

    // batch conversion functions

    async function clampConversionBases(index) {
        const rows = document.querySelectorAll("#batch-conversion-list .batch-row");
        const bases = rows[index].querySelectorAll(".base-field");
        bases.forEach(base => {
            invoke("clamp_base", {
                baseStr: base.value
            })
            .then(ok => {
                base.value = ok;
            })
            .catch(err => {
                console.error(err);
            })
        });
    }

    async function clampConversionInput(index) {
        const rows = document.querySelectorAll("#batch-conversion-list .batch-row");
        const fields = rows[index].querySelectorAll("input[type='text']");
        invoke("clamp_input", {
            input: fields[0].value,
            baseStr: fields[1].value
        })
        .then(ok => {
            fields[0].value = ok;
        })
        .catch(err => {
            console.error(err);
        });
    }

    function solveConversionFields() {
        saveConversionFields();
        conversionList.forEach((item, index) => {
            invoke("process_conversion", {
                input: item.input,
                base1Str: item.base1,
                base2Str: item.base2,
            })
            .then(ok => {
                setConversionList(index, "output", ok);
            })
            .catch(err => {
                console.error(err);
            })
        });
    }

    function swapConversionFields() {
        saveConversionFields();
        conversionList.forEach((item, index) => {
            const tempVal = item.input;
            const tempBase = item.base1;
            setConversionList(index, "input", item.output);
            setConversionList(index, "output", tempVal);
            setConversionList(index, "base1", item.base2);
            setConversionList(index, "base2", tempBase);
        });
    }

    function clearConversionFields() {
        saveConversionFields();
        conversionList.forEach((_, index) => {
            setConversionList(index, "input", "");
            setConversionList(index, "base1", "");
            setConversionList(index, "base2", "");
            setConversionList(index, "output", "");
        });
    }
    

    function addConversionRow() {
        setConversionList([...conversionList, { input: "", base1: "", base2: "", output: "" }]);
    }

    function deleteConversionRow(index) {
        setConversionList(conversionList.filter((_, i) => i !== index));
    }

    async function saveConversionFields() {
        const rows = document.querySelectorAll("#batch-conversion-list .batch-row");
        rows.forEach((row, index) => {
            const fields = row.querySelectorAll("input[type='text']");
            setConversionList(index, {
                input: fields[0].value,
                base1: fields[1].value,
                base2: fields[2].value,
                output: fields[3].value
            });
        });
    }

    function deleteConversionFields() {
        setConversionList([]);
    }

    // batch arithmetic functions

    async function clampArithmeticInputs(index) {
        const rows = document.querySelectorAll("#batch-arithmetic-list .batch-row");
        const inputs = rows[index].querySelectorAll(".input-field");
        const base = rows[index].querySelector(".base-field");
        inputs.forEach( async (input) =>  {
            await invoke("clamp_input", {
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

    async function clampArithmeticBases(index) {
        const rows = document.querySelectorAll("#batch-arithmetic-list .batch-row");
        const base = rows[index].querySelector(".base-field");
        await invoke("clamp_base", {
            baseStr: base.value
        })
        .then(ok => {
            base.value = ok;
        })
        .catch(err => {
            console.error(err);
        });
    }

    function solveArithmeticFields() {
        saveArithmeticFields();
        arithmeticList.forEach((item, index) => {
            invoke("process_arithmetic", {
                input1Str: item.input1,
                input2Str: item.input2,
                baseStr: item.base,
                operation: item.op
            })
            .then(ok => {
                setArithmeticList(index, "output", ok);
            })
            .catch(err => {
                console.error(err);
            })
        });
    }

    function clearArithmeticFields() {
        saveArithmeticFields();
        arithmeticList.forEach((_, index) => {
            setArithmeticList(index, "input1", "");
            setArithmeticList(index, "input2", "");
            setArithmeticList(index, "base", "");
            setArithmeticList(index, "op", "");
            setArithmeticList(index, "output", "");
        });
    }

    function addArithmeticRow() {
        setArithmeticList([...arithmeticList, { input1: "", input2: "", base: "", op: "", output: "" }]);
    }

    function deleteArithmeticRow(index) {
        setArithmeticList(arithmeticList.filter((_, i) => i !== index));
    }

    async function saveArithmeticFields() {
        const rows = document.querySelectorAll("#batch-arithmetic-list .batch-row");
        rows.forEach((row, index) => {
            const fields = row.querySelectorAll("input[type='text']");
            const operator = row.querySelector("select");
            setArithmeticList(index, {
                input1: fields[0].value,
                input2: fields[1].value,
                base: fields[2].value,
                op: operator.value,
                output: fields[3].value
            });
        });
    }

    function deleteArithmeticFields() {
        setArithmeticList([]);
    }

    // batch complement functions

    async function clampComplementInputs(index) {
        const rows = document.querySelectorAll("#batch-complement-list .batch-row");
        const inputs = rows[index].querySelectorAll(".input-field");
        inputs.forEach(async (input) => {
            await invoke("clamp_input", {
                input: input.value,
                baseStr: "2"
            })
            .then(ok => {
                input.value = ok;
            })
            .catch(err => {
                console.error(err);
            });
        });
    }

    async function solveComplementFields() {
        await saveComplementFields();
        complementList.forEach(async (item, index) => {
            if (item.binary.trim() !== "") {
                await invoke("process_complement", {
                    binaryStr: item.binary
                })
                .then(ok => {
                    setComplementList(index, "complement", ok);
                })
                .catch(err => {
                    console.error(err);
                });
            }
            if (item.binary.trim() === "") {
                await invoke("process_binary", {
                    complementStr: item.complement
                })
                .then(ok => {
                    setComplementList(index, "binary", ok);
                })
                .catch(err => {
                    console.error(err);
                });
            }
            await processBinaryDecimal(index);
            await processComplementDecimal(index);
        });
    }

    async function processBinaryDecimal(index) {
        const binary = complementList[index].binary;
        if (binary.trim() !== "") {
            await invoke("process_binary_decimal", {
                binaryStr: binary
            })
            .then(ok => {
                setComplementList(index, "binaryDecimal", ok);
            })
            .catch(err => {
                console.error(err);
            });
        }
    }

    async function processComplementDecimal(index) {
        const complement = complementList[index].complement;
        if (complement.trim() !== "") {
            await invoke("process_complement_decimal", {
                complementStr: complement
            })
            .then(ok => {
                setComplementList(index, "complementDecimal", ok);
            })
            .catch(err => {
                console.error(err);
            });
        }
    }

    function clearComplementFields() {
        saveComplementFields();
        complementList.forEach((_, index) => {
            setComplementList(index, "binary", "");
            setComplementList(index, "binaryDecimal", "");
            setComplementList(index, "complement", "");
            setComplementList(index, "complementDecimal", "");
        });
    }

    function addComplementRow() {
        setComplementList([...complementList, { binary: "", binaryDecimal: "", complement: "", complementDecimal: "" }]);
    }

    function deleteComplementRow(index) {
        setComplementList(complementList.filter((_, i) => i !== index));
    }

    async function saveComplementFields() {
        const rows = document.querySelectorAll("#batch-complement-list .batch-row");
        rows.forEach((row, index) => {
            const fields = row.querySelectorAll("input[type='text']");
            setComplementList(index, {
                binary: fields[1].value,
                binaryDecimal: fields[0].value,
                complement: fields[2].value,
                complementDecimal: fields[3].value
            });
        });
    }

    function deleteComplementFields() {
        setComplementList([]);
    }

    // save fields on cleanup to maintain state persistence

    onCleanup(() => {
        switch (batchMenu()) {
            case "CONVERSION":
                saveConversionFields();
                break;
            case "ARITHMETIC":
                saveArithmeticFields();
                break;
            default:
                break;
        }
    });

    return (
        <div id="batch-handler" class="menu-container expand-fill">
            <div id="batch-header">
                <div id="batch-conversion" class="batch-tab-head active-tab" onClick={() => {selectTab("batch-conversion", "CONVERSION")}}>
                    <label>Conversion</label>
                </div>
                <div id="batch-arithmetic" class="batch-tab-head" onClick={() => {selectTab("batch-arithmetic", "ARITHMETIC")}}>
                    <label>Arithmetic</label>
                </div>
                <div id="batch-complement" class="batch-tab-head" onClick={() => {selectTab("batch-complement", "COMPLEMENT")}}>
                    <label>Complement</label>
                </div>
            </div>
            <hr />
            <div id="batch-body">
                {() => {
                    switch (batchMenu()) {
                        case "CONVERSION":
                            return <div id="batch-conversion-list" class="batch-list">
                                <For each={conversionList}>
                                    {(item, index) => (
                                        <div class="batch-row">
                                            <label>{index() + 1}</label>
                                            <input type="text" class="input-field" autoComplete="off" value={item.input} onBlur={() => {clampConversionInput(index())}} />
                                            <input type="text" class="base-field" inputMode="numeric" autoComplete="off" value={item.base1} onBlur={() => {clampConversionBases(index())}} />
                                            <input type="text" class="base-field" inputMode="numeric" autoComplete="off" value={item.base2} onBlur={() => {clampConversionBases(index())}} />
                                            <input type="text" class="input-field export-field" readOnly value={item.output} onPointerDown={(e) => {handlePointerDown(e, index());}} onPointerUp={handlePointerUp} onPointerCancel={handlePointerCancel} />
                                            <button onClick={() => {deleteConversionRow(index())}}>×</button>
                                        </div>
                                    )}
                                </For>
                            </div>;
                        case "ARITHMETIC":
                            return <div id="batch-arithmetic-list" class="batch-list">
                                <For each={arithmeticList}>
                                    {(item, index) => (
                                        <div class="batch-row">
                                            <label>{index() + 1}</label>
                                            <input type="text" class="input-field" autoComplete="off" value={item.input1} onBlur={() => {clampArithmeticInputs(index())}} />
                                            <input type="text" class="input-field" autoComplete="off" value={item.input2} onBlur={() => {clampArithmeticInputs(index())}} />
                                            <input type="text" class="base-field" inputMode="numeric" autoComplete="off" value={item.base} onBlur={() => {clampArithmeticBases(index())}} />
                                            <select>
                                                <option value="add">+</option>
                                                <option value="subtract">-</option>
                                                <option value="multiply">×</option>
                                                <option value="divide">/</option>
                                            </select>
                                            <input type="text" class="input-field" readOnly value={item.output} />
                                            <button onClick={() => {deleteArithmeticRow(index())}}>×</button>
                                        </div>
                                    )}
                                </For>
                            </div>;
                        case "COMPLEMENT":
                            return <div id="batch-complement-list" class="batch-list">
                                <For each={complementList}>
                                    {(item, index) => (
                                        <div class="batch-row">
                                            <label>{index() + 1}</label>
                                            <input type="text" class="context-field" readOnly value={item.binaryDecimal} />
                                            <input type="text" class="input-field" inputMode="numeric" autoComplete="off" value={item.binary} onBlur={() => {clampComplementInputs(index())}} />
                                            <input type="text" class="input-field" inputMode="numeric" autoComplete="off" value={item.complement} onBlur={() => {clampComplementInputs(index())}} />
                                            <input type="text" class="context-field" readOnly value={item.complementDecimal} />
                                            <button onClick={() => {deleteComplementRow(index())}}>×</button>
                                        </div>
                                    )}
                                </For>
                            </div>;
                        default:
                            return;
                    }
                }}
            </div>
            <hr />
            <div id="batch-controls">
                {() => {
                    switch (batchMenu()) {
                        case "CONVERSION":
                            return <div id="conversion-button-panel" class="button-panel">
                                <div class="panel-row">
                                    <button onClick={solveConversionFields}>Convert All</button>
                                    <button onClick={swapConversionFields}>Swap All</button>
                                    <button onClick={clearConversionFields}>Clear All</button>
                                </div>
                                <div class="panel-row">
                                    <button class="add-row" onClick={addConversionRow}>Add Row</button>
                                    <button class="delete-all" onClick={deleteConversionFields}>Delete All</button>
                                </div>
                            </div>;
                        case "ARITHMETIC":
                            return <div class="button-panel">
                                <button onClick={solveArithmeticFields}>Solve All</button>
                                <button onClick={clearArithmeticFields}>Clear All</button>
                                <button class="add-row" onClick={addArithmeticRow}>Add Row</button>
                                <button class="delete-all" onClick={deleteArithmeticFields}>Delete All</button>
                            </div>;
                        case "COMPLEMENT":
                            return <div class="button-panel">
                                <button onClick={solveComplementFields}>Convert All</button>
                                <button onClick={clearComplementFields}>Clear All</button>
                                <button class="add-row"  onClick={addComplementRow}>Add Row</button>
                                <button class="delete-all" onClick={deleteComplementFields}>Delete All</button>
                            </div>;
                        default:
                            return;
                    }
                }}
            </div>
        </div>
    );

}

export default BatchHandler;