mkdir frames
mkdir out_video
./burning_ship_fractal $1
cd frames
ffmpeg -framerate 30 -pattern_type glob -i '*.png'   -c:v libx264 -pix_fmt yuv420p -y ../out_video/out.mp4