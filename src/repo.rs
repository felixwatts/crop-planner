use crate::genome::Genome;
use std::fs;
use std::error::Error;
use crate::common::*;
use simple_error::*;
use json::object;
use crate::params::Params;
use std::convert::{TryFrom};

#[derive(Debug)]
pub struct Repo {
    path: std::path::PathBuf,
    params_hash: std::string::String,
    solution: Option<Vec<usize>>
}

impl Repo {

    pub fn new(path: &std::path::PathBuf) -> Self {
        let mut repo = Repo {
            path: path.to_path_buf(),
            params_hash: std::string::String::new(),
            solution: None
        };
        repo.path.push(".harvest");
        repo
    }

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

    pub fn reset(&mut self) {
        self.solution = None;
    }

    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        self.require_initialized()?;

        let repo_str = std::fs::read_to_string(self.get_repo_path())?;
        let repo_json = json::parse(&repo_str)?;
        let params_hash = as_string(&repo_json["params_sha1"])?;
        self.params_hash = String::from(params_hash);
        if !repo_json["solution"].is_null() {
            let value_arr = as_array(&repo_json["solution"])?;
            let genes = value_arr.iter().map(|j| as_usize(j)).collect::<Result<Vec<_>, _>>()?;
            self.solution = Some(genes);
        }
        
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        self.require_initialized()?;

        let json = object!{
            "params_sha1" => self.params_hash.clone(),
            "solution" => self.solution.clone(),
        };

        fs::write(self.get_repo_path(), json.dump().as_bytes())?;

        Ok(())
    }

    fn is_params_unchanged(&self) -> Result<bool, Box<dyn Error>> {
        let new_hash = self.get_params_hash()?;
        Ok(new_hash == self.params_hash)
    }

    pub fn put_solution(&mut self, sol: &Genome)-> Result<(), Box<dyn Error>> {
        self.solution = Some(sol.get_genes());
        self.params_hash = self.get_params_hash()?;
        Ok(())
    }

    pub fn require_solution(&self) -> Result<&Vec<usize>, Box<dyn Error>> {
        self.require_initialized()?;
        match &self.solution {
            Some(sol) => {
                match self.is_params_unchanged() {
                    Ok(true) => Ok(&sol),
                    Ok(false) => bail!("The parameters have changed and the solution must be regenerated. Try 'harvest solve'"),
                    Err(e) => Err(e)
                }
            },
            None => bail!("The is no solution. Try 'harvest solve'")
        }
    }

    pub fn require_no_solution(&self) -> Result<(), Box<dyn Error>> {
        self.require_initialized()?;
        match &self.solution {
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
