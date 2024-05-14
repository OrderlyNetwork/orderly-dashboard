# Orderly Dashboard FE

Frontend for Orderly Dashboard via Remix, React & Express.

## Base endpoint Info
### Backend RESTful api url
* https://api-evm.orderly.org/ (Mainnet)
* https://testnet-api-evm.orderly.org (Testnet)

### Query service
* https://orderly-dashboard-query-service.orderly.network (Mainnet)
* https://dev-orderly-dashboard-query-service.orderly.network (Testnet)

## Development

Run the frontend via:

```
cd orderly-dashboard-FE
yarn
yarn dev
```

Then navigate to http://localhost:3000

## Docker build

Docker build & run is done from root project folder:

```
docker build -t orderly-dashboard-fe -f dockerfiles/Dockerfile-FE .
docker run --rm -it -p 3000:3000 --name orderly-dashboard-fe -e QUERY_SERVICE_URL=https://orderly-dashboard-query-service.orderly.network -e EVM_API_URL=https://api-evm.orderly.org orderly-dashboard-fe
```
