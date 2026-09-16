use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationError {
    field: &'static str,
}

impl ValidationError {
    const fn new(field: &'static str) -> Self {
        Self { field }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid {}", self.field)
    }
}

impl std::error::Error for ValidationError {}

fn parse_numeric_id(value: &str, field: &'static str) -> Result<String, ValidationError> {
    let normalized = value.trim();
    if normalized.is_empty()
        || normalized.len() > 20
        || !normalized.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(ValidationError::new(field));
    }

    Ok(normalized.to_owned())
}

fn parse_text(
    value: &str,
    field: &'static str,
    max_length: usize,
) -> Result<String, ValidationError> {
    let without_controls: String = value
        .chars()
        .filter(|character| !character.is_control())
        .collect();
    let normalized = without_controls.trim();
    let length = normalized.chars().count();

    if length == 0 || length > max_length {
        return Err(ValidationError::new(field));
    }

    Ok(normalized.to_owned())
}

macro_rules! numeric_id {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: &str) -> Result<Self, ValidationError> {
                parse_numeric_id(value, $field).map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

numeric_id!(RoomId, "room id");
numeric_id!(UserId, "user id");
numeric_id!(UserUid, "user uid");

macro_rules! validated_text {
    ($name:ident, $field:literal, $max_length:literal) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: &str) -> Result<Self, ValidationError> {
                parse_text(value, $field, $max_length).map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

validated_text!(Username, "username", 50);
validated_text!(SensitiveWord, "sensitive word", 100);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Volume(f64);

impl Volume {
    pub fn parse(value: f64) -> Result<Self, ValidationError> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(ValidationError::new("volume"));
        }

        Ok(Self(value))
    }

    pub const fn get(self) -> f64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Threshold(u64);

impl Threshold {
    pub fn parse(value: i64) -> Result<Self, ValidationError> {
        u64::try_from(value)
            .map(Self)
            .map_err(|_| ValidationError::new("threshold"))
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}
