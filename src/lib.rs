use zed_extension_api as zed;

struct RESTClientExtension {
    // ... state
}

impl RESTClientExtension {
    // TODO
}

impl zed::Extension for RESTClientExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Ok(zed_extension_api::Command {
            command: "my-extension".to_string(),
            args: vec!["--my-extension".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(RESTClientExtension);
