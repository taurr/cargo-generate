use anyhow::{anyhow, Result};
use console::style;
use heck::{KebabCase, SnakeCase};

use crate::emoji;
use crate::interactive;
use crate::Args;

/// Stores user inputted name and provides convenience methods
/// for handling casing.
pub(crate) struct ProjectName {
    pub(crate) user_input: String,
}

impl ProjectName {
    pub fn from_options(args: &Args) -> Result<Self> {
        match args.name {
            Some(ref n) => Ok(ProjectName::new(n)),
            None if !args.silent => Ok(ProjectName::new(interactive::name()?)),
            None => Err(anyhow!(
                "{} {} {}",
                emoji::ERROR,
                style("Project Name Error:").bold().red(),
                style("Option `--silent` provided, but project name was not set. Please use `--project-name`.")
                    .bold()
                    .red(),
            )),
        }
    }

    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self {
            user_input: name.into(),
        }
    }

    pub(crate) fn raw(&self) -> String {
        self.user_input.to_owned()
    }

    pub(crate) fn kebab_case(&self) -> String {
        self.user_input.to_kebab_case()
    }

    pub(crate) fn snake_case(&self) -> String {
        self.user_input.to_snake_case()
    }

    pub(crate) fn is_crate_name(&self) -> bool {
        self.user_input == self.kebab_case()
    }
}
