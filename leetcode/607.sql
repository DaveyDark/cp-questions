-- Question:
-- Find the names of all the salespersons who did not have any orders related to the company with the name "RED".
-- Return the result table in any order.

SELECT name
FROM SalesPerson
WHERE sales_id NOT IN (
    SELECT sales_id
    FROM Orders
    WHERE com_id = (
        SELECT com_id
        FROM Company
        WHERE name = 'RED'
    )
);
