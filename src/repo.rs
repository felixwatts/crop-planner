use crate::plan::Plan;
use std::fs;
use std::error::Error;
use crate::common::*;
use simple_error::*;
use json::object;
use crate::params::Params;
use std::convert::{TryFrom};

// Represents the state of the application, which is stored on disk
#[derive(Debug)]
pub struct Repo {
    path: std::path::PathBuf,
    params_hash: std::string::String,
    plan: Option<Plan>
}

impl Repo {

    pub fn new(path: &std::path::PathBuf) -> Self {
        let mut repo = Repo {
            path: path.to_path_buf(),
            params_hash: std::string::String::new(),
            plan: None
        };
        repo.path.push(".harvest");
        repo
    }

    // Create a new repo in the current directory
    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        if self.is_initialized() {
            bail!("Already initialized");
        }

        fs::create_dir_all(&self.path)?;
        fs::write(self.get_params_path(), crate::params::DEFAULT_PARAMS_JSON)?;
        self.params_hash = self.get_params_hash()?;
        self.save()?;

        Ok(())
    }

    // Create a new repo in the current directory, continuing the plan
    // in the specified directory
    pub fn init_continue(&mut self, from: &str) -> Result<(), Box<dyn Error>> {
        if self.is_initialized() {
            bail!("Already initialized");
        }

        fs::create_dir_all(&self.path)?;

        let mut repo_old = Repo::new(&std::path::PathBuf::from(from));
        repo_old.load()?;

        let params_old_path = repo_old.get_params_path();
        let params_old_str = std::fs::read_to_string(params_old_path)?;
        let mut params_old_json = json::parse(&params_old_str)?;
        let plan_old = repo_old.require_plan()?;
        params_old_json["planting_schedule_prior_year"] = plan_old.to_json();

        fs::write(self.get_params_path(), params_old_json.dump().as_bytes())?;
        self.params_hash = self.get_params_hash()?;
        self.save()?;

        Ok(())
    }

    // Drop the current solution
    pub fn reset(&mut self) {
        self.plan = None;
    }

    // Load application state from the repo in the current directory
    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        self.require_initialized()?;

        let repo_str = std::fs::read_to_string(self.get_repo_path())?;
        let repo_json = json::parse(&repo_str)?;
        let params_hash = as_string(&repo_json["params_sha1"])?;
        self.params_hash = String::from(params_hash);
        if !repo_json["plan"].is_null() {
            let plan = Plan::try_from(&repo_json["plan"])?;
            self.plan = Some(plan);
        }
        
        Ok(())
    }

    // Save application state to the repo in the current directory
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        self.require_initialized()?;

        let json = object!{
            "params_sha1" => self.params_hash.clone(),
            "plan" => match &self.plan { Some(p) => p.to_json(), None => json::Null },
        };

        fs::write(self.get_repo_path(), json.dump().as_bytes())?;

        Ok(())
    }

    pub fn put_solution(&mut self, plan: Plan)-> Result<(), Box<dyn Error>> {
        self.plan = Some(plan);
        self.params_hash = self.get_params_hash()?;
        Ok(())
    }

    pub fn require_plan(&self) -> Result<&Plan, Box<dyn Error>> {
        self.require_initialized()?;
        match &self.plan {
            Some(p) => {
                match self.is_params_unchanged() {
                    Ok(true) => Ok(&p),
                    Ok(false) => bail!("The parameters have changed and the solution must be regenerated. Try 'harvest plan'"),
                    Err(e) => Err(e)
                }
            },
            None => bail!("The is no solution. Try 'harvest plan'")
        }
    }

    pub fn require_no_solution(&self) -> Result<(), Box<dyn Error>> {
        self.require_initialized()?;
        match &self.plan {
            Some(_) => {
                match self.is_params_unchanged() {
                    Ok(true) => bail!("Already solved. Try 'harvest reset'"),
                    Ok(false) => Ok(()),
                    Err(e) => Err(e)
                }
            },
            None => Ok(())
        }
    }

    pub fn get_params(&self) -> Result<Params, Box<dyn Error>> {
        let params_str = std::fs::read_to_string(self.get_params_path())?;
        let params_json = json::parse(&params_str)?;
        let params = Params::try_from(&params_json)?;
        Ok(params)
    }

    fn is_params_unchanged(&self) -> Result<bool, Box<dyn Error>> {
        let new_hash = self.get_params_hash()?;
        Ok(new_hash == self.params_hash)
    }

    fn get_params_path(&self) -> std::path::PathBuf {
        let mut result = self.path.to_path_buf();
        result.push("params.json");
        result
    }

    fn get_repo_path(&self) -> std::path::PathBuf {
        let mut result = self.path.to_path_buf();
        result.push("harvest.json");
        result
    }

    fn is_initialized(&self) -> bool {
        self.path.exists()
    }

    fn get_params_hash(&self) ->  Result<std::string::String, std::io::Error> {
        sha256_digest(&self.get_params_path())
    }

    fn require_initialized(&self) -> Result<(), Box<dyn Error>> {
        if !self.is_initialized() {
            bail!("Not a harvest repo. Try 'harvest init'");
        }

        Ok(())
    }
}

#[cfg(test)]
#[test]
fn repo_init() {
    let mut dir = std::env::temp_dir();
    dir.push(format!("harvest-test-{}", chrono::Utc::now().timestamp()));
    let mut subject = Repo::new(&dir);
    subject.init().expect("init failed");
    subject.init().expect_err("double init");
}
