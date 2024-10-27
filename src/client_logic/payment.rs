use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

//Currency will be added later
pub struct Payment {
    sender_bank_account_id: Uuid,
    receiver_bank_account_id: Uuid,
    amount: Decimal,
    date_of_payment: DateTime<Utc>
}