import * as wasm from "conway-game-of-life";
// 直接获取 wasm 中的内存地址
import { memory } from "conway-game-of-life/conway_game_of_life_bg";
import { FPS } from "./fps";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const fps = new FPS(document.querySelector("#fps"));
/** @type {HTMLCanvasElement} */
const canvas = document.getElementById("game-of-life-canvas");
const universe = wasm.Universe.new(256, 256);
const width = universe.width();
const height = universe.height();

canvas.width = CELL_SIZE * width;
canvas.height = CELL_SIZE * height;

const ctx = canvas.getContext("2d");

const drawGrid = () => {
  // 用 canvas 绘制一个 width x height 的网格
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  for (let i = 0; i <= height; i++) {
    ctx.moveTo(i * CELL_SIZE, 0);
    ctx.lineTo(i * CELL_SIZE, height * CELL_SIZE);
  }

  for (let i = 0; i <= width; i++) {
    ctx.moveTo(0, i * CELL_SIZE);
    ctx.lineTo(width * CELL_SIZE, i * CELL_SIZE);
  }

  ctx.stroke();
};

const drawCells = () => {
  ctx.beginPath();

  for (let y = 0; y < height; y++) {
    const cells = new Uint8Array(memory.buffer, universe.cells(y), width);

    for (let x = 0; x < height; x++) {
      switch (cells[x]) {
        case wasm.Cell.Alive:
          ctx.fillStyle = ALIVE_COLOR;
          break;
        case wasm.Cell.Dead:
        default:
          ctx.fillStyle = DEAD_COLOR;
          break;
      }

      ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    }
  }

  ctx.stroke();
};

let isPaused = false;

const playOrPause = () => {
  if (!isPaused) {
    isPaused = true;
  } else {
    isPaused = false;
    refresh(renderLoop);
  }
};

const onCanvasClick = (event) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const y = Math.floor(canvasTop / CELL_SIZE);
  const x = Math.floor(canvasLeft / CELL_SIZE);

  universe.toggle_cell(y, x);

  drawCells();
  drawGrid();
};

document.querySelector("#play-pause").addEventListener("click", playOrPause);
canvas.addEventListener("click", onCanvasClick);

const refresh = (fn, timer = 16.667) => {
  setTimeout(() => {
    fn();
  }, timer);
};

const renderLoop = () => {
  fps.render();

  for (let i = 0; i < 10; i++) {
    universe.next_tick();
  }

  drawCells();
  drawGrid();

  if (!isPaused) {
    refresh(renderLoop, 8.6);
  }
};

refresh(renderLoop);
