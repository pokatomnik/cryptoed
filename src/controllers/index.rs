use std::path::{Path, PathBuf};

use clap::Args;

use crate::{
    controllers::controller::Controller,
    shared::crypto::{Crypto, PasswordCrypto},
    use_cases::editor::app_ui::run_editor,
};

const ENCRYPTED_EXT: &str = "encrypted";

#[derive(Args)]
pub(crate) struct IndexController {
    /// Path to a file. Use ".encrypted" extension to encrypt a file
    path: PathBuf,
}

impl IndexController {
    fn is_encrypted(pb: &Path) -> bool {
        pb.extension()
            .map(|e| e.to_string_lossy().to_string().ends_with(ENCRYPTED_EXT))
            .unwrap_or(false)
    }

    fn ask_password(prompt: &str) -> anyhow::Result<String> {
        let result = dialoguer::Password::new()
            .with_prompt(prompt)
            .report(false)
            .interact()?;

        Ok(result)
    }

    fn read_file(
        crypto: PasswordCrypto,
        prompt: &str,
        path: &Path,
    ) -> anyhow::Result<(String, Option<String>)> {
        let data = std::fs::read(path).unwrap_or_default();
        let encrypted = Self::is_encrypted(path);

        if !encrypted {
            return Ok((String::from_utf8(data)?, None));
        }

        let password = Self::ask_password(prompt)?;

        if data.is_empty() {
            return Ok((String::default(), Some(password)));
        }

        let decrypted = crypto.decrypt(data.as_slice(), password.as_str())?;
        let text = String::from_utf8(decrypted)?;

        Ok((text, Some(password)))
    }

    fn write_file(
        crypto: PasswordCrypto,
        path: &Path,
        contents: &str,
        password: Option<&str>,
    ) -> anyhow::Result<()> {
        let is_encrypted = Self::is_encrypted(path);

        if !is_encrypted {
            std::fs::write(path, contents)?;
            return Ok(());
        }

        let password = password.ok_or_else(|| anyhow::anyhow!("Password is required"))?;
        let encrypted = crypto.encrypt(contents.as_bytes(), password)?;
        std::fs::write(path, encrypted)?;

        Ok(())
    }
}

impl Controller for IndexController {
    fn handle(&self) -> anyhow::Result<()> {
        let path = self.path.clone();
        let crypto = PasswordCrypto;
        let (decrypted_file_contents, password) = Self::read_file(
            crypto,
            "This is encrypted text. Provide a password",
            path.as_path(),
        )?;

        let save_path = path.clone();
        let save_password = password.clone();
        let result = run_editor(
            decrypted_file_contents,
            Some(path.to_string_lossy().to_string()),
            move |content| {
                Self::write_file(
                    crypto,
                    save_path.as_path(),
                    content.as_str(),
                    save_password.as_deref(),
                )?;
                Ok(())
            },
        )?;

        if result.need_save() {
            Self::write_file(
                crypto,
                path.as_path(),
                result.content(),
                password.as_deref(),
            )?;
        }

        Ok(())
    }
}
