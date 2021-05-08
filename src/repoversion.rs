pub struct RepoVersion
{
    international_bot_version : String,
    repo_version : String,
    build_type : String,
    panel_version : String
}

impl RepoVersion
{
    pub fn new() -> Self
    {
        RepoVersion{
            international_bot_version: "".to_string(),
            repo_version: "".to_string(),
            build_type: "".to_string(),
            panel_version: "".to_string()
        }
    }

    pub fn get_international_bot_version(self) -> String {
        return self.international_bot_version.clone();
    }

    pub fn set_international_bot_version(&mut self,international_bot_version: String) {
        self.international_bot_version = international_bot_version;
    }

    pub fn get_repo_version(self) -> String {
        return self.repo_version.clone();
    }

    pub fn set_repo_version(&mut self,repo_version: String) {
        self.repo_version = repo_version;
    }


}