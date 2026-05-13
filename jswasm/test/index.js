import init, { add, greet } from "../pkg";

(async () => {
  /* =========================
     1️⃣ 提供 import 给 WASM
     ========================= */

  window.js_on_add_result = (value) => {
    console.log("JS received from WASM:", value);
  };

  /* =========================
     2️⃣ 初始化 WASM
     ========================= */

  await init();

  /* =========================
     3️⃣ 调用 WASM
     ========================= */

  const result = add(3, 5);
  console.log("Rust returned:", result);

  const msg = greet("Alice");
  console.log("Greet result:", msg);
})();
