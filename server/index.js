//import * as wasm from "wasm-game-of-life";
//wasm.hello();

// Import the WebAssembly memory at the top of the file.
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

import { Game, index } from "wasm-game-of-life";

const width = 200;
const height = 200;
const CELL_SIZE = 5
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const game = Game.new(width, height, 17);
game.semi_rand(19)

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
console.log(`Canvas size: ${game.width()}, ${game.height()}`)

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
  
    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
  
    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
    ctx.stroke();
};
const drawCells = () => {
    const cellsPtr = game.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  
    ctx.beginPath();
  
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const _idx = index(row, col, width);
  
        ctx.fillStyle = cells[_idx] === 0
          ? DEAD_COLOR
          : ALIVE_COLOR;
  
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }
    ctx.stroke();
};

const ctx = canvas.getContext('2d');
const renderLoop = () => {
    game.tick();
  
    drawGrid();
    drawCells();
    setTimeout(function() {
        requestAnimationFrame(renderLoop);
 
        // ... Code for Drawing the Frame ...
 
    }, 1000 / 60);
};
requestAnimationFrame(renderLoop)
