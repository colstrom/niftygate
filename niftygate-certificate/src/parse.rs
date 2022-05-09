use crate::constants::*;
use rcgen::{
  CidrSubnet, DistinguishedName, DnType, ExtendedKeyUsagePurpose, GeneralSubtree, KeyIdMethod,
  KeyUsagePurpose, SanType, SignatureAlgorithm,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use thiserror::Error;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

// pub(crate) fn datetime_utc(datetime: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
//   Ok(DateTime::parse_from_rfc3339(datetime)?.into())
// }

pub(crate) fn offset_date_time(datetime: &str) -> Result<OffsetDateTime, time::error::Parse> {
  time::OffsetDateTime::parse(datetime, &Rfc3339)
}

#[derive(Debug, Error)]
#[error("Unrecognized Key Identifier Method. CLI argument validation should have prevented this. (╯°□°)╯︵ ┻━┻")]
pub(crate) struct UnrecognizedKeyIdentifierMethod;

pub(crate) fn key_identifier_method(
  name: &str,
) -> Result<KeyIdMethod, UnrecognizedKeyIdentifierMethod> {
  match name {
    KIDM_SHA256 => Ok(KeyIdMethod::Sha256),
    KIDM_SHA384 => Ok(KeyIdMethod::Sha384),
    KIDM_SHA512 => Ok(KeyIdMethod::Sha512),
    _ => Err(UnrecognizedKeyIdentifierMethod),
  }
}

#[derive(Debug, Error)]
#[error("Unrecognized Signature Algorithm. CLI argument validation should have prevented this. (╯°□°)╯︵ ┻━┻")]
pub(crate) struct UnrecognizedSignatureAlgorithm;

pub(crate) fn signature_algorithm(
  signature_algorithm: &str,
) -> Result<&'static SignatureAlgorithm, UnrecognizedSignatureAlgorithm> {
  match signature_algorithm.to_uppercase().as_str() {
    ALG_ECDSA_P256_SHA256 => Ok(&rcgen::PKCS_ECDSA_P256_SHA256),
    ALG_ECDSA_P384_SHA384 => Ok(&rcgen::PKCS_ECDSA_P384_SHA384),
    ALG_ED25519 => Ok(&rcgen::PKCS_ED25519),
    ALG_RSA_SHA256 => Ok(&rcgen::PKCS_RSA_SHA256),
    ALG_RSA_SHA384 => Ok(&rcgen::PKCS_RSA_SHA384),
    ALG_RSA_SHA512 => Ok(&rcgen::PKCS_RSA_SHA512),
    _ => Err(UnrecognizedSignatureAlgorithm),
  }
}

#[derive(Debug, Error)]
#[error(
  "Unrecognized Key Usage Purpose. CLI argument validation should have prevented this. (╯°□°)╯︵ ┻━┻"
)]
pub(crate) struct UnrecognizedKeyUsagePurpose;

pub(crate) fn key_usage_purpose(
  name: &str,
) -> Result<KeyUsagePurpose, UnrecognizedKeyUsagePurpose> {
  match name {
    KU_CONTENT_COMMITMENT => Ok(KeyUsagePurpose::ContentCommitment),
    KU_CRL_SIGN => Ok(KeyUsagePurpose::CrlSign),
    KU_DATA_ENCIPHERMENT => Ok(KeyUsagePurpose::DataEncipherment),
    KU_DECIPHER_ONLY => Ok(KeyUsagePurpose::DecipherOnly),
    KU_DIGITAL_SIGNATURE => Ok(KeyUsagePurpose::DigitalSignature),
    KU_ENCIPHER_ONLY => Ok(KeyUsagePurpose::EncipherOnly),
    KU_KEY_AGREEMENT => Ok(KeyUsagePurpose::KeyAgreement),
    KU_KEY_CERT_SIGN => Ok(KeyUsagePurpose::KeyCertSign),
    KU_KEY_ENCIPHERMENT => Ok(KeyUsagePurpose::KeyEncipherment),
    _ => Err(UnrecognizedKeyUsagePurpose),
  }
}

#[derive(Debug, Error)]
#[error("Unrecognized Extended Key Usage Purpose. CLI argument validation should have prevented this. (╯°□°)╯︵ ┻━┻")]
pub(crate) struct UnrecognizedExtendedKeyUsagePurpose;

