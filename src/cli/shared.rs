pub fn resolve_current_latest_version(
    link_path: &std::path::Path,
) -> Option<semver::Version> {
    if !std::fs::exists(link_path).unwrap() {
        return None;
    }
    let version_str = std::fs::read_link(link_path)
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .split("-")
        .nth(1)
        .unwrap()
        .to_string();

    Some(semver::Version::parse(&version_str).unwrap())
}

pub fn resolve_local_versions(
    gql_binaries_path: &std::path::Path,
) -> Vec<semver::Version> {
    let mut versions = std::fs::read_dir(gql_binaries_path)
        .unwrap()
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter(|entry| !entry.file_type().unwrap().is_symlink())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .map(|file_name| file_name.split("-").nth(1).unwrap().to_string())
        .map(|version| semver::Version::parse(&version).unwrap())
        .collect::<Vec<_>>();
    versions.sort_unstable_by(|a, b| b.cmp(a));
    versions
}
