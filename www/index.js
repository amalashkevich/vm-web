import * as wasm from "vm-web";

const submitButton = document.getElementById("submit-code");
const byteCode = document.getElementById("byte-code");

submitButton.addEventListener("click", event => {
  wasm.submit_code(byteCode.value);
});