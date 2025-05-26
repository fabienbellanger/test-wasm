import "../public/style.css";
import init, { add, greet, setup } from "../pkg/test-wasm";

async function init_wasm() {
  try {
    await init();

    setup();

    const helloBtn = document.getElementById("hello-btn");
    if (helloBtn) {
      helloBtn.onclick = () => {
        const messageElem = document.getElementById("message");
        if (messageElem) {
          messageElem.textContent = greet("John");
        }
      };
    }

    console.log(add(2, 3)); // Assuming add is a function exported from the WASM module
  } catch (error) {
    console.error("Error loading WASM module:", error);
  }
}

await init_wasm();
