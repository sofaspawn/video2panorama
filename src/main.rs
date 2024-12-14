use std::process::Command;

fn main() {
    let filename = "/home/sofaspawn/progs/video2panorama/ftest.mp4";
    // ffmpeg -i input.mp4 -vf "select=gt(scene\,0.01),setpts=N/FRAME_RATE/TB" -vsync vfr frame_%04d.bmp
    let mut ff = Command::new("ffmpeg").arg("-i").arg(filename);
    let _ = ff.status().unwrap();
    let mut ls = Command::new("ls");
    ls.current_dir("frames/")
        .status()
        .expect("process failed to execute");
}
