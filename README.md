# ASICAP-RS
Software to control ZWO ASI cameras remotely

# Build
- ### **Linux & Mac**
    - Install [**libusb-1.0**](https://github.com/libusb/libusb/releases) for ASI Camera SDK

- ### **Raspberry Pi**
    replace `RASPBERRYPI_VER` in .cargo/config.toml with your Raspberry Pi model

# Requires
- ### **Linux & Mac**
    - Run `$ sudo install asi.rules /lib/udev/rules.d` or `$ sudo install asi.rules /etc/udev/rules.d` and reconnect camera, then the camera can be opened without root and run 'cat /sys/module/usbcore/parameters/usbfs_memory_mb' to make sure the result is 200

- ### **Windows**
    - Install [**ZWO ASI cameras driver**](https://astronomy-imaging-camera.com/software-drivers)