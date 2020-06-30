
# A WASM Raytracer Project

Based on Peter Shirley's [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html). Built with Rust + WASM.

![image](/example.png)

### To build:

1. Clone this repo

2. Build

```bash
wasm-pack build --target web
```

### To run:

1. Install a file server

```bash
deno install --allow-net --allow-read https://deno.land/std/http/file_server.ts
```

2. Run

```bash
file_server .
```