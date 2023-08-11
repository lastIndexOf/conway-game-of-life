import * as wasm from "conway-game-of-life";
// 直接获取 wasm 中的内存地址
import { memory } from "conway-game-of-life/conway_game_of_life_bg";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

/** @type {HTMLCanvasElement} */
const canvas = document.getElementById("game-of-life-canvas");
const universe = wasm.Universe.new(64, 64);
const width = universe.width();
const height = universe.height();

canvas.width = CELL_SIZE * width;
canvas.height = CELL_SIZE * height;

const ctx = canvas.getContext("2d");

const drawGrid = () => {
  // 用 canvas 绘制一个 width x height 的网格
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  for (let i = 0; i < width; i++) {
    ctx.moveTo(i * CELL_SIZE, 0);
    ctx.lineTo(i * CELL_SIZE, (height - 1) * CELL_SIZE);
  }

  for (let i = 0; i < width; i++) {
    ctx.moveTo(0, i * CELL_SIZE);
    ctx.lineTo((width - 1) * CELL_SIZE, i * CELL_SIZE);
  }

  ctx.stroke();
};

const drawCell = () => {
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

const refresh = (fn, timer = 16.667) => {
  setTimeout(() => {
    fn();
  }, timer);
};

const renderLoop = () => {
  drawCell();
  drawGrid();

  universe.next_tick();

  refresh(renderLoop, 500);
};

refresh(renderLoop);
