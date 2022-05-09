use crate::{constants::*, load, parse, RawCertificate};
use anyhow::Result;
use rcgen::{
  BasicConstraints, Certificate, CertificateParams, CustomExtension, DistinguishedName,
  ExtendedKeyUsagePurpose, GeneralSubtree, IsCa, KeyIdMethod, KeyPair, KeyUsagePurpose,
  NameConstraints, SanType, SignatureAlgorithm,
};
use time::OffsetDateTime;
use std::{fmt::Debug, path::PathBuf};
use structopt::StructOpt;
use thiserror::Error;
// use time::OffsetDateTime;

#[derive(Debug, Error)]
enum CommandError {
  #[error("Missing output path for either key or certificate. CLI argument validation should have prevented this. (╯°□°)╯︵ ┻━┻")]
  MissingOutputPath,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Generates a Certificate")]
pub struct Command {
  #[structopt(env, long, takes_value = false)]
  is_authority: bool,
  #[structopt(env, long, takes_value = false)]
  unconstrained: bool,
  #[structopt(env, long, takes_value = false)]
  without_default_key_usages: bool,
  #[structopt(env, long, takes_value = false)]
  without_authority_key_identifier: bool,
  #[structopt(env, long, takes_value = false)]
  with_all_key_usages: bool,
  #[structopt(env, long, takes_value = false)]
  with_all_extended_key_usages: bool,

  #[structopt(env, long, short = "l", value_name = "U8", default_value = "0")]
  constrain_path_length: u8,
  #[structopt(env, long, short = "N", value_name = "U64")]
  serial_number: Option<u64>,

  #[structopt(env, long, short = "I", value_name = "Path", parse(try_from_str = load::certificate))]
  in_certificate: Option<RawCertificate>,
  #[structopt(env, long, short = "i", value_name = "Path", parse(try_from_str = load::key_pair))]
  in_key: Option<KeyPair>,

  #[structopt(env, long, short = "O", value_name = "Path", requires = "out-key")]
  out_certificate: Option<PathBuf>,
  #[structopt(
    env,
    long,
    short = "o",
    value_name = "Path",
    requires = "out-certificate"
  )]
  out_key: Option<PathBuf>,

  #[structopt(env, long, short = "C", value_name = "Path", requires = "ca-key", parse(try_from_str = load::certificate))]
  ca_certificate: Option<RawCertificate>,
  #[structopt(env, long, short = "c", value_name = "Path", requires = "ca-certificate", parse(try_from_str = load::key_pair))]
  ca_key: Option<KeyPair>,

  // #[structopt(env, long, short = "B", value_name = "RFC3339", default_value = "1975-01-01T00:00:00+00:00", parse(try_from_str = parse::datetime_utc))]
  #[structopt(env, long, short = "B", value_name = "RFC3339", default_value = "1975-01-01T00:00:00+00:00", parse(try_from_str = parse::offset_date_time))]
  valid_not_before: OffsetDateTime,
  // #[structopt(env, long, short = "A", value_name = "RFC3339", default_value = "2999-04-20T03:13:37+00:00", parse(try_from_str = parse::datetime_utc))]
  #[structopt(env, long, short = "A", value_name = "RFC3339", default_value = "2999-04-20T03:13:37+00:00", parse(try_from_str = parse::offset_date_time))]
  valid_not_after: OffsetDateTime,

  #[structopt(env, long, short = "n", value_name = "DN", default_value = "CN=niftygate certificate", parse(try_from_str = parse::distinguished_name))]
  distinguished_name: DistinguishedName,
  #[structopt(env, long, short = "s", value_name = "Type=Value", parse(try_from_str = parse::san_type))]
  subject_alt_name: Vec<SanType>,
  #[structopt(env, long, short = "p", value_name = "Type=Value", parse(try_from_str = parse::name_constraint))]
  name_constraint_permit: Vec<GeneralSubtree>,
  #[structopt(env, long, short = "e", value_name = "Type=Value", parse(try_from_str = parse::name_constraint))]
  name_constraint_exclude: Vec<GeneralSubtree>,

  #[structopt(
    env,
    long,
    short = "S",
    value_name = "Algorithm",
    case_insensitive = true,
    default_value = ALG_ECDSA_P256_SHA256,
    possible_value = ALG_ECDSA_P256_SHA256,
    possible_value = ALG_ECDSA_P384_SHA384,
    possible_value = ALG_ED25519,
    possible_value = ALG_RSA_SHA256,
    possible_value = ALG_RSA_SHA384,
    possible_value = ALG_RSA_SHA512,
    parse(try_from_str = parse::signature_algorithm)
  )]
  signature_algorithm: &'static SignatureAlgorithm,
  #[structopt(
    env,
    long,
    short = "M",
    value_name = "Method",
    case_insensitive = true,
    default_value = KIDM_SHA256,
    possible_value = KIDM_SHA256,
    possible_value = KIDM_SHA384,
    possible_value = KIDM_SHA512,
    parse(try_from_str = parse::key_identifier_method)
  )]
  key_identifier_method: KeyIdMethod,
  #[structopt(
    env,
    long,
    short = "U",
    value_name = "Purpose",
    case_insensitive = true,
    possible_value = KU_CONTENT_COMMITMENT,
    possible_value = KU_CRL_SIGN,
    possible_value = KU_DATA_ENCIPHERMENT,
    possible_value = KU_DECIPHER_ONLY,
    possible_value = KU_DIGITAL_SIGNATURE,
    possible_value = KU_ENCIPHER_ONLY,
    possible_value = KU_KEY_AGREEMENT,
    possible_value = KU_KEY_CERT_SIGN,
    possible_value = KU_KEY_ENCIPHERMENT,
    parse(try_from_str = parse::key_usage_purpose)
  )]
  key_usage: Vec<KeyUsagePurpose>,
  #[structopt(
    env,
    long,
    short = "E",
    value_name = "Purpose",
    case_insensitive = true,
    possible_value = EKU_ANY,
    possible_value = EKU_CLIENT_AUTH,
    possible_value = EKU_CODE_SIGNING,
    possible_value = EKU_EMAIL_PROTECTION,
    possible_value = EKU_OSCP_SIGNING,
    possible_value = EKU_SERVER_AUTH,
    possible_value = EKU_TIME_STAMPING,
    parse(try_from_str = parse::extended_key_usage_purpose)
  )]
  extended_key_usage: Vec<ExtendedKeyUsagePurpose>,
}

