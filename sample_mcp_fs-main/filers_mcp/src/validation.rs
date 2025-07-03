use std::path::{Path, PathBuf};
use std::collections::HashSet;
use std::fmt;

/// Represents a set of allowed root directories for file operations.
#[derive(Debug, Clone)]
pub struct AllowedPaths {
    allowed_roots: HashSet<PathBuf>,
}

#[derive(Debug)]
pub enum AllowedPathsError {
    NotADirectory(PathBuf),
    DoesNotExist(PathBuf),
    NotAllowed(PathBuf),
    TraversalAttempt(PathBuf),
    SymlinkDenied(PathBuf),
}

impl fmt::Display for AllowedPathsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllowedPathsError::NotADirectory(p) => write!(f, "Not a directory: {}", p.display()),
            AllowedPathsError::DoesNotExist(p) => write!(f, "Does not exist: {}", p.display()),
            AllowedPathsError::NotAllowed(p) => write!(f, "Path not allowed: {}", p.display()),
            AllowedPathsError::TraversalAttempt(p) => write!(f, "Traversal attempt denied: {}", p.display()),
            AllowedPathsError::SymlinkDenied(p) => write!(f, "Symlink denied: {}", p.display()),
        }
    }
}

impl std::error::Error for AllowedPathsError {}

impl AllowedPaths {
    /// Create a new AllowedPaths from a list of directory roots.
    pub fn new<P: AsRef<Path>>(roots: Vec<P>) -> Result<Self, AllowedPathsError> {
        let mut allowed_roots = HashSet::new();
        for root in roots {
            let pb = PathBuf::from(root.as_ref());
            if !pb.exists() {
                return Err(AllowedPathsError::DoesNotExist(pb));
            }
            if !pb.is_dir() {
                return Err(AllowedPathsError::NotADirectory(pb));
            }
            let canonical = pb.canonicalize().map_err(|_| AllowedPathsError::DoesNotExist(pb.clone()))?;
            allowed_roots.insert(canonical);
        }
        Ok(Self { allowed_roots })
    }

    /// Returns a reference to the allowed root directories.
    pub fn get_allowed_directories(&self) -> &HashSet<PathBuf> {
        &self.allowed_roots
    }

    /// Validate a path for access. Returns canonicalized path if allowed.
    pub fn validate_path<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf, AllowedPathsError> {
        let input = path.as_ref();
        let canonical = input.canonicalize().map_err(|_| AllowedPathsError::DoesNotExist(input.to_path_buf()))?;
        // Check for symlinks in the path
        for ancestor in canonical.ancestors() {
            if ancestor.read_link().is_ok() {
                tracing::warn!("Symlink denied: {}", ancestor.display());
                return Err(AllowedPathsError::SymlinkDenied(ancestor.to_path_buf()));
            }
        }
        // Check if canonical path is under any allowed root
        let allowed = self.allowed_roots.iter().any(|root| canonical.starts_with(root));
        if !allowed {
            tracing::warn!("Denied access to path: {} (not under allowed roots)", canonical.display());
            return Err(AllowedPathsError::NotAllowed(canonical));
        }
        Ok(canonical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_allowed_paths_new_and_validate() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let file_path = dir.path().join("file.txt");
        fs::write(&file_path, "data").unwrap();
        let validated = allowed.validate_path(&file_path).unwrap();
        assert!(validated.starts_with(dir.path()));
    }

    #[test]
    fn test_denied_outside_root() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let outside = PathBuf::from("/tmp/outside.txt");
        let result = allowed.validate_path(&outside);
        assert!(result.is_err());
    }

    #[test]
    fn test_symlink_denied() {
        let dir = tempdir().unwrap();
        let allowed = AllowedPaths::new(vec![dir.path()]).unwrap();
        let target = dir.path().join("target.txt");
        fs::write(&target, "data").unwrap();
        let link = dir.path().join("link.txt");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&target, &link).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(&target, &link).unwrap();
        let result = allowed.validate_path(&link);
        assert!(result.is_err());
    }
}
