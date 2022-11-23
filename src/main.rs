mod asi_camera2;

fn main() {
    unsafe { asi_setup() };
}

unsafe fn asi_setup() {
    let connected_cameras = asi_camera2::ASIGetNumOfConnectedCameras();
    println!("{} cameras connected.", connected_cameras);
}
