# SUPER CLI 3D .obj FILES VIEWER
A utility to view low poly obj files from console

[Usage](assets/vid.mp4)

## Build
Build
```shell
cargo build --release
```
Build and run
```shell
cargo run [FILE] [OPTIONS]
```
## Arguments
```shell
-s [SCALE(integer)]  Image scale
-o [X(integer)] [Y(integer)] Sceen offset
-p [X(float)] [Y(float)] [Z(float)] Object position
-d [MILLISECONDS(int)] Delay between frames
-r [X(float)] [Y(float)] [Z(float)] Rotation per frame
-c [CHARACTER] Output characters
```
### Example with default values:
```shell
cli3dviewer [FILE] -s 2 -o 0 0 -p 0 0 3 -d 30 -r 0.02 0.03 0.01 -c \#
```