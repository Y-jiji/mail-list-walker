use crate::conf::{Config, ConfigError};

#[derive(Debug, Clone)]
pub struct MailMessage {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct MailTemplate {
    template: MailMessage,
}

pub enum TemplateError {
    External(ConfigError),
}

impl ToString for TemplateError {
    fn to_string(&self) -> String {
        match self {
            Self::External(x) => x.clone().into(),
        }
    }
}

impl MailTemplate {
    pub fn load(config: &Config) -> Result<MailTemplate, TemplateError> {
        use TemplateError::*;
        let template = MailMessage {
            from: config.get("HEAD-FORMAT.FROM").map_err(External)?,
            to: config.get("HEAD-FORMAT.TO").map_err(External)?,
            subject: config.get("HEAD-FORMAT.SUBJECT").map_err(External)?,
            content: config.get("BODY-FORMAT").map_err(External)?,
        };
        Ok(Self { template })
    }
    pub fn fill(&self, info: Vec<(String, String)>) -> MailMessage {
        let mut msg = self.template.clone();
        for (pattern, content) in info {
            let pattern = format!("{}{pattern}{}", "{", "}");
            msg.from = msg.from.replace(&pattern, &content);
            msg.to = msg.to.replace(&pattern, &content);
            msg.subject = msg.subject.replace(&pattern, &content);
            msg.content = msg.content.replace(&pattern, &content);
        }
        return msg;
    }
}