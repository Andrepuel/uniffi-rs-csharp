use crate::{ComponentInterface, MergeWith};
use anyhow::{Context, Result};
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    cdylib_name: Option<String>,
}

impl From<&ComponentInterface> for Config {
    fn from(ci: &ComponentInterface) -> Self {
        Config {
            cdylib_name: Some(format!("uniffi_{}", ci.namespace())),
        }
    }
}

impl MergeWith for Config {
    fn merge_with(&self, other: &Self) -> Self {
        Config {
            cdylib_name: self.cdylib_name.merge_with(&other.cdylib_name),
        }
    }
}

pub fn generate_csharp_bindings(config: &Config, ci: &ComponentInterface) -> Result<String> {
    CsharpWrapper::new(config.clone(), ci)
        .render()
        .context("failed to render csharp bindings")
}

#[derive(Template)]
#[template(syntax = "cs", escape = "none", path = "wrapper.cs")]
pub struct CsharpWrapper<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> CsharpWrapper<'a> {
    pub fn new(config: Config, ci: &'a ComponentInterface) -> Self {
        let _config = config;
        let _ci = ci;
        CsharpWrapper {
            _marker: Default::default(),
        }
    }
}
