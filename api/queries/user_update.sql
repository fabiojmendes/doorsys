update users set name = $1, email = $2 where id = $3 returning *
