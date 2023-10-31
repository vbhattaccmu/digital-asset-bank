//! A set of SQL statements related to querying accounts.

pub const SELECT_ACCOUNT_INFO_BY_ID: &str = "
SELECT * FROM Account
WHERE id = COALESCE($1, id)
ORDER BY id desc;
";

pub const CREATE_NEW_USER: &str = "
CALL InsertUser(
    $1, -- id
    $2  -- balance
);
";
