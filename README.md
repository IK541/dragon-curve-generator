# Dragon Curve generator

Following program is generates a dragon curve fractal with given parameters.

## Use

The program is interactive and asks following parameters:
- number of iterations in fractal generation (the more, the larger and more detailed the resuting picture)
- initial direction (up / right / down / left) - the resulting fractal will be rotated accordlingly (note that each iteration will seemingly rotate the fractal $\frac{\pi}{4}$ rad ($45^\circ$) clockwise if not flipped thus appearance of the fractal will depend not only on initial direction but on the number of iterations too)
- width and height of the resulting picture or '-' if for given dimension (width or height) size of the fractal is used instead of user chosen value (no margins left)
- beginning color of the curve
- end color of the curve (if different than the beginning then a gradient is used)
- option to flip y-axis (resulting in mirror image of the fractal)
- resulting file name (the file will be saved to ```pictures``` directory)
- file extension - available: png, jpg, gif, bmp, tif, ppm

## Compilation

The program can be compiled with ```cargo build``` - the resulting binary will be ```./target/debug/dragon```. Flag ```--release``` can be used for optimized compilation with resulting ```./target/release/dragon```.
