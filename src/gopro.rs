//
// https://community.gopro.com/s/article/GoPro-Camera-File-Naming-Convention?language=en_US
//
// There are three types of videos:
// 1. Single
// 2. Chaptered
// 3. Looping

// I only care about single and chaptered videos.
// The general format is: GXYYZZZZ.mp4, where:
//                        X is the encoding type (X for HEVC, H for AVC .... yes, I know)
//                        YY is the chapter number
//                        ZZZZ is the video number
//
// Single Video: GH011234.mp4 (first video)
// No more processing is needed here, since it's the first (and only) video.

// Chaptered Video: GH011234.mp4 (first video)
//                  GH021234.mp4 (second video)
//                  ...
// Chaptered videos require concatenation of... all chapters

use std::collections::HashMap;
use std::io::Error;
use std::path::PathBuf;

use log::warn;

/// This struct represents a chaptered GoPro video file (what the camera writes to disk)
#[derive(Debug, Clone)]
pub struct GoProChapteredVideoFile {
    pub abs_path: PathBuf,
    pub video_number: u16,
    pub chapter: u16,
}

// Creates a formatter for user output of a "GoProChapteredVideoFile" 'struct'.
impl std::fmt::Display for GoProChapteredVideoFile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\nabs_path: {}\nvideo_number: {}\nchapter: {}\n",
            self.abs_path.display(),
            self.video_number,
            self.chapter
        )
    }
}

// This function's input called "path", is a 'PathBuf'.
// This function's output is a "GoProChapteredVideoFile" 'struct'.
pub fn parse_gopro_file(path: PathBuf) -> Result<GoProChapteredVideoFile, Error> {
    // Uncomment to print the 'path' 'string'.
    // println!("\n\nParsing file: {:?}", path);
    // Creates a 'string' called "filename" with the file_name() of "path".
    let filename = path.as_path().file_name().unwrap().to_str().unwrap();
    
    // Checks if the 'PathBuf' "Path" is a 'directory'. Return error kind 'InvalidData' with message. 
    if path.is_dir() {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{} is a directory", filename),
        ));
    }
    
    // Creates a 'string' called "extension" with the extension() of "path".
    let extension = path
        .as_path()
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase();
        
    // Creates a 'string' called "prefix" with the first (2) characters of "filename".
    let prefix = filename.get(0..2).unwrap();
    
    // Checks if the string "extension" is 'jpg'. Return error 'InvalidData' with message. 
    if extension == "jpg" && (prefix == "GO" || prefix == "G0") {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            format!("{} is (likely) a GoPro image", filename),
        ));
    }
    
    // Checks if the string "extension" is NOT 'mp4'. Return error 'InvalidData' with message.
    if extension != "mp4" || (prefix != "GH" && prefix != "GX") {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid file extension or prefix: {}", filename),
        ));
    }
    
    // Creates a 16-bit unsigned integer called "video_number" with the "filename" 'string' retrieved from the 4-8 slots of the string. Throw error if it cannot not be created.
    let video_number: u16 = match filename.get(4..8).unwrap().parse() {
        Ok(v) => v,
        Err(e) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Error parsing video number: {filename}
                n{e}"
                ),
            ));
        }
    };
    
    // Creates a 16-bit unsigned integer called "chapter" with the "filename" 'string' retrieved from the 2-4 slots of the string. Throw error if it cannot not be created.
    let chapter: u16 = match filename.get(2..4).unwrap().parse() {
        Ok(v) => v,
        Err(e) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Error parsing chapter number: {filename}\n{e}"),
            ));
        }
    };
    
    // Return Ok() result a "GoProChapteredVideoFile" 'struct'. 
    Ok(GoProChapteredVideoFile {
        abs_path: path.canonicalize().unwrap(),
        video_number,
        chapter,
    })
}


// TODO?: Maybe write parse_gopro_file() / parse_gopro_files_directory() to check and tally the (3) other extensions expected to find (.jpg, .lrv [LowResVideo?], .thm [Thumbnail]) and tally all others as "Unknown". Then just state on 1 line or 2 lines: the unparsed files were found tallied, & ignored. Removed excess lines of output for each file found that wasn't parsed because of a "bad" extension.


// This function's input called "input_files", is a 'vector' of 'PathBufs'.
// This function's output called "videos", is a 'vector' containing "GoProChapteredVideoFile" 'structs'
pub fn parse_gopro_files_directory(input_files: Vec<PathBuf>) -> Vec<GoProChapteredVideoFile> {
    
    // Creates an empty mutable 'vector' of 'GoProChapteredVideoFile' 'structs' called "videos"
    let mut videos: Vec<GoProChapteredVideoFile> = Vec::new();
    
    // Loop through "input_files" for each 'PathBuf', fill the "GoProChapteredVideoFile" 'struct' with function "parse_gopro_file()", and push into the "videos" vector.
    for file in input_files {
        let gopro_file_metadata: GoProChapteredVideoFile = match parse_gopro_file(file) {
            Ok(gopro_file_metadata) => {
                // Uncomment to print the 'struct'.
                // info!("Parsed GoPro Video File: {}", gopro_file_metadata);
                gopro_file_metadata
            }
            // Create warn error. Not fatal so continue.
            Err(e) => {
                warn!("Failed to parse GoPro video file: {}", e);
                continue;
            }
        };
        videos.push(gopro_file_metadata);
    }
    // Return the "videos" 'vector'.
    videos
}

// This function's input called "videos", is a 'vector' of "GoProChapteredVideoFile" 'structs'.
// This function's output called "video_number_to_subvideos_mapping", is a 'HashMap' 
pub fn sort_gopro_files(
    videos: Vec<GoProChapteredVideoFile>,
) -> HashMap<u16, Vec<GoProChapteredVideoFile>> {
    
    // Creates a new() mutable 'HashMap' "video_number_to_subvideos_mapping" with an explicit type signature <'integer', 'vector'>.
    let mut video_number_to_subvideos_mapping: HashMap<u16, Vec<GoProChapteredVideoFile>> =
        HashMap::new();
    
    // Loop through "videos" for each "GoProChapteredVideoFile" 'struct', and push them in a 'HashMap'.
    for video in videos {
        video_number_to_subvideos_mapping
            .entry(video.video_number)
            .or_insert(Vec::new()) // If "video_number" doesn't exist insert a new 'vector'.
            .push(video); // Push video in the 'vector'.
    }

    // Sort chapters within each video group by chapter number
    for chapters in video_number_to_subvideos_mapping.values_mut() {
        chapters.sort_by_key(|c| c.chapter);
    }
    // Return the "video_number_to_subvideos_mapping" 'HashMap'.
    video_number_to_subvideos_mapping
}

// Assumes output_dir is a normalized directory path. Adds GoPro_{}.EXTENSION to the end of the path.
pub fn gen_output_path(output_dir: &PathBuf, video_number: u16, extension: &str) -> PathBuf {
    let mut output_path = PathBuf::from(output_dir);
    output_path.push(format!("GoPro_{}", video_number));
    output_path.set_extension(extension);
    output_path
}
