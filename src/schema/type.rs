use std::{fmt, str::FromStr};

use crate::error::Error;

const OBJECT_KEYWORD: &str = "object";
const BOOLEAN_KEYWORD: &str = "boolean";
const STRING_KEYWORD: &str = "string";
const PASSWORD_KEYWORD: &str = "password";
const HOSTNAME_KEYWORD: &str = "hostname";
const INTEGER_KEYWORD: &str = "integer";
const ARRAY_KEYWORD: &str = "array";
const NUMBER_KEYWORD: &str = "number";
const DATE_TIME_KEYWORD: &str = "datetime";
const DATE_KEYWORD: &str = "date";
const TIME_KEYWORD: &str = "time";
const EMAIL_KEYWORD: &str = "email";
const IPV4_KEYWORD: &str = "ipv4";
const IPV6_KEYWORD: &str = "ipv6";
const URI_KEYWORD: &str = "uri";
const FILE_KEYWORD: &str = "file"; // TODO: Update spec
const PORT_KEYWORD: &str = "port";
const TEXT_KEYWORD: &str = "text"; // TODO: Update spec
const STRINGLIST_KEYWORD: &str = "stringlist"; // TODO: Update spec
const DNSMASQ_ADDRESS_KEYWORD: &str = "dnsmasq-address"; // TODO: Update spec
const CHRONY_ADDRESS_KEYWORD: &str = "chrony-address"; // TODO: Update spec
const IPTABLES_ADDRESS_KEYWORD: &str = "iptables-address"; // TODO: Update spec

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrimitiveType {
    Object,
    Boolean,
    String,
    Password,
    Hostname,
    Integer,
    Array,
    Number,
    DateTime,
    Date,
    Time,
    Email,
    IPv4,
    IPv6,
    Uri,
    File,
    Port,
    Text,
    StringList,
    DNSMasqAddress,
    ChronyAddress,
    IPTablesAddress,
}

