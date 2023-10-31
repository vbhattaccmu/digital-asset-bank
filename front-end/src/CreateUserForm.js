import React, { useState, useEffect } from "react";
import axios from "axios";

function CreateUserForm() {
  const [userId, setUserId] = useState("");
  const [balance, setBalance] = useState("");
  const [userResponse, setUserResponse] = useState("");

  const [fromId, setFromId] = useState("");
  const [toId, setToId] = useState("");
  const [transactionAmount, setTransactionAmount] = useState("");
  const [transactionResponse, setTransactionResponse] = useState("");

  const [users, setUsers] = useState([]);
  const [transactions, setTransactions] = useState([]);

  const handleUserSubmit = async (e) => {
    e.preventDefault();

    // Validation: Check for negative values
    if (parseInt(userId) < 0 || parseInt(balance) < 0) {
      setUserResponse("User ID and balance cannot be negative.");
      return;
    }

    try {
      const response = await axios.post("http://localhost:9095/users", {
        id: parseInt(userId),
        balance: parseInt(balance),
      });

      setUserResponse(response.data);
      fetchUsers();
      setUserId("");
      setBalance("");
    } catch (error) {
      console.error("Error:", error);
      setUserResponse(
        error.response ? error.response.data : "An error occurred",
      );
    }
  };

  const handleTransactionSubmit = async (e) => {
    e.preventDefault();

    // Validation: Check for negative values
    if (
      parseInt(fromId) < 0 ||
      parseInt(toId) < 0 ||
      parseInt(transactionAmount) < 0
    ) {
      setTransactionResponse("From ID, To ID, and amount cannot be negative.");
      return;
    }

    try {
      await axios.post("http://localhost:9095/transactions", {
        from_id: parseInt(fromId),
        to_id: parseInt(toId),
        amount: parseInt(transactionAmount),
      });

      setTransactionResponse("Transaction submitted");
      fetchTransactions();
      fetchUsers(); // Fetch updated users data after transaction
      setFromId("");
      setToId("");
      setTransactionAmount("");
    } catch (error) {
      console.error("Error:", error);
      setTransactionResponse(
        error.response ? error.response.data : "An error occurred",
      );
    }
  };

  const fetchUsers = () => {
    axios
      .get("http://localhost:9095/users")
      .then((response) => setUsers(response.data))
      .catch((error) => console.error("Error fetching users:", error));
  };

  const fetchTransactions = () => {
    axios
      .get("http://localhost:9095/transactions")
      .then((response) => setTransactions(response.data))
      .catch((error) => console.error("Error fetching transactions:", error));
  };

  useEffect(() => {
    fetchUsers();
    fetchTransactions();
  }, []);

  return (
    <div
      style={{ display: "flex", flexDirection: "column", alignItems: "center" }}
    >
      {/* User Creation Form */}
      <div>
        <h2>Create User</h2>
        <form onSubmit={handleUserSubmit}>
          <label>
            User ID:
            <input
              type="number"
              value={userId}
              onChange={(e) => setUserId(e.target.value)}
            />
          </label>
          <br />
          <label>
            Balance:
            <input
              type="number"
              value={balance}
              onChange={(e) => setBalance(e.target.value)}
            />
          </label>
          <br />
          <button type="submit">Submit User</button>
        </form>
        <p>{userResponse}</p>
      </div>

      {/* Transaction Form */}
      <div>
        <h2>Make Transaction</h2>
        <form onSubmit={handleTransactionSubmit}>
          <label>
            From ID:
            <input
              type="number"
              value={fromId}
              onChange={(e) => setFromId(e.target.value)}
            />
          </label>
          <br />
          <label>
            To ID:
            <input
              type="number"
              value={toId}
              onChange={(e) => setToId(e.target.value)}
            />
          </label>
          <br />
          <label>
            Amount:
            <input
              type="number"
              value={transactionAmount}
              onChange={(e) => setTransactionAmount(e.target.value)}
            />
          </label>
          <br />
          <button type="submit">Submit Transaction</button>
        </form>
        <p>{transactionResponse}</p>
      </div>

      {/* Users Table */}
      <div>
        <h2>Users</h2>
        <table>
          <thead>
            <tr>
              <th>ID</th>
              <th>Balance</th>
            </tr>
          </thead>
          <tbody>
            {users.map((user) => (
              <tr key={user.id}>
                <td>{user.id}</td>
                <td>{user.balance}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Transactions Table */}
      <div>
        <h2>Transactions</h2>
        <table>
          <thead>
            <tr>
              <th>From ID</th>
              <th>To ID</th>
              <th>Amount</th>
            </tr>
          </thead>
          <tbody>
            {transactions.map((transaction) => (
              <tr key={transaction.from_id}>
                <td>{transaction.from_id}</td>
                <td>{transaction.to_id}</td>
                <td>{transaction.amount}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default CreateUserForm;