impl Command {
  pub(crate) fn execute(self) -> Result<()> {
    let mut params = CertificateParams::from(&self);
    params.key_pair = self.in_key;
    let certificate = Certificate::from_params(params)?;

    let ca = match (self.ca_certificate, self.ca_key) {
      (Some(raw), Some(key_pair)) => {
        let ca_cert = raw.0;
        let params = CertificateParams::from_ca_cert_der(&ca_cert, key_pair)?;
        let ca = Certificate::from_params(params)?;
        Some(ca)
      }
      _ => None,
    };

    let secret = certificate.serialize_private_key_pem();
    let public = match ca {
      Some(ca) => certificate.serialize_pem_with_signer(&ca)?,
      None => certificate.serialize_pem()?,
    };

    match (self.out_certificate, self.out_key) {
      (None, None) => {
        println!("{}", public);
        println!("{}", secret);
      }
      (Some(cert_path), Some(key_path)) => {
        std::fs::write(cert_path, public.as_bytes())?;
        std::fs::write(key_path, secret.as_bytes())?;
      }
      _ => return Err(CommandError::MissingOutputPath.into()),
    }

    Ok(())
  }
}

impl From<&Command> for CertificateParams {
  fn from(command: &Command) -> Self {
    let mut params = CertificateParams::default();
    params.alg = command.into();
    params.custom_extensions = command.into();
    params.distinguished_name = command.into();
    params.extended_key_usages = command.into();
    params.is_ca = command.into();
    params.key_identifier_method = command.into();
    // params.key_pair = command.into();
    params.key_usages = command.into();
    params.name_constraints = command.into();
    params.not_after = command.valid_not_after;
    params.not_before = command.valid_not_before;
    params.serial_number = command.serial_number;
    params.subject_alt_names = command.into();
    params.use_authority_key_identifier_extension = !command.without_authority_key_identifier;
    params
  }
}

impl From<Command> for Option<KeyPair> {
  fn from(command: Command) -> Self {
    command.in_key
  }
}

impl From<&Command> for &SignatureAlgorithm {
  fn from(command: &Command) -> Self {
    command.signature_algorithm
  }
}

impl From<&Command> for DistinguishedName {
  fn from(command: &Command) -> Self {
    command.distinguished_name.clone()
  }
}

impl From<&Command> for KeyIdMethod {
  fn from(command: &Command) -> Self {
    command.key_identifier_method.clone()
  }
}

impl From<&Command> for Vec<SanType> {
  fn from(command: &Command) -> Self {
    command.subject_alt_name.clone()
  }
}

impl From<&Command> for IsCa {
  fn from(command: &Command) -> Self {
    if command.is_authority {
      if command.unconstrained {
        IsCa::Ca(BasicConstraints::Unconstrained)
      } else {
        IsCa::Ca(BasicConstraints::Constrained(command.constrain_path_length))
      }
    } else {
      IsCa::SelfSignedOnly
    }
  }
}

impl From<&Command> for Option<NameConstraints> {
  fn from(command: &Command) -> Self {
    if command.name_constraint_permit.is_empty() && command.name_constraint_exclude.is_empty() {
      None
    } else {
      Some(NameConstraints {
        permitted_subtrees: command.name_constraint_permit.clone(),
        excluded_subtrees: command.name_constraint_exclude.clone(),
      })
    }
  }
}

impl From<&Command> for Vec<KeyUsagePurpose> {
  fn from(command: &Command) -> Self {
    if command.with_all_key_usages {
      return KU_ALL
        .iter()
        .flat_map(|&name| parse::key_usage_purpose(name))
        .collect();
    }

    let mut key_usages = command.key_usage.clone();
    if !command.without_default_key_usages {
      if command.is_authority {
        key_usages.push(KeyUsagePurpose::KeyCertSign);
      } else {
        key_usages.push(KeyUsagePurpose::DigitalSignature);
        key_usages.push(KeyUsagePurpose::KeyEncipherment);
      }
    }
    key_usages
  }
}

impl From<&Command> for Vec<ExtendedKeyUsagePurpose> {
  fn from(command: &Command) -> Self {
    if command.with_all_extended_key_usages {
      return EKU_ALL
        .iter()
        .flat_map(|&name| parse::extended_key_usage_purpose(name))
        .collect();
    }

    let mut extended_key_usages = command.extended_key_usage.clone();
    if !command.without_default_key_usages && !command.is_authority {
      extended_key_usages.push(ExtendedKeyUsagePurpose::ServerAuth);
    }
    extended_key_usages
  }
}

impl From<&Command> for Vec<CustomExtension> {
  fn from(_command: &Command) -> Self {
    Vec::new()
  }
}
