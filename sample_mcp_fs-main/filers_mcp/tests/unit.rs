//! Unit tests for AllowedPaths and FileTools

use crate::validation::AllowedPaths;
use crate::tools::FileTools;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_allowed_paths_exact_and_subdir() {
    let dir = tempdir().unwrap();
    let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
    let subdir = dir.path().join("sub");
    fs::create_dir(&subdir).unwrap();
    let file = subdir.join("file.txt");
    fs::write(&file, "abc").unwrap();
    assert!(allowed.validate_path(&file).is_ok());
}

#[test]
fn test_allowed_paths_denied() {
    let dir = tempdir().unwrap();
    let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
    let outside = std::env::temp_dir().join("not_allowed.txt");
    assert!(allowed.validate_path(&outside).is_err());
}

#[test]
fn test_filetools_read_write_list() {
    let dir = tempdir().unwrap();
    let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
    let tools = FileTools { allowed_paths: &allowed };
    let file = dir.path().join("f.txt");
    tools.write_file(file.to_str().unwrap(), "hi").unwrap();
    let content = tools.read_file(file.to_str().unwrap(), None, None).unwrap();
    assert_eq!(content, "hi");
    let entries = tools.list_directory(dir.path().to_str().unwrap()).unwrap();
    assert!(entries.iter().any(|(n, k)| n == "f.txt" && k == "file"));
}
