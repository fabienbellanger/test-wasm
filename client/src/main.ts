import init, { add, greet, setup } from "../pkg/test-wasm";

async function init_wasm() {
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

  console.log(add(2, 3));
}

try {
  await init_wasm();
} catch (error) {
  console.error("Error initializing WASM:", error);
}
