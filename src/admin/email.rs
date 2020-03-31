// AStAPrint
// Copyright (C) 2018, 2019 AStA der Universität Paderborn
//
// Authors: Gerrit Pape <gerrit.pape@asta.upb.de>
//
// This file is part of AStAPrint
//
// AStAPrint is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
use lettre::{
    smtp::{
        error::Error,
        ClientSecurity,
    },
    Envelope,
    SendableEmail,
    SmtpClient,
    SmtpTransport,
    Transport,
};

/// sending a password to a given mail
/// uses smtp so it only works within the university network
pub fn send_password_reset_email(to: &str, password: &str) -> Result<(), Error>
{
    let email = SendableEmail::new(
        Envelope::new(Some("passwordreset@astaprint.upb.de".parse().unwrap()), vec![
            to.parse().unwrap(),
        ])
        .unwrap(),
        "id".to_string(),
        format!(
            "Subject: AStA Copyclient Password Reset\nTo: {}\n\n Your New Password: {}",
            to, password
        )
        .as_bytes()
        .to_vec(),
    );

    let mut mailer = SmtpTransport::new(SmtpClient::new("localhost:25", ClientSecurity::None)?);

    let _response = mailer.send(email)?;

    Ok(())
}
