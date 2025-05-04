use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "@modelcontextprotocol/server-gitlab";
const SERVER_PATH: &str = "node_modules/@modelcontextprotocol/server-gitlab/dist/index.js";

struct GitlabModelContextExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct GitlabContextServerSettings {
    gitlab_personal_access_token: String,
    #[serde(default)]
    gitlab_api_url: Option<String>,
}

impl zed::Extension for GitlabModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(latest_version.as_ref()) {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-gitlab", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `gitlab_personal_access_token` setting".into());
        };
        let settings: GitlabContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string()],
            env: {
                let mut env_vars = vec![(
                    "GITLAB_PERSONAL_ACCESS_TOKEN".into(),
                    settings.gitlab_personal_access_token,
                )];

                if let Some(api_url) = settings.gitlab_api_url {
                    env_vars.push(("GITLAB_API_URL".into(), api_url));
                }

                env_vars
            },
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(GitlabContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(GitlabModelContextExtension);
