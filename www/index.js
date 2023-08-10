import * as wasm from "conway-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = wasm.Universe.new(64, 64);

const renderLoop = () => {
  pre.textContent = universe.render();
  universe.next_tick();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
