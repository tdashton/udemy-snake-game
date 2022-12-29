import init, { Direction, GameState, World } from "snake_game";
import { web } from "webpack";

init().then(wasm => {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;

  const snakeSpawnIndex = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);
  const world = World.new(WORLD_WIDTH, snakeSpawnIndex);
  const worldWidth = world.width();

  const gameControlBtn = document.getElementById("game-control-btn");
  const gameStateLabel = document.getElementById("game-state");
  const gamePointsLabel = document.getElementById("game-points");
  const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");
  if (ctx === null) {
    return;
  }

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  gameControlBtn.addEventListener("click", _ => {
    const gameState = world.game_state();

    if (gameState === undefined) {
      gameControlBtn.textContent = "Playing";
      world.start_game();
      play();
    } else {
      location.reload();
    }
  });

  document.addEventListener("keydown", (event) => {
    switch (event.code) {
      case "ArrowUp":
      case "KeyW":
        console.log(event.code);
        world.set_snake_direction(Direction.Up)

        break;
      case "ArrowRight":
      case "KeyD":
        console.log(event.code);
        world.set_snake_direction(Direction.Right)

        break;
      case "ArrowDown":
      case "KeyS":
        console.log(event.code);
        world.set_snake_direction(Direction.Down)

        break;
      case "ArrowLeft":
      case "KeyA":
        console.log(event.code);
        world.set_snake_direction(Direction.Left)

        break;
    }
  });

  function drawWorld() {
    ctx.beginPath();

    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE)
    }

    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y)
    }

    ctx.stroke();
  }

  function drawReward() {
    const idx = world.reward_cell();
    const col = idx % worldWidth;
    const row = Math.floor(idx / worldWidth);

    ctx.beginPath();
    ctx.fillStyle = "#ff0000";
    ctx.fillRect(
      col * CELL_SIZE,
      row * CELL_SIZE,
      CELL_SIZE,
      CELL_SIZE
    );      
    ctx.stroke();
  }

  function drawSnake() {
    const snakeCellPtr = world.snake_cells();
    const snakeLen = world.snake_len();

    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_len()
    );

    snakeCells
      .filter((cellIdx, i) => !(i > 0 && cellIdx === snakeCells[0]))
      .forEach((cellIdx, i) => {
        const col = cellIdx % worldWidth;
        const row = Math.floor(cellIdx / worldWidth);

        // we are overriding the snake color 
        ctx.fillStyle = i === 0 ? "#7878bb" : "#000000"

        ctx.beginPath();
        ctx.fillRect(
          col * CELL_SIZE,
          row * CELL_SIZE,
          CELL_SIZE,
          CELL_SIZE
        );      
      })


    ctx.stroke();
  }

  function drawGameStatus() {
    gameStateLabel.textContent = world.game_state_text();
    gamePointsLabel.textContent = world.points().toString();
  }

  function paint() {
    drawWorld();
    drawSnake();
    drawReward();
    drawGameStatus();
  }

  function play() {
    const state = world.game_state();

    if (state == GameState.Won || state == GameState.Lost) {
      gameControlBtn.textContent = "Play again";
      return;
    }

    console.log("playing");
    const fps = 2;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      // the method takes a callback to invoked before the next repaint
      requestAnimationFrame(play)
    }, 1000 / fps)
  }

  paint();
})