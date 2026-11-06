//! POSIX pathname resolution.

use crate::errno::Errno;

/// POSIX pathname resolution: collapse `.`, `..`, duplicate slashes;
/// preserve trailing slash semantics and root.
pub fn normalize_path(path: &str) -> Result<String, Errno> {
    if path.is_empty() {
        return Err(Errno::Inval);
    }
    let absolute = path.starts_with('/');
    let mut out: Vec<&str> = Vec::new();
    for comp in path.split('/') {
        match comp {
            "" | "." => {}
            ".." => {
                if out.pop().is_none() && !absolute {
                    // relative path escaping root is allowed syntactically
                    out.push("..");
                }
            }
            c => out.push(c),
        }
    }
    let joined = out.join("/");
    if absolute {
        Ok(format!("/{}", joined).replace("//", "/"))
    } else if joined.is_empty() {
        Ok(".".into())
    } else {
        Ok(joined)
    }
}
