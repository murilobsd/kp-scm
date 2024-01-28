// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use std::path::Path;

use git2::Repository;
use serde::Deserialize;

pub struct Github {
    owner: String,
}

impl Github {
    pub fn new<S: Into<String>>(o: S) -> Self {
        Self { owner: o.into() }
    }

    pub async fn repos(&self) -> Vec<GithubRepo> {
        let client = reqwest::Client::new();
        client
            .get(format!("https://api.github.com/users/{}/repos?type=owner&sort=updated&direction=desc", self.owner))
            .header("User-Agent", "kp-scm/0.1.0")
            .send()
            .await
            .unwrap()
            .json::<GithubRepos>()
            .await
            .unwrap()
    }
}

pub type GithubRepos = Vec<GithubRepo>;

#[derive(Debug, Deserialize)]
struct Owner {
    login: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubRepo {
    name: String,
    description: Option<String>,
    full_name: String,
    owner: Owner,
    url: String,
    git_url: String,
    language: Option<String>,
}

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    pub fn clone(src: &str, dest: &Path) -> Self {
        let repo = match Repository::clone(src, dest) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };
        Self { repo }
    }

    pub fn dest(&self) -> &Path {
        self.repo.workdir().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn github_repos() {
        let github = Github::new("murilobsd");
        let repos = github.repos().await;
        assert!(repos.len() > 0);
    }

    #[test]
    fn git_clone() {
        let src = "https://github.com/murilobsd/thread-pool";
        let dest = std::env::current_dir().unwrap().join("thread-pool");
        let repo = GitRepo::clone(src, &dest);

        assert_eq!(dest.as_path(), repo.dest());
        assert!(dest.exists());
        assert!(dest.is_dir());

        std::fs::remove_dir_all(&dest).unwrap();
    }
}
