const loading = document.getElementById("loading");
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const MAX_THREADS = navigator.hardwareConcurrency || 4;
const WORKERS = [];

let image_data_array = new Uint8ClampedArray();
let workers_returned = 0;

for (let i in Array(MAX_THREADS).fill(0)) {
  let worker = new Worker("public/worker.js", { type: "module" });
  worker.postMessage(
    { msg: "init", width: canvas.width, height: canvas.height, seed: i },
  );

  worker.onmessage = onMessageHandler;
  worker.onerror = onErrorHandler;
}

function onMessageHandler(e) {
  let data = e.data;
  switch (data.msg) {
    case "init":
      e.target.postMessage({ msg: "render", rays: 1, bounces: 50 });
      break;
    case "render":
      updateImage(data.data);
      if (workers_returned > 150) {
        break;
      } else {
        e.target.postMessage({ msg: "render", rays: 5, bounces: 50 });
        break;
      }
    case "pong":
      console.log(data.msg);
      break;
  }
}

function onErrorHandler(e) {
  console.log(e);
}

function updateImage(data) {
  if (workers_returned == 0) {
    image_data_array = data;
    workers_returned += 1;
  } else {
    image_data_array = element_average(image_data_array, data);
    workers_returned += 1;
  }

  console.log(workers_returned);
  let img = new ImageData(image_data_array, canvas.width, canvas.height);
  ctx.putImageData(img, 0, 0);
}

function element_average(a, b) {
  return a.map((e, i) =>
    (workers_returned * e + b[i]) /
    (workers_returned + 1)
  );
}

// myWorker.onmessage = function (e) {
//   console.log(e);
//   let imgData = new ImageData(
//     e.data,
//     canvas.width,
//     canvas.height,
//   );
//   ctx.putImageData(imgData, 0, 0);
// };
