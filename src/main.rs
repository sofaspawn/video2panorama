use std::process::Command;

fn main() {
    let filename = "/home/sofaspawn/progs/video2panorama/ftest.mp4";
    // ffmpeg -i input.mp4 -vf "select=gt(scene\,0.01),setpts=N/FRAME_RATE/TB" -vsync vfr frame%01d.bmp
    // let mut ff = Command::new("ffmpeg").arg("-i").arg(filename);
    // let _ = ff.status().unwrap();

    Command::new("ffmpeg")
        .arg("-i")
        .arg(filename)
        .arg("-vf")
        .arg("select=gt(scene\\,0.01),setpts=N/FRAME_RATE/TB")
        .arg("-vsync")
        .arg("vfr")
        .arg("frames/frame%01d.bmp")
        .status()
        .expect("process failed to execute");

    /*
    let mut ls = Command::new("ls");
    ls.current_dir("frames/")
        .status()
        .expect("process failed to execute");

    ls.status().unwrap();
    */
}
