use crate::{ComponentInterface, MergeWith};
use anyhow::{Context, Result};
use askama::Template;
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    class_name: Option<String>,
    cdylib_name: Option<String>,
}
impl Config {
    pub fn class_name(&self) -> &str {
        self.class_name.as_deref().unwrap_or("Uniffi")
    }

    pub fn cdylib_name(&self) -> &str {
        self.cdylib_name.as_deref().unwrap_or("uniffi")
    }
}

impl From<&ComponentInterface> for Config {
    fn from(ci: &ComponentInterface) -> Self {
        Config {
            class_name: None,
            cdylib_name: Some(format!("uniffi_{}", ci.namespace())),
        }
    }
}

impl MergeWith for Config {
    fn merge_with(&self, other: &Self) -> Self {
        Config {
            class_name: self.class_name.merge_with(&other.class_name),
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
    config: Config,
    ci: &'a ComponentInterface,
}

impl<'a> CsharpWrapper<'a> {
    pub fn new(config: Config, ci: &'a ComponentInterface) -> Self {
        CsharpWrapper { config, ci }
    }
}

pub mod filters {
    use super::*;
    use crate::interface::{types, FFIType};

    pub fn optional_ffi_type_name(ffi: Option<&FFIType>) -> Result<String, askama::Error> {
        match ffi {
            Some(ffi) => ffi_type_name(ffi),
            None => Ok("void".into()),
        }
    }

    pub fn ffi_type_name(ffi: &FFIType) -> Result<String, askama::Error> {
        Ok(match ffi {
            FFIType::UInt8 => "byte",
            FFIType::Int8 => "sbyte",
            FFIType::UInt16 => "ushort",
            FFIType::Int16 => "short",
            FFIType::UInt32 => "uint",
            FFIType::Int32 => "int",
            FFIType::UInt64 => "ulong",
            FFIType::Int64 => "long",
            FFIType::Float32 => todo!(),
            FFIType::Float64 => todo!(),
            FFIType::RustArcPtr(_) => todo!(),
            FFIType::RustBuffer => "_UniFFIRustBuffer",
            FFIType::ForeignBytes => "[In, Out] [MarshalAs(UnmanagedType.LPArray)] byte[]",
            FFIType::ForeignCallback => todo!(),
        }
        .to_string())
    }

    pub fn optional_type_name(type_: Option<&types::Type>) -> Result<String, askama::Error> {
        match type_ {
            Some(type_) => type_name(type_),
            None => Ok("void".into()),
        }
    }

    pub fn type_name(type_: &types::Type) -> Result<String, askama::Error> {
        Ok(match type_ {
            types::Type::UInt8 => "byte",
            types::Type::Int8 => "sbyte",
            types::Type::UInt16 => "ushort",
            types::Type::Int16 => "short",
            types::Type::UInt32 => "uint",
            types::Type::Int32 => "int",
            types::Type::UInt64 => "ulong",
            types::Type::Int64 => "long",
            types::Type::Float32 => todo!(),
            types::Type::Float64 => todo!(),
            types::Type::Boolean => "bool",
            types::Type::String => todo!(),
            types::Type::Timestamp => todo!(),
            types::Type::Duration => todo!(),
            types::Type::Object(_) => todo!(),
            types::Type::Record(_) => todo!(),
            types::Type::Enum(_) => todo!(),
            types::Type::Error(_) => todo!(),
            types::Type::CallbackInterface(_) => todo!(),
            types::Type::Optional(_) => todo!(),
            types::Type::Sequence(_) => todo!(),
            types::Type::Map(_, _) => todo!(),
            types::Type::External { .. } => todo!(),
            types::Type::Custom { .. } => todo!(),
        }
        .to_string())
    }

    pub fn optional_lift_function(type_: Option<&types::Type>) -> Result<String, askama::Error> {
        Ok(match type_ {
            Some(type_) => format!(
                "_UniFFIConverter{}.Lift",
                type_.canonical_name().to_upper_camel_case()
            ),
            None => Default::default(),
        })
    }
}
