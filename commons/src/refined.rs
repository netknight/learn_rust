use chrono::{DateTime, Utc};
use nutype::nutype;

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 20),
    derive(Debug, PartialEq, Clone)
)]
pub struct Username(String);

#[nutype(
    validate(regex = r"^[^@]+@[^@]+\.[^@]+$"),
    derive(Debug, PartialEq, Clone)
)]
pub struct Email(String);

#[nutype(
    sanitize(trim, with = |s| s.replace(&['(', ')', '-', ' '][..], "")),
    validate(regex = r"^\+?[0-9]*$"),
    derive(Debug, PartialEq, Clone),
)]
pub struct PhoneNumber(String);

#[nutype(validate(greater = 0), derive(Debug, PartialEq, Clone))]
pub struct PositiveU64(u64);

#[nutype(
    validate(predicate = |d| d > &Utc::now()),
    derive(Debug, PartialEq, Clone)
)]
pub struct FutureDate(DateTime<Utc>);

#[nutype(
    validate(predicate = |d| d < &Utc::now()),
    derive(Debug, PartialEq, Clone)
)]
pub struct PastDate(DateTime<Utc>);

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_username_sanitization() {
        let username = Username::try_new("  ExampleUser  ").unwrap();
        assert_eq!(username.into_inner(), "exampleuser");
    }

    #[test]
    fn test_username_validation() {
        assert!(Username::try_new("validusername").is_ok());
        assert!(Username::try_new("").is_err());
        assert!(Username::try_new(&fakeit::internet::username()).is_ok());
        assert!(Username::try_new("a_very_long_username_exceeding_limit").is_err());
    }

    #[test]
    fn test_email_validation() {
        assert!(Email::try_new("test@test.com").is_ok());
        assert!(Email::try_new(&fakeit::contact::email()).is_ok());
        assert!(Email::try_new("").is_err());
        assert!(Email::try_new("test@").is_err());
        assert!(Email::try_new("@test.com").is_err());
        assert!(Email::try_new("test@test").is_err());
        assert!(Email::try_new("invalid_email").is_err());
    }

    #[test]
    fn test_phone_number_sanitization() {
        let phone_number = PhoneNumber::try_new(" (123) 456-7890 ").unwrap();
        assert_eq!(phone_number.into_inner(), "1234567890");
    }

    #[test]
    fn test_phone_number_validation() {
        assert!(PhoneNumber::try_new("+1234567890").is_ok());
        assert!(PhoneNumber::try_new("1234567890").is_ok());
        assert!(PhoneNumber::try_new(&fakeit::contact::phone()).is_ok());
        assert!(PhoneNumber::try_new(&fakeit::contact::phone_formatted()).is_ok());
        assert!(PhoneNumber::try_new("invalid_phone_number").is_err());
    }

    #[test]
    fn test_positive_u64_validation() {
        assert!(PositiveU64::try_new(42).is_ok());
        assert!(PositiveU64::try_new(0).is_err());
    }

    #[test]
    fn test_future_date_validation() {
        let future_date = Utc::now() + chrono::Duration::days(1);
        assert!(FutureDate::try_new(future_date).is_ok());

        let past_date = Utc::now() - chrono::Duration::days(1);
        assert!(FutureDate::try_new(past_date).is_err());
    }

    #[test]
    fn test_past_date_validation() {
        let past_date = Utc::now() - chrono::Duration::days(1);
        assert!(PastDate::try_new(past_date).is_ok());

        let future_date = Utc::now() + chrono::Duration::days(1);
        assert!(PastDate::try_new(future_date).is_err());
    }
}
