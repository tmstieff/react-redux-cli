use std::path::PathBuf;
use std::string::String;
use std::path::Path;
use std::env::current_dir;
use std::fs::create_dir_all;
use std::fs::remove_file;
use std::vec::Vec;
use std::io::prelude::*;
use std::fs::File;
use ansi_term::Colour::*;
use std::env::home_dir;
use glob::glob;

pub struct Generator {
    pub name: String,
    pub verbose: bool,
    pub current_dir: String,
    pub extension: String,
    pub template: String,
    pub output_dir: String,
    pub output_test_dir: String,
    search_paths: Vec<String>,
}

impl Generator {
    pub fn new(command: String, verbose: bool, dir: String, extension: String, template: String,
               output_dir: String, output_test_dir: String) -> Generator {
        let current_dir_buf: PathBuf = match current_dir() {
            Ok(path) => path,
            Err(e) => {
                println!("{}", e);
                PathBuf::new()
            }
        };

        let mut current_dir = match current_dir_buf.into_os_string().into_string() {
            Ok(dir) => dir,
            Err(e) => panic!("Could not find the current working directory, {:?}", e)
        };

        if !dir.eq("") {
            current_dir = dir;
        }

        let home_dir = match home_dir() {
            Some(path) => path.into_os_string().into_string().unwrap(),
            None => "/".to_string(),
        };

        // Setup the default search paths
        // In order:
        // 1. ./templates
        // 2. ~/.recli-templates
        // 3. ~/.config/recli/templates
        let mut search_paths: Vec<String> = Vec::new();
        search_paths.push(current_dir.clone() + "./templates");
        search_paths.push(home_dir.clone() + "/.recli-templates");
        search_paths.push(home_dir.clone() + "/.config/recli/templates");

        Generator {
            name: command,
            verbose,
            current_dir,
            extension,
            template,
            output_dir,
            output_test_dir,
            search_paths,
        }
    }

    pub fn run(&self) {
        let template_files = self.find_templates();
    }

    // Find the templates for a specified component
    fn find_templates(&self) -> Vec<PathBuf> {
        let mut files: Vec<PathBuf> = Vec::new();

        for path in &self.search_paths {
            let possible_template_dir = Path::new(&path).join(Path::new(&self.name));
            if possible_template_dir.is_dir() {
                if self.verbose {
                    print!("Using directory: {:?}", possible_template_dir.clone().into_os_string().into_string().unwrap());
                }

                let mut path = possible_template_dir.into_os_string().into_string().unwrap();
                path = path + "/**/*.tlp";

                for file in glob(path.as_str()).expect("Error parsing glob pattern for files") {
                    files.push(file.unwrap());
                }

                break;
            }
        }

        return files;
    }
}

fn write_file(dir: &Path, file_name: String, content: &String, show_confirmation: bool) {
    match create_dir_all(dir) {
        Ok(ok) => ok,
        Err(e) => panic!("Could not create directories in path {}\n {}", dir.to_str().unwrap(), e)
    };

    let file_path: PathBuf = Path::new(&dir).join(Path::new(&file_name));
    let mut buffer = match File::create(&file_path) {
        Ok(buf) => buf,
        Err(e) => panic!("Error creating file at path {}\n {}", file_path.into_os_string().into_string().unwrap(), e)
    };

    match buffer.write_all(&content.as_bytes()) {
        Ok(res) => res,
        Err(e) => panic!("Error writing to file at path {}\n {}", file_path.into_os_string().into_string().unwrap(), e)
    };

    if show_confirmation {
        println!("File {} written with {} bytes", file_path.into_os_string().into_string().unwrap(), content.len());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_component_type() {
        let generator = Generator::new("TestComponent".to_string(), true, "".to_string(), "jsx".to_string(),
                                       "component".to_string(), "./temp".to_string(), "./temp/test".to_string());

        generator.run();
    }

    #[test]
    fn write_file_valid() {
        write_file(&current_dir().unwrap(), "test_file.tst".to_string(), &"Some content".to_string(), true);

        assert_eq!(Path::new(&current_dir().unwrap()).join("test_file.tst").exists(), true);

        remove_file(Path::new(&current_dir().unwrap()).join("test_file.tst")).expect("Couldn't remove test file");
    }
}