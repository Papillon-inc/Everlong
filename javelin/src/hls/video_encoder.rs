use {
    std::path::{
        PathBuf,
        Path,
    },
    std::fs,
    std::io::Write,
    std::process::{
        Command,
        Stdio,
    }
};

pub struct VideoEncoder;

impl VideoEncoder {
    pub fn encode_h264_mpeg2(filepath: &PathBuf) {
        let filename = filepath.file_name().unwrap();
        let current_dir = filepath.as_path().parent().unwrap();
        let tmp_filename = format!("tmp-{}", filename.to_str().unwrap());
        let tmp_filepath = current_dir.join(Path::new(&tmp_filename));

        // let mut child = Command::new("ls")
        //     .current_dir(&current_dir)
        //     .spawn()
        //     .expect("failed to run ls");
        // let _result = child.wait().unwrap();
        
        let mut child = Command::new("ffmpeg")
            .current_dir(&current_dir)
            .arg("-i")
            .arg(Path::new(&filename))
            .arg("-vcodec")
            .arg("mpeg2video")
            .arg(Path::new(&tmp_filename))
            .spawn()
            .expect("failed to encode video");
        let _result = child.wait().unwrap();

        fs::remove_file(filepath.as_path())
            .expect("failed to remove temporary file in video-encoder.");
        fs::rename(tmp_filepath.as_path(),filepath.as_path())
            .expect("failed to rename temporary file in video-encoder.");
    }
}