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
      "date": 123, // unix timestamp
      "amount": 1000,
      "status": "done" // "done" or "ongoing" or "cancelled"
    }
  ],
  "exchanges": [{}], // TODO
  "boosts": [
    {
      "direction": "incoming", // "incoming" or "outgoing"
      "date": 123, // unix timestamp
      "amount": 1000,
      "post": "www.firesky.com" // string or null
    }
  ],
  "transfers": [
    {
      "direction": "incoming", // "incoming" or "outgoing"
      "date": 123, // unix timestamp
      "amount": 1000,
      "to_address": "1DkyAJL8Kt8O67GJNKJbdd9083Qh26jklQepA"
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
