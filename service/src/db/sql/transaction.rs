//! A set of SQL statements related to querying transactions.

pub const SELECT_LATEST_TX: &str = "
SELECT * FROM Transaction
ORDER by number desc
LIMIT $1;
";

pub const CREATE_NEW_TX: &str = "
BEGIN;
CALL UpdateUser($1, $3, 0);
CALL UpdateUser($2, $3, 1);
CALL InsertTx($1, $2, $3);
COMMIT;
";
