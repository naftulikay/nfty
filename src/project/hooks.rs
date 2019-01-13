use log::debug;

use std::fs::File;
use std::fs::create_dir;
use std::io;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

static GENERIC_HOOK_PAYLOAD: &'static str = include_str!("lib/generic-hook.sh");

/// A list of types of Git hooks. The only supported hooks for now are client-side hooks, as
/// server-side hooks aren't super useful in this case.
static HOOK_TYPES: &'static [&'static str] = &[
    "applypatch-msg",
    "commit-msg",
    "post-applypatch",
    "post-checkout",
    "post-commit",
    "post-merge",
    "post-rewrite",
    "pre-applypatch",
    "pre-auto-gc",
    "pre-commit",
    "pre-push",
    "pre-rebase",
    "prepare-commit-msg",
];

mod git_lfs {
    pub static POST_CHECKOUT: &'static str = include_str!("lib/git-lfs/post-checkout.sh");
    pub static POST_COMMIT: &'static str = include_str!("lib/git-lfs/post-commit.sh");
    pub static POST_MERGE: &'static str = include_str!("lib/git-lfs/post-merge.sh");
    pub static PRE_PUSH: &'static str = include_str!("lib/git-lfs/pre-push.sh");
}

mod custom {
    pub static BRANCH_CLEAN: &'static str = include_str!("lib/branch-clean.sh");
}

pub fn install(path: &Path) -> io::Result<()> {
    let hooks_dir = path.join(".git").join("hooks");

    // create the hooks directory if it doesnt't exist
    if !hooks_dir.is_dir() {
        create_dir(&path)?;
    }

    for hook in HOOK_TYPES {
        // .git/hooks/post-merge.d
        let hooks_d = hooks_dir.join(format!("{}.d", &hook));

        if !hooks_d.is_dir() {
            // create hooks.d if it doesn't exist
            create_dir(&hooks_d)?;
        }

        // install the generic hook
        write_hook(&hooks_dir.join(&hook), GENERIC_HOOK_PAYLOAD)?;

        // install the custom hooks
        install_custom_hooks(&hook, &hooks_d)?;
    }

    Ok(())
}

#[cfg(unix)]
fn write_hook<T>(path: &Path, payload: T) -> io::Result<()>
        where T: Into<Vec<u8>> {
    let mut output = File::create(&path)?;
    output.write_all(&payload.into())?;
    output.set_permissions(PermissionsExt::from_mode(0o0700))?;
    output.sync_all()?;

    Ok(())
}

fn install_custom_hooks(hook_name: &str, hooks_d: &Path) -> io::Result<()> {
    debug!("Installing custom hooks for {}...", hook_name);

    match hook_name {
        "applypatch-msg"     => install_applypatch_msg(&hooks_d)?,
        "commit-msg"         => install_commit_msg(&hooks_d)?,
        "post-applypatch"    => install_post_applypatch(&hooks_d)?,
        "post-checkout"      => install_post_checkout(&hooks_d)?,
        "post-commit"        => install_post_commit(&hooks_d)?,
        "post-merge"         => install_post_merge(&hooks_d)?,
        "post-rewrite"       => install_post_rewrite(&hooks_d)?,
        "post-update"        => install_post_update(&hooks_d)?,
        "pre-applypatch"     => install_pre_applypatch(&hooks_d)?,
        "pre-auto-gc"        => install_pre_auto_gc(&hooks_d)?,
        "pre-commit"         => install_pre_commit(&hooks_d)?,
        "pre-push"           => install_pre_push(&hooks_d)?,
        "pre-rebase"         => install_pre_rebase(&hooks_d)?,
        "prepare-commit-msg" => install_prepare_commit_msg(&hooks_d)?,
        _ => (),
    }

    Ok(())
}

fn install_applypatch_msg(_hooks_d: &Path) -> io::Result<()> {
    Ok(())
}

fn install_commit_msg(_hooks_d: &Path) -> io::Result<()> {
    Ok(())
}

fn install_post_applypatch(_hooks_d: &Path) -> io::Result<()> {
    Ok(())
}

fn install_post_checkout(hooks_d: &Path) -> io::Result<()> {
    write_hook(&hooks_d.join("10-git-lfs.sh"), git_lfs::POST_CHECKOUT)
}

fn install_post_commit(hooks_d: &Path) -> io::Result<()> {
    write_hook(&hooks_d.join("10-git-lfs.sh"), git_lfs::POST_COMMIT)
}

fn install_post_merge(hooks_d: &Path) -> io::Result<()> {
    write_hook(&hooks_d.join("10-git-lfs.sh"), git_lfs::POST_MERGE)?;
    write_hook(&hooks_d.join("90-branch-clean.sh"), custom::BRANCH_CLEAN)
}

fn install_post_rewrite(_hooks_d: &Path) -> io::Result<()> {
    Ok(())
}

fn install_post_update(_hooks_d: &Path) -> io::Result<()> {
    Ok(())
}

fn install_pre_applypatch(_hooks_d: &Path) -> io::Result<()> {
    Ok(())
}

fn install_pre_auto_gc(_hooks_d: &Path) -> io::Result<()> {
    Ok(())
}

fn install_pre_commit(_hooks_d: &Path) -> io::Result<()> {
    Ok(())
}

fn install_pre_push(hooks_d: &Path) -> io::Result<()> {
    write_hook(&hooks_d.join("10-git-lfs.sh"), git_lfs::PRE_PUSH)
}

fn install_pre_rebase(_hooks_d: &Path) -> io::Result<()> {
    Ok(())
}

fn install_prepare_commit_msg(_hooks_d: &Path) -> io::Result<()> {
    Ok(())
}
