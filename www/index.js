import * as wasm from "conway-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = wasm.Universe.from_vec(
  [0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0],
  4,
  4
);

const renderLoop = () => {
  pre.textContent = universe.render();
  universe.next_tick();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
