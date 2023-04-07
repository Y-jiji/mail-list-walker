use crate::conf::{Config, ConfigError};
use crate::text::MailMessage;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct MailService {
    provider: String,
    port: u16,
    username: String,
    password: String,
}

pub enum MailError {
    SMTPWrongProvider(String),
    SMTPTestConnectionFailure(String),
    SMTPWrongAddressFormat(String),
    SMTPCannotBuildMessage,
    SMTPMailSendingFailure(String),
    ConfigErrorCannotParsePortU16,
    External(ConfigError),
}

impl ToString for MailError {
    fn to_string(&self) -> String {
        use MailError::*;
        match &self {
            SMTPCannotBuildMessage => format!("Cannot build a message"),
            SMTPMailSendingFailure(who) => format!("Cannot send message to {who}"),
            SMTPTestConnectionFailure(who) => format!("Test connection failed on {who}"),
            ConfigErrorCannotParsePortU16 => format!("Cannot parse port in configuration to u16"),
            External(config) => config.clone().into(),
            SMTPWrongAddressFormat(addr) => format!("{addr} is not a correct message format, it should look like \"xxxx <xxxxx@example.mail.com>\""),
            SMTPWrongProvider(who) => format!("May be {who} is not a right email provider!"),
        }
    }
}

impl MailService {
    pub fn load(conf: &Config) -> Result<MailService, MailError> {
        use MailError::*;
        Ok(Self {
            provider: conf.get("SERVICE.PROVIDER").map_err(External)?,
            port: conf.get("SERVICE.PORT").map_err(External)?.parse::<u16>().map_err(|_| ConfigErrorCannotParsePortU16)?,
            username: conf.get("SERVICE.USERNAME").map_err(External)?,
            password: conf.get("SERVICE.PASSWORD").map_err(External)?
        })
    }
    pub fn send(&self, message: MailMessage) -> Result<(), MailError> {
        use MailError::*;
        let to_who = message.to.clone();
        let builder = SmtpTransport::relay(&self.provider)
            .map_err(|_| SMTPWrongProvider(self.provider.to_string()))?
            .port(self.port)
            .credentials(Credentials::new(
                self.username.to_owned(),
                self.password.to_owned(),
            ));
        let service = builder.build();
        let checkok = service.test_connection().map_err(|_| SMTPTestConnectionFailure(to_who.clone()))?;
        if !checkok { Err(SMTPTestConnectionFailure(to_who.clone()))? }
        let message = Message::builder()
            .from(message.from.parse().map_err(|_| SMTPWrongAddressFormat(message.from))?)
            .to(message.to.parse().map_err(|_| SMTPWrongAddressFormat(message.to))?)
            .subject(message.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(message.content)
            .map_err(|_| SMTPCannotBuildMessage)?;
        service.send(&message).map_err(|_| SMTPMailSendingFailure(to_who))?;
        Ok(())
    }
}
