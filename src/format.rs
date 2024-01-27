use dprint_plugin_json::configuration::{
    Configuration as JsonConfiguration, ConfigurationBuilder as JsonConfigurationBuilder,
};
use dprint_plugin_json::format_text as format_text_json;
use dprint_plugin_markdown::configuration::{
    Configuration as MdConfiguration, ConfigurationBuilder as MdConfigurationBuilder,
};
use dprint_plugin_markdown::format_text as format_text_md;
use dprint_plugin_typescript::configuration::{
    Configuration as TsConfiguration, ConfigurationBuilder as TsConfigurationBuilder,
};
use dprint_plugin_typescript::format_text as format_text_ts;
use std::path::Path;
// use dprint_plugin_css::configuration::{Configuration as CssConfiguration, ConfigurationBuilder as CssConfigurationBuilder};
// use dprint_plugin_css::format_text as format_text_css;
use log::info;
use nginx_config::parse_main;
use serde::Deserialize;

pub enum ExtType {
    TypeScript,
    JavaScript,
    Json,
    Markdown,
    Css,
    Html,
    Nginx,
}

impl ExtType {
    pub fn value_of(&self) -> String {
        match self {
            ExtType::TypeScript => "ts".to_string(),
            ExtType::JavaScript => "js".to_string(),
            ExtType::Json => "json".to_string(),
            ExtType::Markdown => "md".to_string(),
            ExtType::Css => "css".to_string(),
            ExtType::Html => "html".to_string(),
            ExtType::Nginx => "nginx".to_string(),
        }
    }
    pub fn from_str(ext: &str) -> ExtType {
        match ext {
            "ts" => ExtType::TypeScript,
            "js" => ExtType::JavaScript,
            "json" => ExtType::Json,
            "md" => ExtType::Markdown,
            "css" => ExtType::Css,
            "html" => ExtType::Html,
            "nginx" => ExtType::Nginx,
            _ => ExtType::TypeScript,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FormatFormData {
    pub ext: String,
    pub indent2: Option<String>,
    pub content: String,
}

pub fn format_text_by_ext(ext: ExtType, text: &str, indent2: bool) -> Result<String, String> {
    let name = format!("content.{}", ext.value_of());
    let file = Path::new(&name);
    let indent_width = if indent2 { 2 } else { 4 };

    info!("ext: {}, indent_width: {}", ext.value_of(), indent_width);

    match ext {
        ExtType::TypeScript => dprint_format_ts(
            file,
            text,
            &TsConfigurationBuilder::new()
                .indent_width(indent_width)
                .build(),
        ),
        ExtType::Json => dprint_format_json(
            file,
            text,
            &JsonConfigurationBuilder::new()
                .indent_width(indent_width)
                .build(),
        ),
        ExtType::Markdown => dprint_format_md(text, &MdConfigurationBuilder::new().build()),
        ExtType::Css => {
            Err(format!("should not use: {}", ext.value_of()))
            // dprint_format_css(file, text, &CssConfigurationBuilder::new().indent_width(indent_width).build())
        }
        ExtType::Nginx => format_nginx_config(text),
        ExtType::Html => Err(format!("should not use: {}", ext.value_of())),
        _ => Err(format!("unknown extension: {}", ext.value_of())),
    }
}

fn format_nginx_config(text: &str) -> Result<String, String> {
    let result = parse_main(text);

    match result {
        Ok(ret) => Ok(ret.to_string()),
        Err(err) => {
            eprint!("{}", err);
            Err(err.to_string())
        }
    }
}

// fn dprint_format_css(file: &Path, text: &str, config: &CssConfiguration) -> Result<String, String> {
//     let result = format_text_css(file, text, &config);
//     match result {
//         Ok(formatted) => {
//             info!("formatted: {}", formatted.len());
//             Ok(formatted)
//         },
//         Err(err) => {
//             Err(err.to_string())
//         }
//     }
// }

fn dprint_format_md(text: &str, config: &MdConfiguration) -> Result<String, String> {
    let result = format_text_md(text, &config, |_, _, _| Ok(None));
    return process_result(result);
}
fn dprint_format_json(
    file: &Path,
    text: &str,
    config: &JsonConfiguration,
) -> Result<String, String> {
    let result = format_text_json(file, text, &config);
    return process_result(result);
}
fn dprint_format_ts(file: &Path, text: &str, config: &TsConfiguration) -> Result<String, String> {
    let result = format_text_ts(file, text, &config);
    return process_result(result);
}

fn process_result(result: anyhow::Result<Option<String>>) -> Result<String, String> {
    match result {
        Ok(Some(formatted)) => {
            info!("formatted: {}", formatted.len());
            Ok(formatted)
        }
        Ok(None) => Err("Empty content".to_string()),
        Err(err) => Err(err.to_string()),
    }
}
