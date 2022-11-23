use tokio::process::Command;

use crate::execute_command;

use crate::action::{Action, ActionDescription, ActionState};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct CreateSyntheticObjects {
    action_state: ActionState,
}

impl CreateSyntheticObjects {
    #[tracing::instrument(skip_all)]
    pub async fn plan() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Self {
            action_state: ActionState::Uncompleted,
        })
    }
}

#[async_trait::async_trait]
#[typetag::serde(name = "create_synthetic_objects")]
impl Action for CreateSyntheticObjects {
    fn tracing_synopsis(&self) -> String {
        "Create objects defined in `/etc/synthetic.conf`".to_string()
    }

    fn execute_description(&self) -> Vec<ActionDescription> {
        vec![ActionDescription::new(
            self.tracing_synopsis(),
            vec!["Populates the `/nix` path".to_string()],
        )]
    }

    #[tracing::instrument(skip_all, fields())]
    async fn execute(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Yup we literally call both and ignore the error! Reasoning: https://github.com/NixOS/nix/blob/95331cb9c99151cbd790ceb6ddaf49fc1c0da4b3/scripts/create-darwin-volume.sh#L261
        execute_command(
            Command::new("/System/Library/Filesystems/apfs.fs/Contents/Resources/apfs.util")
                .arg("-t")
                .stdin(std::process::Stdio::null()),
        )
        .await
        .ok(); // Deliberate
        execute_command(
            Command::new("/System/Library/Filesystems/apfs.fs/Contents/Resources/apfs.util")
                .arg("-B")
                .stdin(std::process::Stdio::null()),
        )
        .await
        .ok(); // Deliberate

        Ok(())
    }

    fn revert_description(&self) -> Vec<ActionDescription> {
        vec![ActionDescription::new(
            "Refresh the objects defined in `/etc/synthetic.conf`".to_string(),
            vec!["Will remove the `/nix` path".to_string()],
        )]
    }

    #[tracing::instrument(skip_all, fields())]
    async fn revert(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Yup we literally call both and ignore the error! Reasoning: https://github.com/NixOS/nix/blob/95331cb9c99151cbd790ceb6ddaf49fc1c0da4b3/scripts/create-darwin-volume.sh#L261
        execute_command(
            Command::new("/System/Library/Filesystems/apfs.fs/Contents/Resources/apfs.util")
                .arg("-t")
                .stdin(std::process::Stdio::null()),
        )
        .await
        .ok(); // Deliberate
        execute_command(
            Command::new("/System/Library/Filesystems/apfs.fs/Contents/Resources/apfs.util")
                .arg("-B")
                .stdin(std::process::Stdio::null()),
        )
        .await
        .ok(); // Deliberate

        Ok(())
    }

    fn action_state(&self) -> ActionState {
        self.action_state
    }

    fn set_action_state(&mut self, action_state: ActionState) {
        self.action_state = action_state;
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateSyntheticObjectsError {
    #[error("Failed to execute command")]
    Command(#[source] std::io::Error),
}
