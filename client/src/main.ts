import "./assets/css/style.css";
import init, { add, greet, setup } from "../pkg/test-wasm";

async function init_wasm() {
  await init();

  setup();

  const btn = document.getElementById("msg-btn");
  const input = document.getElementById("msg-input");
  const messageElem = document.getElementById("msg");
  if (btn && input && messageElem) {
    btn.onclick = () => {
      messageElem.textContent = greet((input as HTMLInputElement).value);
    };
  }

  // Test the add function
  console.log(add(2, 3));
}

(async () => {
  try {
    await init_wasm();
  } catch (error) {
    console.error("Error initializing WASM:", error);
  }
})();
