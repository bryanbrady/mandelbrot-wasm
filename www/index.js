// import { memory } from "mandelbrot-wasm/mandelbrot_wasm_bg";
import { Mandelbrot } from "mandelbrot-wasm";

// let start = performance.now();
// let end = performance.now();
const frame_width  = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
const frame_height = window.innerHeight|| document.documentElement.clientHeight|| document.body.clientHeight;
// const W = 1600;
// const H = 900;
const padding = 20;
const W = frame_width - padding;
const H = frame_height - padding;

var mandelbrot = Mandelbrot.new(W, H, -0.75, 0.0);
const width = mandelbrot.width();
const height = mandelbrot.height();
var zoom = mandelbrot.get_zoom();

const canvas = document.getElementById("mandelbrot");
canvas.width = width;
canvas.height = height;
const ctx = canvas.getContext("2d");

canvas.addEventListener("click", function(event) {
  if(event.shiftKey) {
    zoom *= 2;
  } else if(event.ctrlKey) {
    zoom /= 2;
    if(zoom < 100) {
      zoom = 100;
    }
  }
  mandelbrot.set_camera(event.clientX, event.clientY, zoom);
  mandelbrot.draw(ctx);
});

canvas.addEventListener("wheel", function(event) {
  if(event.deltaY < 0) {
    zoom *= 2;
  } else {
    zoom /= 2;
    if(zoom < 100) {
      zoom = 100;
    }
  }
  mandelbrot.set_camera(event.clientX, event.clientY, zoom);
  mandelbrot.draw(ctx);
});


let start = performance.now();
mandelbrot.draw(ctx);
let end = performance.now();
console.log('rendered in ' + (end-start) + ' ms');
