use crate::validation::AllowedPaths;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use tracing::{warn, error};

pub struct FileTools<'a> {
    pub allowed_paths: &'a AllowedPaths,
}

impl<'a> FileTools<'a> {
    pub fn read_file(&self, path: &str, head: Option<usize>, tail: Option<usize>) -> Result<String, String> {
        let validated = match self.allowed_paths.validate_path(path) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied read_file access to '{}': {}", path, e);
                return Err(format!("Path validation failed: {}", e));
            }
        };
        let file = match fs::File::open(&validated) {
            Ok(f) => f,
            Err(e) => {
                error!("Failed to open file '{}': {}", validated.display(), e);
                return Err(format!("Failed to open file: {}", e));
            }
        };
        let lines: Vec<String> = io::BufReader::new(file).lines().filter_map(Result::ok).collect();
        let result = if let Some(h) = head {
            lines.iter().take(h).cloned().collect::<Vec<_>>().join("\n")
        } else if let Some(t) = tail {
            lines.iter().rev().take(t).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n")
        } else {
            lines.join("\n")
        };
        Ok(result)
    }

    pub fn write_file(&self, path: &str, content: &str) -> Result<(), String> {
        let validated = match self.allowed_paths.validate_path(path) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied write_file access to '{}': {}", path, e);
                return Err(format!("Path validation failed: {}", e));
            }
        };
        if let Err(e) = fs::write(&validated, content) {
            error!("Failed to write file '{}': {}", validated.display(), e);
            return Err(format!("Failed to write file: {}", e));
        }
        Ok(())
    }

    pub fn list_directory(&self, path: &str) -> Result<Vec<(String, String)>, String> {
        let validated = match self.allowed_paths.validate_path(path) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied list_directory access to '{}': {}", path, e);
                return Err(format!("Path validation failed: {}", e));
            }
        };
        let mut entries = Vec::new();
        let read_dir = match fs::read_dir(&validated) {
            Ok(rd) => rd,
            Err(e) => {
                error!("Failed to read directory '{}': {}", validated.display(), e);
                return Err(format!("Failed to read directory: {}", e));
            }
        };
        for entry in read_dir {
            match entry {
                Ok(e) => {
                    let file_type = e.file_type().map(|ft| {
                        if ft.is_dir() { "dir" } else if ft.is_file() { "file" } else { "other" }
                    }).unwrap_or("unknown");
                    entries.push((e.file_name().to_string_lossy().to_string(), file_type.to_string()));
                },
                Err(e) => {
                    error!("Failed to read entry in directory '{}': {}", validated.display(), e);
                }
            }
        }
        Ok(entries)
    }

    pub fn create_directory(&self, path: &str) -> Result<(), String> {
        let validated = match self.allowed_paths.validate_path(path) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied create_directory access to '{}': {}", path, e);
                return Err(format!("Path validation failed: {}", e));
            }
        };
        if let Err(e) = fs::create_dir_all(&validated) {
            error!("Failed to create directory '{}': {}", validated.display(), e);
            return Err(format!("Failed to create directory: {}", e));
        }
        Ok(())
    }

    pub fn remove_file(&self, path: &str) -> Result<(), String> {
        let validated = match self.allowed_paths.validate_path(path) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied remove_file access to '{}': {}", path, e);
                return Err(format!("Path validation failed: {}", e));
            }
        };
        if let Err(e) = fs::remove_file(&validated) {
            error!("Failed to remove file '{}': {}", validated.display(), e);
            return Err(format!("Failed to remove file: {}", e));
        }
        Ok(())
    }

    pub fn remove_directory(&self, path: &str) -> Result<(), String> {
        let validated = match self.allowed_paths.validate_path(path) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied remove_directory access to '{}': {}", path, e);
                return Err(format!("Path validation failed: {}", e));
            }
        };
        if let Err(e) = fs::remove_dir_all(&validated) {
            error!("Failed to remove directory '{}': {}", validated.display(), e);
            return Err(format!("Failed to remove directory: {}", e));
        }
        Ok(())
    }

    pub fn rename(&self, from: &str, to: &str) -> Result<(), String> {
        let validated_from = match self.allowed_paths.validate_path(from) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied rename-from access to '{}': {}", from, e);
                return Err(format!("Path validation failed (from): {}", e));
            }
        };
        let validated_to = match self.allowed_paths.validate_path(to) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied rename-to access to '{}': {}", to, e);
                return Err(format!("Path validation failed (to): {}", e));
            }
        };
        if let Err(e) = fs::rename(&validated_from, &validated_to) {
            error!("Failed to rename '{}' to '{}': {}", validated_from.display(), validated_to.display(), e);
            return Err(format!("Failed to rename: {}", e));
        }
        Ok(())
    }

    pub fn stat_metadata(&self, path: &str) -> Result<fs::Metadata, String> {
        let validated = match self.allowed_paths.validate_path(path) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied stat_metadata access to '{}': {}", path, e);
                return Err(format!("Path validation failed: {}", e));
            }
        };
        match fs::metadata(&validated) {
            Ok(meta) => Ok(meta),
            Err(e) => {
                error!("Failed to stat '{}': {}", validated.display(), e);
                Err(format!("Failed to stat: {}", e))
            }
        }
    }

    pub fn copy_file(&self, from: &str, to: &str) -> Result<(), String> {
        let validated_from = match self.allowed_paths.validate_path(from) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied copy-from access to '{}': {}", from, e);
                return Err(format!("Path validation failed (from): {}", e));
            }
        };
        let validated_to = match self.allowed_paths.validate_path(to) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied copy-to access to '{}': {}", to, e);
                return Err(format!("Path validation failed (to): {}", e));
            }
        };
        if let Err(e) = fs::copy(&validated_from, &validated_to) {
            error!("Failed to copy '{}' to '{}': {}", validated_from.display(), validated_to.display(), e);
            return Err(format!("Failed to copy: {}", e));
        }
        Ok(())
    }

    pub fn touch_file(&self, path: &str) -> Result<(), String> {
        let validated = match self.allowed_paths.validate_path(path) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied touch_file access to '{}': {}", path, e);
                return Err(format!("Path validation failed: {}", e));
            }
        };
        match fs::OpenOptions::new().create(true).write(true).open(&validated) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to touch file '{}': {}", validated.display(), e);
                Err(format!("Failed to touch file: {}", e))
            }
        }
    }

    pub fn read_symlink(&self, path: &str) -> Result<String, String> {
        let validated = match self.allowed_paths.validate_path(path) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied read_symlink access to '{}': {}", path, e);
                return Err(format!("Path validation failed: {}", e));
            }
        };
        match fs::read_link(&validated) {
            Ok(target) => Ok(target.display().to_string()),
            Err(e) => {
                error!("Failed to read symlink '{}': {}", validated.display(), e);
                Err(format!("Failed to read symlink: {}", e))
            }
        }
    }

    pub fn set_permissions_readonly(&self, path: &str, readonly: bool) -> Result<(), String> {
        let validated = match self.allowed_paths.validate_path(path) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied set_permissions access to '{}': {}", path, e);
                return Err(format!("Path validation failed: {}", e));
            }
        };
        match fs::metadata(&validated) {
            Ok(meta) => {
                let mut perms = meta.permissions();
                perms.set_readonly(readonly);
                if let Err(e) = fs::set_permissions(&validated, perms) {
                    error!("Failed to set permissions for '{}': {}", validated.display(), e);
                    return Err(format!("Failed to set permissions: {}", e));
                }
                Ok(())
            },
            Err(e) => {
                error!("Failed to stat for permissions '{}': {}", validated.display(), e);
                Err(format!("Failed to stat for permissions: {}", e))
            }
        }
    }

    pub fn hard_link(&self, src: &str, dst: &str) -> Result<(), String> {
        let validated_src = match self.allowed_paths.validate_path(src) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied hard_link src access to '{}': {}", src, e);
                return Err(format!("Path validation failed (src): {}", e));
            }
        };
        let validated_dst = match self.allowed_paths.validate_path(dst) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied hard_link dst access to '{}': {}", dst, e);
                return Err(format!("Path validation failed (dst): {}", e));
            }
        };
        if let Err(e) = fs::hard_link(&validated_src, &validated_dst) {
            error!("Failed to create hard link from '{}' to '{}': {}", validated_src.display(), validated_dst.display(), e);
            return Err(format!("Failed to create hard link: {}", e));
        }
        Ok(())
    }

    #[cfg(unix)]
    pub fn create_symlink(&self, src: &str, dst: &str, is_dir: bool) -> Result<(), String> {
        use std::os::unix::fs;
        let validated_src = match self.allowed_paths.validate_path(src) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied create_symlink src access to '{}': {}", src, e);
                return Err(format!("Path validation failed (src): {}", e));
            }
        };
        let validated_dst = match self.allowed_paths.validate_path(dst) {
            Ok(p) => p,
            Err(e) => {
                warn!("Denied create_symlink dst access to '{}': {}", dst, e);
                return Err(format!("Path validation failed (dst): {}", e));
            }
        };
        let result = if is_dir {
            fs::symlink(&validated_src, &validated_dst)
        } else {
            fs::symlink(&validated_src, &validated_dst)
        };
        if let Err(e) = result {
            error!("Failed to create symlink from '{}' to '{}': {}", validated_src.display(), validated_dst.display(), e);
            return Err(format!("Failed to create symlink: {}", e));
        }
        Ok(())
    }

    pub fn file_exists(&self, path: &str) -> bool {
        self.allowed_paths.validate_path(path).map(|p| p.exists()).unwrap_or(false)
    }

    pub fn is_dir(&self, path: &str) -> bool {
        self.allowed_paths.validate_path(path).map(|p| p.is_dir()).unwrap_or(false)
    }

    pub fn is_file(&self, path: &str) -> bool {
        self.allowed_paths.validate_path(path).map(|p| p.is_file()).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::AllowedPaths;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_read_write_list() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let tools = FileTools { allowed_paths: &allowed };
        let file_path = dir.path().join("test.txt");
        // Write
        tools.write_file(file_path.to_str().unwrap(), "hello\nworld").unwrap();
        // Read
        let content = tools.read_file(file_path.to_str().unwrap(), None, None).unwrap();
        assert_eq!(content, "hello\nworld");
        // List
        let entries = tools.list_directory(dir.path().to_str().unwrap()).unwrap();
        assert!(entries.iter().any(|(name, kind)| name == "test.txt" && kind == "file"));
    }

    #[test]
    fn test_create_and_remove_directory() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let tools = FileTools { allowed_paths: &allowed };
        let subdir = dir.path().join("subdir");
        tools.create_directory(subdir.to_str().unwrap()).unwrap();
        assert!(subdir.exists());
        tools.remove_directory(subdir.to_str().unwrap()).unwrap();
        assert!(!subdir.exists());
    }

    #[test]
    fn test_remove_file() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let tools = FileTools { allowed_paths: &allowed };
        let file = dir.path().join("deleteme.txt");
        fs::write(&file, "bye").unwrap();
        tools.remove_file(file.to_str().unwrap()).unwrap();
        assert!(!file.exists());
    }

    #[test]
    fn test_rename_and_stat() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let tools = FileTools { allowed_paths: &allowed };
        let file = dir.path().join("a.txt");
        let file2 = dir.path().join("b.txt");
        fs::write(&file, "data").unwrap();
        tools.rename(file.to_str().unwrap(), file2.to_str().unwrap()).unwrap();
        assert!(!file.exists() && file2.exists());
        let meta = tools.stat_metadata(file2.to_str().unwrap()).unwrap();
        assert!(meta.is_file());
    }

    #[test]
    fn test_copy_file() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let tools = FileTools { allowed_paths: &allowed };
        let file_from = dir.path().join("from.txt");
        let file_to = dir.path().join("to.txt");
        fs::write(&file_from, "copy this").unwrap();
        tools.copy_file(file_from.to_str().unwrap(), file_to.to_str().unwrap()).unwrap();
        assert!(file_to.exists());
        let content = tools.read_file(file_to.to_str().unwrap(), None, None).unwrap();
        assert_eq!(content, "copy this");
    }

    #[test]
    fn test_touch_file() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let tools = FileTools { allowed_paths: &allowed };
        let file = dir.path().join("touch.txt");
        tools.touch_file(file.to_str().unwrap()).unwrap();
        assert!(file.exists());
        let metadata = fs::metadata(&file).unwrap();
        assert!(metadata.is_file());
    }

    #[test]
    fn test_read_symlink() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let tools = FileTools { allowed_paths: &allowed };
        let target = dir.path().join("target.txt");
        let symlink = dir.path().join("link.txt");
        fs::write(&target, "symlink target").unwrap();
        std::os::unix::fs::symlink(&target, &symlink).unwrap();
        let read_target = tools.read_symlink(symlink.to_str().unwrap()).unwrap();
        assert_eq!(read_target, target.display().to_string());
    }

    #[test]
    fn test_copy_and_touch_file() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let tools = FileTools { allowed_paths: &allowed };
        let file1 = dir.path().join("a.txt");
        let file2 = dir.path().join("b.txt");
        tools.write_file(file1.to_str().unwrap(), "copyme").unwrap();
        tools.copy_file(file1.to_str().unwrap(), file2.to_str().unwrap()).unwrap();
        assert!(file2.exists());
        tools.touch_file(file2.to_str().unwrap()).unwrap();
        assert!(file2.exists());
    }

    #[test]
    fn test_set_permissions_readonly() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let tools = FileTools { allowed_paths: &allowed };
        let file = dir.path().join("readonly.txt");
        tools.write_file(file.to_str().unwrap(), "data").unwrap();
        tools.set_permissions_readonly(file.to_str().unwrap(), true).unwrap();
        let meta = fs::metadata(&file).unwrap();
        assert!(meta.permissions().readonly());
        tools.set_permissions_readonly(file.to_str().unwrap(), false).unwrap();
        let meta2 = fs::metadata(&file).unwrap();
        assert!(!meta2.permissions().readonly());
    }

    #[test]
    fn test_hard_link_and_exists() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let tools = FileTools { allowed_paths: &allowed };
        let file = dir.path().join("orig.txt");
        let link = dir.path().join("hardlink.txt");
        tools.write_file(file.to_str().unwrap(), "hl").unwrap();
        tools.hard_link(file.to_str().unwrap(), link.to_str().unwrap()).unwrap();
        assert!(tools.file_exists(link.to_str().unwrap()));
        assert!(tools.is_file(link.to_str().unwrap()));
    }

    #[cfg(unix)]
    #[test]
    fn test_create_symlink_and_is_dir() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let tools = FileTools { allowed_paths: &allowed };
        let target = dir.path().join("targetdir");
        let link = dir.path().join("linkdir");
        fs::create_dir(&target).unwrap();
        tools.create_symlink(target.to_str().unwrap(), link.to_str().unwrap(), true).unwrap();
        assert!(tools.is_dir(link.to_str().unwrap()));
    }
}
