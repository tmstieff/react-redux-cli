use std::path::PathBuf;
use std::string::String;
use std::path::Path;

use std::vec::Vec;
use glob::glob;

pub struct TemplateSearchResult {
    pub files: Vec<PathBuf>,
    pub selected_template_dir: String,
}

// Find any .tpl files in the given directory named the same as the template
pub fn find_templates(template: String, search_paths: &Vec<String>, verbose: bool) -> TemplateSearchResult {
    let mut files: Vec<PathBuf> = Vec::new();
    let mut selected_template_dir = "".to_string();

    for path in search_paths {
        if verbose {
            println!("Looking in {:?} for template {:?}", &path, &template);
        }

        let possible_template_dir = Path::new(&path).join(&template);

        if possible_template_dir.is_dir() {
            let possible_template_dir_path = possible_template_dir.into_os_string().into_string().unwrap();

            if verbose {
                println!("Using directory: {:?}", &possible_template_dir_path);
            }

            selected_template_dir = possible_template_dir_path.clone();

            let mut path = possible_template_dir_path.clone();
            path = path + "/**/*.tpl";

            if verbose {
                println!("Glob all .tpl files in {}", &path);
            }

            for entry in glob(path.as_str()).expect("Error parsing glob pattern for files") {
                match entry {
                    Ok(path) => {
                        println!("Using template file {:?}", &path);
                        files.push(path);
                    },
                    Err(e) => println!("{:?}", e),
                }
            }

            break;
        }
    }

    return TemplateSearchResult {
        files,
        selected_template_dir,
    };
}


#[cfg(test)]
mod tests {
    use super::*;
    use ctrl::generator::write_file;
    use std::env::current_dir;
    use std::fs::create_dir_all;

    #[test]
    fn create_component_type() {
        let path = Path::new(&current_dir().unwrap()).join("templates").join("component");
        create_dir_all(&path).unwrap();
        write_file(&path, "{{class_name}}.tpl".to_string(), &"Test".to_string(), true);

        let current_dir_buf: PathBuf = match current_dir() {
            Ok(path) => path,
            Err(e) => {
                println!("{}", e);
                PathBuf::new()
            }
        };

        let current_dir = match current_dir_buf.into_os_string().into_string() {
            Ok(dir) => dir,
            Err(e) => panic!("Could not find the current working directory, {:?}", e)
        };

        let mut search_paths: Vec<String> = Vec::new();
        search_paths.push(current_dir.clone() + "/templates");

        let result = find_templates("component".to_string(), &search_paths, true);

        assert_eq!(result.selected_template_dir, current_dir.clone() + "/templates/component");
    }
}