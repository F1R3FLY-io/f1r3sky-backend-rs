# Wallet api

## Get wallet balance and transaction history

Request:

GET `/api/wallet/state`

Responce:

200 OK

```json
{
  "address": "1DkyAJL8Kt8O67GJNKJbdd9083Qh26jklQepA",
  "balance": 1000,
  "requests": [
    {
      "id": "ddin2b48SD0-2d",
      "date": 123,
      // unix timestamp
      "amount": 1000,
      "status": "done"
      // "done" or "ongoing" or "cancelled"
    }
  ],
  "exchanges": [
    {}
  ],
  // TODO
  "boosts": [
    {
      "id": "ddin2b48SD0-2d",
      "username": "foo.bar",
      "direction": "incoming",
      // "incoming" or "outgoing"
      "date": 123,
      // unix timestamp
      "amount": 1000,
      "post": "www.firesky.com"
      // string or null
    }
  ],
  "transfers": [
    {
      "id": "ddin2b48SD0-2d",
      "direction": "incoming",
      // "incoming" or "outgoing"
      "date": 123,
      // unix timestamp
      "amount": "1000",
      "to_address": "1DkyAJL8Kt8O67GJNKJbdd9083Qh26jklQepA",
      "cost": "666"
    }
  ]
}
```

## Create transfer request

Request:

POST `/api/wallet/request`

```json
{
  "amount": 100,
  "description": ""
}
```

Responce:

201 CREATED

```json
{
  "id": ""
}
```

## Get transfer request

Request:

GET `/api/wallet/request/<id>`

Responce:

200 OK

```json
{
  "amount": 100,
  "description": "",
  "user_handle": ""
}
```

## Fulfill transfer request

Request:

POST `/api/wallet/request/<id>/fulfill`

Responce:

200 OK

## Transfer tokens

Request:

POST `/api/wallet/transfer`

```json
{
  "amount": 100,
  "to_address": "1DkyAJL8Kt8O67GJNKJbdd9083Qh26jklQepA",
  "description": ""
}
```

Responce:

200 OK

```json
{
  "cost": "666"
}
```
