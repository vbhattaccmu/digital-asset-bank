//! A set of SQL statements are related to setting up the schema and stored procedures.

pub const SETUP_DATABASE: &str = "
-------- Schema and SP Setup ----------

CREATE TABLE IF NOT EXISTS Account(
    number BIGSERIAL,
    id BIGINT,  
    balance BIGINT,        

    PRIMARY KEY (id)
);

DROP PROCEDURE IF EXISTS InsertUser(
    IN _id BIGINT,         
    IN _balance BIGINT);

CREATE PROCEDURE InsertUser(
    IN _id BIGINT,         
    IN _balance BIGINT)  

LANGUAGE plpgsql 
AS $$ 
BEGIN 
    INSERT INTO Account(
        id,
        balance
    )
    VALUES
    (
        _id,         
        _balance
    );
END 
$$;

DROP PROCEDURE IF EXISTS UpdateUser(
    IN _id BIGINT,         
    IN _amount BIGINT,
    IN _flag BIGINT);

CREATE PROCEDURE UpdateUser(
    IN _id BIGINT,         
    IN _amount BIGINT,
    IN _flag BIGINT)  

LANGUAGE plpgsql 
AS $$ 
BEGIN 
    IF _flag = 0 THEN
        -- Update sender's balance
        UPDATE Account
        SET balance = balance - _amount
        WHERE id = _id;
    ELSE
        -- Update receiver's balance
        UPDATE Account
        SET balance = balance + _amount
        WHERE id = _id;
    END IF;
END 
$$;

CREATE TABLE IF NOT EXISTS Transaction(
    number BIGSERIAL,
    from_id BIGINT,  
    to_id BIGINT,  
    amount BIGINT,  

    PRIMARY KEY (number)
);

DROP PROCEDURE IF EXISTS InsertTx(
    IN _from_id BIGINT,     
    IN _to_id BIGINT,             
    IN _amount BIGINT);

CREATE PROCEDURE InsertTx(
    IN _from_id BIGINT,     
    IN _to_id BIGINT,             
    IN _amount BIGINT)  

LANGUAGE plpgsql 
AS $$ 
BEGIN 
    INSERT INTO Transaction(
        from_id,
        to_id,
        amount
    )
    VALUES
    (
        _from_id,     
        _to_id,    
        _amount
    );
END 
$$;

-------- Indexes ------------------

CREATE INDEX IF NOT EXISTS \"id_index\" ON Account (\"id\");
";

pub const DROP_ALL_TABLES: &str = "
DROP SCHEMA public CASCADE;
CREATE SCHEMA public;
GRANT ALL ON SCHEMA public TO postgres;
GRANT ALL ON SCHEMA public TO public;
";
