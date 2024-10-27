use chrono::NaiveDate;
use uuid::Uuid;
use rust_decimal::Decimal;

//Later, currency will be added for multi currency
//Using Decimal for money, maybe it's wrong, we will see.
pub struct BankAccount {
    bank_account_id: Uuid,
    account: Decimal,
    released_date: NaiveDate,
    owner_id: Uuid
}