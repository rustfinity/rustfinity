use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the system's temporary directory.
///
/// # Examples
///
/// ```
/// use tempfile_operations::get_temp_dir;
///
/// let temp_dir = get_temp_dir();
/// assert!(temp_dir.exists());
/// ```
pub fn get_temp_dir() -> PathBuf {
    env::temp_dir()
}

/// Generates a unique filename using prefix, suffix, process ID, and timestamp.
fn generate_unique_name(prefix: &str, suffix: &str) -> String {
    let pid = process::id();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{}_{}_{}_{}", prefix, pid, timestamp, suffix)
}

/// Creates a uniquely named temporary file and returns its path.
/// The filename includes the prefix, process ID, timestamp, and suffix.
///
/// # Examples
///
/// ```
/// use tempfile_operations::create_temp_file;
///
/// let path = create_temp_file("myapp", ".tmp").unwrap();
/// assert!(path.exists());
/// assert!(path.to_string_lossy().contains("myapp"));
/// std::fs::remove_file(&path).unwrap();
/// ```
pub fn create_temp_file(prefix: &str, suffix: &str) -> io::Result<PathBuf> {
    let mut path = get_temp_dir();
    let filename = generate_unique_name(prefix, suffix);
    path.push(filename);
    File::create(&path)?;
    Ok(path)
}

/// Creates a temporary file with the given content and returns its path.
///
/// # Examples
///
/// ```
/// use tempfile_operations::create_temp_file_with_content;
///
/// let path = create_temp_file_with_content("data", ".txt", "Hello, World!").unwrap();
/// let content = std::fs::read_to_string(&path).unwrap();
/// assert_eq!(content, "Hello, World!");
/// std::fs::remove_file(&path).unwrap();
/// ```
pub fn create_temp_file_with_content(prefix: &str, suffix: &str, content: &str) -> io::Result<PathBuf> {
    let mut path = get_temp_dir();
    let filename = generate_unique_name(prefix, suffix);
    path.push(filename);
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(path)
}

/// A temporary file that is automatically deleted when dropped.
///
/// # Examples
///
/// ```
/// use tempfile_operations::TempFile;
///
/// let temp = TempFile::new("auto", ".tmp").unwrap();
/// temp.write("Some data").unwrap();
/// let content = temp.read().unwrap();
/// assert_eq!(content, "Some data");
/// let path = temp.path().to_path_buf();
/// drop(temp);
/// assert!(!path.exists()); // File was deleted
/// ```
pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    /// Creates a new temporary file with a unique name.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile_operations::TempFile;
    ///
    /// let temp = TempFile::new("test", ".tmp").unwrap();
    /// assert!(temp.path().exists());
    /// ```
    pub fn new(prefix: &str, suffix: &str) -> io::Result<Self> {
        let path = create_temp_file(prefix, suffix)?;
        Ok(TempFile { path })
    }

    /// Returns a reference to the file's path.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile_operations::TempFile;
    ///
    /// let temp = TempFile::new("test", ".tmp").unwrap();
    /// assert!(temp.path().to_string_lossy().contains("test"));
    /// ```
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Writes content to the temporary file, overwriting any existing content.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile_operations::TempFile;
    ///
    /// let temp = TempFile::new("test", ".tmp").unwrap();
    /// temp.write("Hello").unwrap();
    /// temp.write("World").unwrap(); // Overwrites previous content
    /// assert_eq!(temp.read().unwrap(), "World");
    /// ```
    pub fn write(&self, content: &str) -> io::Result<()> {
        let mut file = File::create(&self.path)?;
        file.write_all(content.as_bytes())
    }

    /// Reads the entire content of the temporary file as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile_operations::TempFile;
    ///
    /// let temp = TempFile::new("test", ".tmp").unwrap();
    /// temp.write("Hello, World!").unwrap();
    /// assert_eq!(temp.read().unwrap(), "Hello, World!");
    /// ```
    pub fn read(&self) -> io::Result<String> {
        let mut file = File::open(&self.path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // Ignore errors during cleanup - file might already be deleted
        let _ = fs::remove_file(&self.path);
    }
}

/// Creates a uniquely named temporary directory and returns its path.
///
/// # Examples
///
/// ```
/// use tempfile_operations::create_temp_dir;
///
/// let dir = create_temp_dir("workdir").unwrap();
/// assert!(dir.is_dir());
/// std::fs::remove_dir(&dir).unwrap();
/// ```
pub fn create_temp_dir(prefix: &str) -> io::Result<PathBuf> {
    let mut path = get_temp_dir();
    let dirname = generate_unique_name(prefix, "");
    path.push(dirname);
    fs::create_dir(&path)?;
    Ok(path)
}

/// Deletes all files in a directory that start with the given prefix.
/// Returns the count of successfully deleted files.
///
/// # Examples
///
/// ```
/// use tempfile_operations::{get_temp_dir, create_temp_file_with_content, cleanup_temp_files};
///
/// // Create some test files with a unique prefix
/// let prefix = format!("cleanup_test_{}", std::process::id());
/// create_temp_file_with_content(&prefix, ".tmp", "test1").unwrap();
/// create_temp_file_with_content(&prefix, ".tmp", "test2").unwrap();
///
/// let deleted = cleanup_temp_files(&get_temp_dir(), &prefix).unwrap();
/// assert!(deleted >= 2);
/// ```
pub fn cleanup_temp_files(dir: &Path, prefix: &str) -> io::Result<usize> {
    let mut count = 0;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with(prefix) {
                    if fs::remove_file(&path).is_ok() {
                        count += 1;
                    }
                }
            }
        }
    }
    Ok(count)
}

pub fn main() {
    println!("=== Temporary File Operations Demo ===\n");

    // Get system temp directory
    println!("System temp directory: {}", get_temp_dir().display());

    // Create a simple temp file
    println!("\nCreating a temporary file...");
    match create_temp_file("demo", ".tmp") {
        Ok(path) => {
            println!("  Created: {}", path.display());
            let _ = fs::remove_file(&path);
            println!("  Cleaned up manually");
        }
        Err(e) => eprintln!("  Error: {}", e),
    }

    // Create temp file with content
    println!("\nCreating a temp file with content...");
    match create_temp_file_with_content("data", ".txt", "Hello, Temp World!") {
        Ok(path) => {
            let content = fs::read_to_string(&path).unwrap_or_default();
            println!("  Created: {}", path.display());
            println!("  Content: '{}'", content);
            let _ = fs::remove_file(&path);
        }
        Err(e) => eprintln!("  Error: {}", e),
    }

    // Use TempFile for automatic cleanup
    println!("\nUsing TempFile with automatic cleanup...");
    {
        match TempFile::new("auto", ".tmp") {
            Ok(temp) => {
                println!("  Created: {}", temp.path().display());
                let _ = temp.write("Automatic cleanup demo");
                match temp.read() {
                    Ok(content) => println!("  Content: '{}'", content),
                    Err(e) => eprintln!("  Read error: {}", e),
                }
                println!("  (File will be deleted when leaving this scope)");
            }
            Err(e) => eprintln!("  Error: {}", e),
        }
    }
    println!("  TempFile has been dropped and cleaned up!");

    // Create a temp directory
    println!("\nCreating a temporary directory...");
    match create_temp_dir("workdir") {
        Ok(dir) => {
            println!("  Created: {}", dir.display());
            let _ = fs::remove_dir(&dir);
            println!("  Cleaned up");
        }
        Err(e) => eprintln!("  Error: {}", e),
    }

    println!("\n=== Demo Complete ===");
}
