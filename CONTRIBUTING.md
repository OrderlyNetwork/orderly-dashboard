Thank you for interest in contributing to the Orderly Dashboard project! We welcome contributions from everyone. Below are various bits of information to help you get started.
## Pull Requests
All the contributions to `orderly dashboard` happen via Pull Requests. Please follow the following steps when creating a PR:

1. Fork the `orderly dashboard` repository and create a new branch there to do your work.
2. The branch can contain any number of commits. When merged, all commits will be squashed into a single commit.
3. The changes should be thoroughly tested. Please refer to this document for our testing guidelines and an overview of the testing infrastructure.
4. Code should be formatted. 
5. Building process should be `0` errors and `0` warnings.
5. When ready, send a pull request against the master branch of the `orderly dashboard` repository.
5. Feel free to submit draft PRs to get early feedback and to make sure you are on the right track.
6. The PR name should follow the template: `<type>(scope): <name>`. Where `type` is:
* `fix` for bug fixes;
* `feat` for new features;
* `refactor` for changes that reorganize code without adding new content;
* `doc` for changes that change documentation or comments;
* `test` for changes that introduce new tests;
* `chore` for grunt tasks like updating dependencies.  
And the `scope` is:
* `indexer`: orderly dashboard indexer
* `analyzer`: orderly dashboard analyzer
* `query service` or `Qs`: orderly dashboard query service
* `FE`: orderly dashboard front end
* empty: didn't make change on above scopes.
7. The PR should also contain a description when appropriate to provide additional information to help the reviewer inspect the proposed change.
## SQL commits
1. confirm to generate new schema in the package of [indexer](./orderly-dashboard-indexer) and [analyzer](./orderly-dashboard-analyzer)
```shell
cd orderly-dashboard-indexer
# or `cd orderly-dashboard-analyzer`
diesel migration run
```
2. confirm to add changes in `schema.rs`:
```shell
git add schema.rs
```
3. should not change sql in legacy migrations, those changes may not take effects. For example in [this commits](https://gitlab.com/orderlynetwork/orderly-v2/orderly-dashboard/-/commit/c3b472576c2b5647fb5a396cb68eba4d77bea01f), delete sql in `migrations/2024-03-14-234455_gas_fee/up.sql` but didn't updates `schema.rs`