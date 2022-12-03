General Instructions followed from this tutorial:

https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050

remember to change readonly variables in build_for_pi.sh. Or use
the command:

cargo build --release --target=armv7-unknown-linux-gnueabihf

General instructions as follows:
Install rust from: https://www.rust-lang.org/tools/install
Add rustup target for rpi 3b+/4: 

rustup target add armv7-unknown-linux-gnueabihf

Make sure to install it.

sudo apt install gcc-arm-linux-gnueabihf

If using the bash script make sure to install:

sudo apt-get install sshpass

And to have the readonly variables changed as necessary. WAY easier than c++.

## New Steps:
Follow upen CV github steps. 

Some helpful links:

https://medium.com/analytics-vidhya/the-ultimate-opencv-cross-compilation-guide-for-embedded-processors-f2fdc8ccb7b2

https://github.com/twistedfall/opencv-rust

https://docs.opencv.org/4.x/d0/d76/tutorial_arm_crosscompile_with_cmake.html