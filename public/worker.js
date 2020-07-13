import init, { Image } from "../pkg/raytracer.js";

let wasmMod;
let img;
let width;
let height;
let ptr;
let ptr_len;

let noRender = false;

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

  if (noRender) {
    return;
  }

  postMessage(
    {
      msg: "render",
      data: new Uint8ClampedArray(wasmMod.memory.buffer, ptr, ptr_len),
    },
  );
}

function set_camera(cameraData) {
  noRender = true;

  img.set_camera_origin(
    cameraData.origin_x,
    cameraData.origin_y,
    cameraData.origin_z,
  );
  img.set_camera_target(
    cameraData.target_x,
    cameraData.target_y,
    cameraData.target_z,
  );

  let dx = cameraData.origin_x - cameraData.target_x;
  let dy = cameraData.origin_y - cameraData.target_y;
  let dz = cameraData.origin_z - cameraData.target_z;

  img.set_camera_focus(Math.sqrt(dx * dx + dy * dy + dz * dz));

  postMessage({ msg: "set_camera" });
}

onmessage = async function (e) {
  let data = e.data;
  switch (data.msg) {
    case "init":
      await workerInit(data.width, data.height, data.seed);
      break;
    case "render":
      noRender = false;
      render(data.rays, data.bounces);
      break;
    case "set_camera":
      set_camera(data.cameraData);
      break;
    case "ping":
      postMessage({ msg: "pong" });
  }
};

function getRandomInt(max) {
  return Math.floor(Math.random() * Math.floor(max));
}
