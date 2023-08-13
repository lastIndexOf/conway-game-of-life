export class FPS {
  constructor(root) {
    this.root = root;
    this.fps = 0;
    this.frames = [];
    this.lastTime = performance.now();
  }

  render() {
    const now = performance.now();
    const delta = now - this.lastTime;

    this.lastTime = now;

    const fps = (1 / delta) * 1000;

    this.fps = fps;
    this.frames.push(fps);

    const max = Math.max(...this.frames);
    const min = Math.min(...this.frames);
    const avg = this.frames.reduce((a, b) => a + b) / this.frames.length;

    this.root.textContent = `FPS: ${fps.toFixed(2)} (min: ${min.toFixed(
      2
    )}, max: ${max.toFixed(2)}, avg: ${avg.toFixed(2)})`.trim();
  }
}
