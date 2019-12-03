use lettre::{
    SmtpTransport,
    SmtpClient,
    SendableEmail,
    Envelope,
    smtp::ClientSecurity,
    smtp::error::Error,
};

use crate::lettre::Transport;

pub fn send_password_reset_email(to: &str, password: &str) -> Result<(), Error>
{
    let email = SendableEmail::new(
        Envelope::new(
            Some("passwordreset@astaprint.upb.de".parse().unwrap()),
            vec![to.parse().unwrap()],
        ).unwrap(),
        "id".to_string(),
        format!("Subject: AStA Copyclient Password Reset\nTo: {}\n\n Your New Password: {}", to, password).as_bytes().to_vec(),
    );

    let mut mailer = SmtpTransport::new(SmtpClient::new("localhost:25", ClientSecurity::None)?);

    let _response = mailer.send(email)?;

    Ok(())
}