pub(crate) fn extended_key_usage_purpose(
  name: &str,
) -> Result<ExtendedKeyUsagePurpose, UnrecognizedExtendedKeyUsagePurpose> {
  match name {
    EKU_ANY => Ok(ExtendedKeyUsagePurpose::Any),
    EKU_CLIENT_AUTH => Ok(ExtendedKeyUsagePurpose::ClientAuth),
    EKU_CODE_SIGNING => Ok(ExtendedKeyUsagePurpose::CodeSigning),
    EKU_EMAIL_PROTECTION => Ok(ExtendedKeyUsagePurpose::EmailProtection),
    EKU_OSCP_SIGNING => Ok(ExtendedKeyUsagePurpose::OcspSigning),
    EKU_SERVER_AUTH => Ok(ExtendedKeyUsagePurpose::ServerAuth),
    EKU_TIME_STAMPING => Ok(ExtendedKeyUsagePurpose::TimeStamping),
    _ => Err(UnrecognizedExtendedKeyUsagePurpose),
  }
}

#[derive(Debug, Error)]
pub(crate) enum SanTypeError {
  #[error("Invalid SAN Type (Unrecognized Type)")]
  UnrecognizedType,
  #[error("Invalid SAN Type (IP Address Syntax)")]
  IpAddressSyntax,
}

pub(crate) fn san_type(s: &str) -> Result<SanType, SanTypeError> {
  match s.split_once("=") {
    None => Ok(SanType::DnsName(s.to_string())),
    Some((prefix, rest)) => match prefix {
      SAN_DNS_NAME => Ok(SanType::DnsName(rest.to_string())),
      SAN_RFC_822_NAME => Ok(SanType::Rfc822Name(rest.to_string())),
      SAN_IP_ADDRESS => {
        if rest.contains(':') {
          match rest.parse::<Ipv6Addr>() {
            Ok(addr) => Ok(SanType::IpAddress(IpAddr::V6(addr))),
            Err(_) => Err(SanTypeError::IpAddressSyntax),
          }
        } else {
          match rest.parse::<Ipv4Addr>() {
            Ok(addr) => Ok(SanType::IpAddress(IpAddr::V4(addr))),
            Err(_) => Err(SanTypeError::IpAddressSyntax),
          }
        }
      }
      _ => Err(SanTypeError::UnrecognizedType),
    },
  }
}

#[derive(Debug, Error)]
pub(crate) enum NameConstraintError {
  #[error("Invalid Name Constraint (Missing Delimiter)")]
  MissingDelimiter,
  #[error("Invalid Name Constraint (Unrecognized Type)")]
  UnrecognizedType,
  #[error("Invalid Name Constraint (IP Address Syntax)")]
  IpAddressSyntax,
  #[error("{0}")]
  DistinguishedName(DistinguishedNameError),
}

pub(crate) fn name_constraint(constraint: &str) -> Result<GeneralSubtree, NameConstraintError> {
  match constraint.split_once('=') {
    None => Err(NameConstraintError::MissingDelimiter),
    Some((prefix, rest)) => match prefix {
      NC_DNS_NAME => Ok(GeneralSubtree::DnsName(rest.to_string())),
      NC_RFC_822_NAME => Ok(GeneralSubtree::Rfc822Name(rest.to_string())),
      NC_DIRECTORY_NAME => match distinguished_name(rest) {
        Ok(distinguished_name) => Ok(GeneralSubtree::DirectoryName(distinguished_name)),
        Err(error) => Err(NameConstraintError::DistinguishedName(error)),
      },
      NC_IP_ADDRESS => match CidrSubnet::from_str(rest) {
        Ok(cidr_subnet) => Ok(GeneralSubtree::IpAddress(cidr_subnet)),
        Err(_) => Err(NameConstraintError::IpAddressSyntax),
      },
      _ => Err(NameConstraintError::UnrecognizedType),
    },
  }
}

#[derive(Debug, Error)]
pub(crate) enum DistinguishedNameError {
  #[error("Invalid Distinguished Name (Missing Delimiter)")]
  MissingDelimiter,
  #[error("Invalid Distinguised Name (Unrecognized Type)")]
  UnrecognizedType,
}

pub(crate) fn distinguished_name(s: &str) -> Result<DistinguishedName, DistinguishedNameError> {
  let mut distinguished_name = DistinguishedName::new();

  for component in s.split('/') {
    match component.split_once('=') {
      None => return Err(DistinguishedNameError::MissingDelimiter),
      Some((ty, s)) => {
        let ty = match ty {
          DN_COMMON => DnType::CommonName,
          DN_COUNTRY => DnType::CountryName,
          DN_LOCALITY => DnType::LocalityName,
          DN_ORGANIZATION => DnType::OrganizationName,
          DN_ORGANIZATIONAL_UNIT => DnType::OrganizationalUnitName,
          DN_STATE_OR_PROVINCE => DnType::StateOrProvinceName,
          // "CUSTOM" => DnType::CustomDnType(),
          _ => return Err(DistinguishedNameError::UnrecognizedType),
        };
        distinguished_name.push(ty, s);
      }
    }
  }

  Ok(distinguished_name)
}
