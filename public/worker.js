import init, { Image } from "../pkg/raytracer.js";

let wasmMod;
let img;
let width;
let height;
let ptr;
let ptr_len;

async function workerInit(iwidth, iheight, seed) {
  wasmMod = await init();

  img = Image.new(iwidth, iheight, seed);

  width = iwidth;
  height = iheight;

  ptr = img.get_image_data_ptr();
  ptr_len = img.get_image_data_len();

  postMessage({ msg: "init" });
}

function render(rays, bounces) {
  img.render(rays, bounces, getRandomInt(9999));

  let imgData = new ImageData(
    new Uint8ClampedArray(wasmMod.memory.buffer, ptr, ptr_len),
    width,
    height,
  );

  postMessage(
    {
      msg: "render",
      data: new Uint8ClampedArray(wasmMod.memory.buffer, ptr, ptr_len),
    },
  );
}

onmessage = async function (e) {
  let data = e.data;
  switch (data.msg) {
    case "init":
      await workerInit(data.width, data.height, data.seed);
      break;
    case "render":
      render(data.rays, data.bounces);
      break;
    case "ping":
      postMessage({ msg: "pong" });
  }
};

function getRandomInt(max) {
  return Math.floor(Math.random() * Math.floor(max));
}
