pub(crate) const ALG_ECDSA_P256_SHA256: &str = "ECDSA_P256_SHA256";
pub(crate) const ALG_ECDSA_P384_SHA384: &str = "ECDSA_P384_SHA384";
pub(crate) const ALG_ED25519: &str = "ED25519";
pub(crate) const ALG_RSA_SHA256: &str = "RSA_SHA256";
pub(crate) const ALG_RSA_SHA384: &str = "RSA_SHA384";
pub(crate) const ALG_RSA_SHA512: &str = "RSA_SHA512";

pub(crate) const DN_COMMON: &str = "CN";
pub(crate) const DN_COUNTRY: &str = "C";
pub(crate) const DN_LOCALITY: &str = "L";
pub(crate) const DN_ORGANIZATION: &str = "O";
pub(crate) const DN_ORGANIZATIONAL_UNIT: &str = "OU";
pub(crate) const DN_STATE_OR_PROVINCE: &str = "ST";

pub(crate) const EKU_ANY: &str = "Any";
pub(crate) const EKU_CLIENT_AUTH: &str = "ClientAuth";
pub(crate) const EKU_CODE_SIGNING: &str = "CodeSigning";
pub(crate) const EKU_EMAIL_PROTECTION: &str = "EmailProtection";
pub(crate) const EKU_OSCP_SIGNING: &str = "OscpSigning";
pub(crate) const EKU_SERVER_AUTH: &str = "ServerAuth";
pub(crate) const EKU_TIME_STAMPING: &str = "TimeStamping";
pub(crate) const EKU_ALL: [&str; 7] = [
  EKU_ANY,
  EKU_CLIENT_AUTH,
  EKU_CODE_SIGNING,
  EKU_EMAIL_PROTECTION,
  EKU_OSCP_SIGNING,
  EKU_SERVER_AUTH,
  EKU_TIME_STAMPING,
];

pub(crate) const KIDM_SHA256: &str = "SHA256";
pub(crate) const KIDM_SHA384: &str = "SHA384";
pub(crate) const KIDM_SHA512: &str = "SHA512";

pub(crate) const KU_CONTENT_COMMITMENT: &str = "ContentCommitment";
pub(crate) const KU_CRL_SIGN: &str = "CrlSign";
pub(crate) const KU_DATA_ENCIPHERMENT: &str = "DataEncipherment";
pub(crate) const KU_DECIPHER_ONLY: &str = "DecipherOnly";
pub(crate) const KU_DIGITAL_SIGNATURE: &str = "DigitalSignature";
pub(crate) const KU_ENCIPHER_ONLY: &str = "EncipherOnly";
pub(crate) const KU_KEY_AGREEMENT: &str = "KeyAgreement";
pub(crate) const KU_KEY_CERT_SIGN: &str = "KeyCertSign";
pub(crate) const KU_KEY_ENCIPHERMENT: &str = "KeyEncipherment";
pub(crate) const KU_ALL: [&str; 9] = [
  KU_CONTENT_COMMITMENT,
  KU_CRL_SIGN,
  KU_DATA_ENCIPHERMENT,
  KU_DATA_ENCIPHERMENT,
  KU_DIGITAL_SIGNATURE,
  KU_ENCIPHER_ONLY,
  KU_KEY_AGREEMENT,
  KU_KEY_CERT_SIGN,
  KU_KEY_ENCIPHERMENT,
];

pub(crate) const NC_DIRECTORY_NAME: &str = "DN";
pub(crate) const NC_DNS_NAME: &str = "DNS";
pub(crate) const NC_IP_ADDRESS: &str = "IP";
pub(crate) const NC_RFC_822_NAME: &str = "Email";

pub(crate) const SAN_DNS_NAME: &str = "DNS";
pub(crate) const SAN_IP_ADDRESS: &str = "IP";
pub(crate) const SAN_RFC_822_NAME: &str = "Email";
