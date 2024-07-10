Example actix_web with POST reading users in a MariaDB database.

Build and run the server:
```bash
cargo install cargo-run-script
cargo run-script db_create
cargo run
```

Send a post request to the server:
```bash
curl \
  --header "Content-Type: application/json" \
  --request POST \
  --data '{"id": 2}' \
  http://127.0.0.1:8080/api/user | python -m json.tool
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100    60  100    51  100     9  36770   6488 --:--:-- --:--:-- --:--:-- 60000
{
    "id": 2,
    "username": "Bill",
    "email": "bill@gmail.com"
}

```
  
