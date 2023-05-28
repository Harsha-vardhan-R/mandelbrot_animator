Rendering Mandelbrot set using Rust and Image crate.

-> Enter a point in the complex plane, and the zoom ratio, it will generate series of photos zooming into the fractal.

Example Image:(Theseimages are completely rendered by the above code.)

resolution => 1000*1000
![Fractal final_resolution-1000 frame-1](https://github.com/Harsha-vardhan-R/mandelbrot_animator/assets/112687561/b050a98f-559b-4ef2-8a43-c414d80515e1)


![Fractal final_resolution-1000 frame-8](https://github.com/Harsha-vardhan-R/mandelbrot_animator/assets/112687561/4f3782d7-4763-4a17-9947-56c0fc479782)

resolution => 3000*3000
![Fractal final_resolution-3000 frame-5](https://github.com/Harsha-vardhan-R/mandelbrot_animator/assets/112687561/65314f67-59e8-41b2-8fed-51f86c19ce25)


![Fractal final_resolution-3000 frame-3](https://github.com/Harsha-vardhan-R/mandelbrot_animator/assets/112687561/03e7d076-7d08-44a4-8153-be758feb373e)


Code uses concurrency, to get better render times.
