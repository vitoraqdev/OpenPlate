use crate::address::Address;
use serde::{Serialize, Deserialize};
use postgres::types::{ToSql, FromSql};
/*
Create Customer struct. The customer_id field should be a reference to a Customer.
A customer can order from multiple addresses, but only the last address used
will be stored in the database.

Customer has the following fields:
- id: String
- name: String
- last_name: String
- phone: String
- address: Address (last used, updated on order)
*/

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
pub struct Customer {
    /// Creates a new `Customer` to store the information of a customer.
    /// Example:
    /// ```
    /// let customer = Customer::new("John", "Doe", "123456789", Address::new("123 Main St", "Main", 3.0));
    /// ```
    pub name: String,
    pub last_name: String,
    pub phone: String,
    #[serde(flatten)]
    pub address: Address,
}