use std::path::Path;
use std::path::PathBuf;

/// Return the legacy global Codex home next to a Cradex home, if applicable.
pub(super) fn legacy_home_sibling(cradex_home: &Path) -> Option<PathBuf> {
    cradex_home
        .file_name()
        .filter(|name| *name == ".cradex")
        .and_then(|_| cradex_home.parent())
        .map(|parent| parent.join(".codex"))
}

#[cfg(test)]
mod tests {
    use super::legacy_home_sibling;
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn excludes_sibling_of_cradex_home() {
        assert_eq!(
            legacy_home_sibling(Path::new("/home/jos/.cradex")),
            Some(PathBuf::from("/home/jos/.codex"))
        );
    }

    #[test]
    fn does_not_exclude_projects_for_other_home_names() {
        assert_eq!(legacy_home_sibling(Path::new("/tmp/custom-home")), None);
        assert_eq!(legacy_home_sibling(Path::new("/home/jos/.codex")), None);
    }
}
