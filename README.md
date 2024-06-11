# Orderly dashboard
Orderly Dashboard is aim to be a web-based data analyze platform that will present some of the most popular analytical indicator data in Orderly Blockchain. It also allows users to query the analyzed data of transactions that have occurred in Orderly Blockchain. Users can also query the analyzed data through the Analyzer module of Orderly Dashboard. Analyzed  data can be displayed in the form of visual charts. The statistical analysis data can be downloaded by data users. Orderly Dashboard will generate chart links from data,those generated chart links can be also embedded into other web pages or shared by users in other ways. Users can also construct their own queries through the data of Orderly Dashboard and create their own Dashboard Board by Orderly Dashboard data.
## Components
* [Orderly dashboard indexer](./orderly-dashboard-indexer)
  * Indexing orderly blockchain's tx and events and decode them, save them to database, provide api to query them by sequence
* [Orderly dashboard analyzer](./orderly-dashboard-analyzer)
* [Orderly dashboard query service](./orderly-dashboard-query-service)
  * Provide query api for orderly dashboard FE, source data generate by [Orderly dashboard analyzer](./orderly-dashboard-analyzer) or [Orderly dashboard indexer](./orderly-dashboard-indexer)
* [Orderly dashboard FE](./orderly-dashboard-FE)
  * A web front end for query and present data from orderly dashboard indexer or analyzer
## Documentation
- [Build and run](./BUILD_AND_RUN.md) will help you to deploy orderly dashboard if you want to deploy orderly dashboard project
- [How to indexing contract events](./orderly-dashboard-indexer/doc/How-to-indexing-data.md) will help you to know how index can index all events from smart contract with contract upgrade

## Service
- [prod](https://orderly-dashboard.orderly.network)
- [testnet](https://dev-orderly-dashboard.orderly.network)