impl AsRef<str> for PrimitiveType {
    fn as_ref(&self) -> &str {
        match self {
            PrimitiveType::Object => OBJECT_KEYWORD,
            PrimitiveType::Boolean => BOOLEAN_KEYWORD,
            PrimitiveType::String => STRING_KEYWORD,
            PrimitiveType::Password => PASSWORD_KEYWORD,
            PrimitiveType::Hostname => HOSTNAME_KEYWORD,
            PrimitiveType::Integer => INTEGER_KEYWORD,
            PrimitiveType::Array => ARRAY_KEYWORD,
            PrimitiveType::Number => NUMBER_KEYWORD,
            PrimitiveType::DateTime => DATE_TIME_KEYWORD,
            PrimitiveType::Date => DATE_KEYWORD,
            PrimitiveType::Time => TIME_KEYWORD,
            PrimitiveType::Email => EMAIL_KEYWORD,
            PrimitiveType::IPv4 => IPV4_KEYWORD,
            PrimitiveType::IPv6 => IPV6_KEYWORD,
            PrimitiveType::Uri => URI_KEYWORD,
            PrimitiveType::File => FILE_KEYWORD,
            PrimitiveType::Port => PORT_KEYWORD,
            PrimitiveType::Text => TEXT_KEYWORD,
            PrimitiveType::StringList => STRINGLIST_KEYWORD,
            PrimitiveType::DNSMasqAddress => DNSMASQ_ADDRESS_KEYWORD,
            PrimitiveType::ChronyAddress => CHRONY_ADDRESS_KEYWORD,
            PrimitiveType::IPTablesAddress => IPTABLES_ADDRESS_KEYWORD,
        }
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl FromStr for PrimitiveType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            OBJECT_KEYWORD => Ok(PrimitiveType::Object),
            BOOLEAN_KEYWORD => Ok(PrimitiveType::Boolean),
            STRING_KEYWORD => Ok(PrimitiveType::String),
            PASSWORD_KEYWORD => Ok(PrimitiveType::Password),
            HOSTNAME_KEYWORD => Ok(PrimitiveType::Hostname),
            INTEGER_KEYWORD => Ok(PrimitiveType::Integer),
            ARRAY_KEYWORD => Ok(PrimitiveType::Array),
            NUMBER_KEYWORD => Ok(PrimitiveType::Number),
            DATE_TIME_KEYWORD => Ok(PrimitiveType::DateTime),
            DATE_KEYWORD => Ok(PrimitiveType::Date),
            TIME_KEYWORD => Ok(PrimitiveType::Time),
            EMAIL_KEYWORD => Ok(PrimitiveType::Email),
            IPV4_KEYWORD => Ok(PrimitiveType::IPv4),
            IPV6_KEYWORD => Ok(PrimitiveType::IPv6),
            URI_KEYWORD => Ok(PrimitiveType::Uri),
            FILE_KEYWORD => Ok(PrimitiveType::File),
            PORT_KEYWORD => Ok(PrimitiveType::Port),
            TEXT_KEYWORD => Ok(PrimitiveType::Text),
            STRINGLIST_KEYWORD => Ok(PrimitiveType::StringList),
            DNSMASQ_ADDRESS_KEYWORD => Ok(PrimitiveType::DNSMasqAddress),
            CHRONY_ADDRESS_KEYWORD => Ok(PrimitiveType::ChronyAddress),
            IPTABLES_ADDRESS_KEYWORD => Ok(PrimitiveType::IPTablesAddress),
            _ => Err(Error::with_message("invalid primitive object type")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Type {
    primitive_type: PrimitiveType,
    optional: bool,
}

impl Type {
    pub fn primitive_type(&self) -> &PrimitiveType {
        &self.primitive_type
    }

    pub fn is_optional(&self) -> bool {
        self.optional
    }

    pub fn is_required(&self) -> bool {
        !self.optional
    }
}

impl Type {
    pub fn new(primitive_type: PrimitiveType, optional: bool) -> Type {
        Type {
            primitive_type,
            optional,
        }
    }

    pub fn new_optional(primitive_type: PrimitiveType) -> Type {
        Type {
            primitive_type,
            optional: true,
        }
    }

    pub fn new_required(primitive_type: PrimitiveType) -> Type {
        Type {
            primitive_type,
            optional: false,
        }
    }
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (optional, s) = if s.ends_with('?') {
            (true, &s[..s.len() - 1])
        } else {
            (false, s)
        };
        let primitive_type = s.parse::<PrimitiveType>()?;
        Ok(Type::new(primitive_type, optional))
    }
}

impl Default for Type {
    fn default() -> Type {
        Type::new_required(PrimitiveType::Object)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_optional() {
            write!(f, "{}?", self.primitive_type())
        } else {
            write!(f, "{}", self.primitive_type())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_types() {
        assert_eq!("string".parse::<PrimitiveType>().unwrap(), PrimitiveType::String);
        assert_eq!("boolean".parse::<PrimitiveType>().unwrap(), PrimitiveType::Boolean);
        assert_eq!("integer".parse::<PrimitiveType>().unwrap(), PrimitiveType::Integer);
        assert_eq!("object".parse::<PrimitiveType>().unwrap(), PrimitiveType::Object);
        assert_eq!("array".parse::<PrimitiveType>().unwrap(), PrimitiveType::Array);
        assert_eq!("password".parse::<PrimitiveType>().unwrap(), PrimitiveType::Password);
        assert_eq!("hostname".parse::<PrimitiveType>().unwrap(), PrimitiveType::Hostname);
        assert_eq!("number".parse::<PrimitiveType>().unwrap(), PrimitiveType::Number);
        assert_eq!("datetime".parse::<PrimitiveType>().unwrap(), PrimitiveType::DateTime);
        assert_eq!("date".parse::<PrimitiveType>().unwrap(), PrimitiveType::Date);
        assert_eq!("time".parse::<PrimitiveType>().unwrap(), PrimitiveType::Time);
        assert_eq!("email".parse::<PrimitiveType>().unwrap(), PrimitiveType::Email);
        assert_eq!("ipv4".parse::<PrimitiveType>().unwrap(), PrimitiveType::IPv4);
        assert_eq!("ipv6".parse::<PrimitiveType>().unwrap(), PrimitiveType::IPv6);
        assert_eq!("uri".parse::<PrimitiveType>().unwrap(), PrimitiveType::Uri);
        assert_eq!("file".parse::<PrimitiveType>().unwrap(), PrimitiveType::File);
        assert_eq!("port".parse::<PrimitiveType>().unwrap(), PrimitiveType::Port);
        assert_eq!("text".parse::<PrimitiveType>().unwrap(), PrimitiveType::Text);
        assert_eq!(
            "stringlist".parse::<PrimitiveType>().unwrap(),
            PrimitiveType::StringList
        );
        assert_eq!(
            "dnsmasq-address".parse::<PrimitiveType>().unwrap(),
            PrimitiveType::DNSMasqAddress
        );
        assert_eq!(
            "chrony-address".parse::<PrimitiveType>().unwrap(),
            PrimitiveType::ChronyAddress
        );
        assert_eq!(
            "iptables-address".parse::<PrimitiveType>().unwrap(),
            PrimitiveType::IPTablesAddress
        );
    }

    #[test]
    fn required_object_type() {
        assert!(!"string".parse::<Type>().unwrap().optional);
    }

    #[test]
    fn optional_object_type() {
        assert!("string?".parse::<Type>().unwrap().optional);
    }
}
