const loading = document.getElementById("loading");
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const MAX_THREADS = navigator.hardwareConcurrency || 4;
const WORKERS = [];

let image_data_array = new Uint8ClampedArray();
let frames_rendered = 0;
let workers_returned = MAX_THREADS;

let updateTimer = null;
let dirty = false;

for (let i in Array(MAX_THREADS).fill(0)) {
  let worker = new Worker("public/worker.js", { type: "module" });
  worker.postMessage(
    { msg: "init", width: canvas.width, height: canvas.height, seed: i },
  );

  worker.onmessage = onMessageHandler;
  worker.onerror = onErrorHandler;
  WORKERS.push(worker);
}

function onMessageHandler(e) {
  let data = e.data;
  switch (data.msg) {
    case "init":
      e.target.postMessage({ msg: "render", rays: 1, bounces: 50 });
      workers_returned -= 1;
      break;

    case "render":
      workers_returned += 1;

      document.getElementById("num_frames").innerHTML =
        `Frames: ${frames_rendered}`;
      updateImage(data.data);

      if (frames_rendered >= 200) {
        break;
      } else if (dirty) {
        e.target.postMessage(updateCamera());
        break;
      } else {
        e.target.postMessage({ msg: "render", rays: 2, bounces: 50 });
        workers_returned -= 1;
        break;
      }

    case "set_camera":
      if (workers_returned >= MAX_THREADS) {
        dirty = false;
        image_data_array.fill(0);
        frames_rendered = 0;
        sendToAll({ msg: "render", rays: 2, bounces: 50 });
        workers_returned -= MAX_THREADS;
      }
      break;

    case "pong":
      console.log(data.msg);
      break;
  }
}

function onErrorHandler(e) {
  console.log(e);
}

function updateImage(data) {
  if (frames_rendered == 0) {
    image_data_array = data;
    frames_rendered += 1;
  } else {
    image_data_array = element_average(image_data_array, data);
    frames_rendered += 1;
  }

  let img = new ImageData(image_data_array, canvas.width, canvas.height);
  ctx.putImageData(img, 0, 0);
}

function resetTimeout() {
  if (updateTimer) {
    window.clearTimeout(updateTimer);
  }
  updateTimer = setTimeout(() => {
    dirty = true;
    if (workers_returned >= MAX_THREADS) sendToAll(updateCamera());
  }, 250);
}

function updateCamera() {
  let cameraData = {};
  cameraData.origin_x = document.getElementById("origin_x").value;
  cameraData.origin_y = document.getElementById("origin_y").value;
  cameraData.origin_z = document.getElementById("origin_z").value;

  cameraData.target_x = document.getElementById("target_x").value;
  cameraData.target_y = document.getElementById("target_y").value;
  cameraData.target_z = document.getElementById("target_z").value;

  return { msg: "set_camera", cameraData: cameraData };
}

function sendToAll(msg) {
  for (let worker of WORKERS) {
    worker.postMessage(msg);
  }
}

function element_average(a, b) {
  return a.map((e, i) =>
    (frames_rendered * e + b[i]) /
    (frames_rendered + 1)
  );
}

window.resetTimeout = resetTimeout;
