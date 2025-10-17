use git2::{Repository, DiffOptions, Commit, Signature, Diff};
use std::path::Path;

pub fn read_file_from_repo(repo_path: &str, file_path: &str) -> Result<String, String> {
    let repo = Repository::open(repo_path).map_err(|e| e.to_string())?;
    let head = repo.head().map_err(|e| e.to_string())?;
    let commit = head.peel_to_commit().map_err(|e| e.to_string())?;
    let tree = commit.tree().map_err(|e| e.to_string())?;
    let entry = tree.get_path(Path::new(file_path)).map_err(|e| e.to_string())?;
    let object = entry.to_object(&repo).map_err(|e| e.to_string())?;
    let blob = object.as_blob().ok_or("Could not find blob".to_string())?;
    String::from_utf8(blob.content().to_vec()).map_err(|e| e.to_string())
}

pub fn apply_patch(repo_path: &str, patch_str: &str) -> Result<(), String> {
    let repo = Repository::open(repo_path).map_err(|e| e.to_string())?;
    let diff = Diff::from_buffer(patch_str.as_bytes()).map_err(|e| e.to_string())?;
    let mut apply_opts = git2::ApplyOptions::new();
    repo.apply(&diff, git2::ApplyLocation::WorkDir, Some(&mut apply_opts)).map_err(|e| e.to_string())
}

pub fn get_diff(repo_path: &str) -> Result<String, String> {
    let repo = Repository::open(repo_path).map_err(|e| e.to_string())?;
    let head = repo.head().map_err(|e| e.to_string())?;
    let tree = head.peel_to_tree().map_err(|e| e.to_string())?;
    let mut opts = DiffOptions::new();
    let diff = repo.diff_tree_to_workdir_with_index(Some(&tree), Some(&mut opts)).map_err(|e| e.to_string())?;
    let mut patch_str = String::new();
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        patch_str.push(line.origin());
        patch_str.push_str(std::str::from_utf8(line.content()).unwrap_or(""));
        true
    }).map_err(|e| e.to_string())?;
    Ok(patch_str)
}

pub fn create_branch_and_commit(repo_path: &str, branch_name: &str, commit_message: &str) -> Result<(), String> {
    let repo = Repository::open(repo_path).map_err(|e| e.to_string())?;
    let head = repo.head().map_err(|e| e.to_string())?;
    let head_commit = head.peel_to_commit().map_err(|e| e.to_string())?;

    // Create a new branch
    repo.branch(branch_name, &head_commit, false).map_err(|e| e.to_string())?;

    // Checkout the new branch
    let obj = repo.revparse_single(&("refs/heads/".to_owned() + branch_name)).map_err(|e| e.to_string())?;
    repo.checkout_tree(&obj, None).map_err(|e| e.to_string())?;
    repo.set_head(&("refs/heads/".to_owned() + branch_name)).map_err(|e| e.to_string())?;

    // Commit changes
    let mut index = repo.index().map_err(|e| e.to_string())?;
    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None).map_err(|e| e.to_string())?;
    index.write().map_err(|e| e.to_string())?;
    let tree_id = index.write_tree().map_err(|e| e.to_string())?;
    let tree = repo.find_tree(tree_id).map_err(|e| e.to_string())?;

    let signature = repo.signature().map_err(|e| {
        if e.code() == git2::ErrorCode::NotFound {
            "Git signature not found. Please configure your git user name and email.".to_string()
        } else {
            e.to_string()
        }
    })?;

    repo.commit(Some("HEAD"), &signature, &signature, commit_message, &tree, &[&head_commit]).map_err(|e| e.to_string())?;

    Ok(())
}