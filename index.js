import init, {Image} from './pkg/wasm.js';

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

async function main() {
    let wasmMod = await init();

    let img = Image.new(canvas.width, canvas.height);
    // let memory = new Uint8ClampedArray(wasmMod.memory.buffer);

    let ptr = img.get_image_data_ptr();
    let ptr_len = img.get_image_data_len();

    img.render();

    let imgData = new ImageData(
        new Uint8ClampedArray(wasmMod.memory.buffer, ptr, ptr_len),
        canvas.width,
        canvas.height
    );

    ctx.putImageData(imgData, 0, 0);

}
main();



