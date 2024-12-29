use std::sync::Arc;

use git2;
use octocrab::Octocrab;

pub struct Github {
    handle: Arc<Octocrab>,
}

impl Github {
    pub fn new(username: String, password: String) -> Self {
        let handle = Octocrab::builder()
            .basic_auth(username, password)
            .build()
            .unwrap();
        Self {
            handle: Arc::new(handle),
        }
    }

    pub fn new_token(token: String) -> Self {
        let handle = Octocrab::builder().personal_token(token).build().unwrap();
        Self {
            handle: Arc::new(handle),
        }
    }

    // return (name, git url)
    async fn list_source_repos(&self) -> Vec<(String, String)> {
        let repos = self
            .handle
            .current()
            .list_repos_for_authenticated_user()
            .send()
            .await
            .unwrap();
        let repos = self
            .handle
            .all_pages::<octocrab::models::Repository>(repos)
            .await
            .unwrap();
        repos
            .into_iter()
            .filter(|repo| !repo.fork.unwrap_or(false))
            .map(|repo| {
                (
                    repo.name.clone(),
                    format!("git@github.com:Shylock-Hg/{}.git", repo.name),
                )
            })
            .collect::<Vec<_>>()
    }

    pub async fn sync_github(&self) {
        let repos = self.list_source_repos().await;
        let home = simple_home_dir::home_dir().unwrap();
        let mut dir = home.clone();
        dir.push("Github");
        let _ = tokio::fs::create_dir(&dir).await;
        if let Ok(t) = tokio::fs::try_exists(&dir).await {
            if t {
                use git2::{Cred, RemoteCallbacks};

                let mut ssh_key_path = home.clone();
                ssh_key_path.push(".ssh/id_ed25519");
                // Prepare callbacks.
                let mut callbacks = RemoteCallbacks::new();
                callbacks.credentials(|_url, username_from_url, _allowed_types| {
                    Cred::ssh_key(username_from_url.unwrap(), None, &ssh_key_path, None)
                });

                // Prepare fetch options.
                let mut fo = git2::FetchOptions::new();
                fo.remote_callbacks(callbacks);

                // Prepare builder.
                let mut builder = git2::build::RepoBuilder::new();
                builder.fetch_options(fo);

                for repo in repos {
                    let mut dest = dir.clone();
                    dest.push(repo.0);
                    // Clone the project.
                    let _ = builder.clone(repo.1.as_str(), &dest);
                }
            }
        } else {
            panic!("Can not create dir {:?}", dir);
        }
    }
}
