# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.0.1 (2023-02-14)

### Chore

 - <csr-id-004aa955e8bed7687090762efa0bcc53577ecf2c/> added team developer to save spam
 - <csr-id-b18c039255180c8d18e786e783a40f5cf9724358/> tokens are now used
 - <csr-id-869294b93591055b8b078943771915aef0bf33d8/> redesign source and sinks to store features by environment and filter the responses by project
 - <csr-id-9a34999914d7c27b01b2ab7793863c8c139589fd/> remove sinks for offline mode
 - <csr-id-cdfa7c216c1b7066ab059259d319a8c8ce2dc82a/> redesign source/sink architecture
 - <csr-id-ba72e090c400e7d2d7f276a89ecf79f3760c7c47/> remove redis test that doesn't make sense anymore
 - <csr-id-286dfd536ff1c5d865829dcd98bda49da6ad9d36/> test auto-assign-pr action
 - <csr-id-e58f4fc3306ae71c1bcb8e8704d38eeb176cac96/> move server startup and traits to async
 - <csr-id-ea8cd1ba7fb36afb039f31ec4ba000a2b7271700/> improve tests for redis provider
 - <csr-id-9132cc1410d1d4a14e08de15ee53c9fce1fc5c92/> bump unleash-types
 - <csr-id-1d6a5188a6334b341db72f847f55450726da3bee/> Update cargo keys with ownership and license

### Documentation

 - <csr-id-e6fd6c5fda8adea94f06eaaf10033e9ae9a194a3/> add edge mode
   * docs: add edge mode
   
   * docs: organize modes differently, small fixes
   
   * docs: edge mode does not need token to start, explain warm up
   
   * Update README.md
 - <csr-id-16771118dbfdb4fc2dd819564b9d3f3355154134/> update README

### New Features

 - <csr-id-3a8cd761a8cd92696c9229df1a6c3614aae261fa/> switch to backing with HashMap<TokenString, EdgeToken>
 - <csr-id-0d037ec243b120f093b5a20efb3c5ddda6e25767/> adds a call for validating tokens
 - <csr-id-eab0878ce2bf49a499f032a13c47f58a4b346cc7/> implement simplify tokens
 - <csr-id-9e99f4b64b3d53b2e79381a2cb0d80ef4b010b2b/> add client for getting features
 - <csr-id-5ae644c8e4c98c588111a7461f359439c994209f/> implement an in memory data store
 - <csr-id-0469918e24763a5fef41a706f6f88fde986f955d/> internal backstage build info endpoint
   * feat: internal backstage build info endpoint
   * chore: add test documenting info endpoint
 - <csr-id-92aa64bc58e4193adc95370e651579feddea2811/> add enabled toggles routes
   * feat: add enabled toggles routes
   
   * fix: disabling metrics for not linux
 - <csr-id-5f55517e4407a7acf4b7906d82eee737bb58a53d/> add basic proxy endpoints and related test code
 - <csr-id-8fe7cabbb496c34618cae77e82ddceeeb8cfb617/> use subcommands rather than ValueEnum
   Worth mentioning
   
   The value here is harder to see, since we still only have one subcommand. Judging from our experience in the node version here, where we ended up with a rather complicated yaml config, this is pre-optimising a bit to ease the extension into more modes later.
 - <csr-id-3addbd639c12749c5d18775f95b1bfede106c4cf/> Added cors middleware
 - <csr-id-e6bc817c21affd7e06883a9d56f85f254878a4c8/> Add edge-token extractor to lock down access
 - <csr-id-4bf25a3402c8e9a3c48c63118da1469a69a3bbdd/> Adds client features endpoint
   This is worth a comment, since the way it was solved depends on which
   mode you're executing edge in. Currently edge only supports one mode, so
   we could've just explicitly passed in the provider for getting features.
   
   However, we are aware that we want at least two more providers for
   features, so this lays the ground work for implementing more modes and
   providers, but still using the same actix handler for the
   `/api/client/features` endpoint.
 - <csr-id-c270685a08207e0ab283e563ad6f58ad4f859161/> add /api/client/features endpoint
 - <csr-id-231efc30353f6af6f20b8431220101802ca5c2b3/> Server with metrics and health check ready

### Bug Fixes

 - <csr-id-eea450a47bfe5c32ea84994570223c1d5a746bc8/> update rust crate unleash-types to 0.8.3
 - <csr-id-4f528b76b718405d151a06af6657376c9358a7a2/> update rust crate unleash-types to 0.8.2
 - <csr-id-2d4a74312db1e5adc0d042e52e47c4f7286a966d/> update rust crate unleash-yggdrasil to 0.4.5
 - <csr-id-986a7433f687de3126cf05bf8d776cabf3a28290/> update rust crate serde_json to 1.0.93
 - <csr-id-cd86cdd7c5f6a9a6577a10b01278e3b17e36811d/> update rust crate serde_json to 1.0.92
 - <csr-id-0be62e8547f76508f9f14f949958b8529ae96b39/> update rust crate anyhow to 1.0.69
 - <csr-id-ca0a50d711f8c504f2ad9671929abc663639264b/> expose correct route on frontend api
 - <csr-id-2b0f8320e4120b8451ddd004b8c83b1c8b9193bc/> features get refreshed.
   Previously our spin loop slept for 15 seconds and then hit the await on
   the channel for a new token to validate.
   This PR changes that to use tokio::select to either refresh features for
   known tokens each 10th second, or receive a new token to validate.
   
   Should allow us to use more than one token and get them refreshed
 - <csr-id-5593376c3a89b28df6b6a8be2c93c1dc38a30c89/> allow any on CORS
 - <csr-id-93b0f22802f3fb16ac97174ccf8dc2574dafb9e0/> make sure reqwest does not bring along openssl
 - <csr-id-46a10d229bf2ccfd03f367a8e34e6f7f9f148013/> update rust crate tokio to 1.25.0
 - <csr-id-be9428d76742a3f5b2436b8b5cb61374609b98c3/> update rust crate unleash-yggdrasil to 0.4.2
 - <csr-id-71a9a2372d2e5110b628fe30438cf5b6760c8899/> patch the way CORS headers are done, without this, the server crashes on startup with an unhelpful error message
 - <csr-id-4b9e889a3d42089f206b62b9eea45dcfd8bae2f3/> update rust crate clap to 4.1.4
 - <csr-id-02e201b5142e6b95ced38f3636d3015ce4f79e03/> Update unleash-types to 0.5.1
 - <csr-id-fa8e9610dc74dd6868e36cdb6d2ae46c3aa17303/> update rust crate unleash-yggdrasil to 0.4.0
 - <csr-id-9f817bd7f0039315ad40aa61319c6ff1543b5241/> update rust crate clap to 4.1.3
 - <csr-id-042ae381536614d76f387c8d24b82c9ed9cb93bc/> update rust crate actix-web to 4.3.0

### Other

 - <csr-id-76e8e2a8d6e71bd1cf8920e00ce2373da9054a8e/> move obvious debug level logging to debug
 - <csr-id-45d6b6641c941e391a16df3294427efe64863c3c/> Subsume keys to check
   This collapses the keys seen. Removing keys that have been subsumed by a
   wider key (a key that includes the same projects or more as existing
   keys).
 - <csr-id-749b3ad08de04644d0182d891e4f097dc0c438f5/> token validator
   * task: add token validator
 - <csr-id-d32e20bebc02fcc40670f508c86ab37ee8967b5f/> Updated to only refresh tokens of type Client
 - <csr-id-bcc20510714f9c48985367e00fbd2eb6124e669a/> update to include openapi and hashes feature of types
 - <csr-id-b618ff1b1cd3ea30d2705b21db31be042d89309f/> added etag middleware
 - <csr-id-8f6fa05435caae5cdc112fefa187b8e0681df2dd/> Added prometheus metrics from shadow

### Style

 - <csr-id-2d99d7e01e602185337f79529aba9f9fd86cd634/> fix formatting

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 57 commits contributed to the release over the course of 25 calendar days.
 - 53 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 47 unique issues were worked on: [#10](https://github.com/Unleash/unleash-edge/issues/10), [#12](https://github.com/Unleash/unleash-edge/issues/12), [#13](https://github.com/Unleash/unleash-edge/issues/13), [#14](https://github.com/Unleash/unleash-edge/issues/14), [#15](https://github.com/Unleash/unleash-edge/issues/15), [#16](https://github.com/Unleash/unleash-edge/issues/16), [#17](https://github.com/Unleash/unleash-edge/issues/17), [#18](https://github.com/Unleash/unleash-edge/issues/18), [#20](https://github.com/Unleash/unleash-edge/issues/20), [#22](https://github.com/Unleash/unleash-edge/issues/22), [#23](https://github.com/Unleash/unleash-edge/issues/23), [#25](https://github.com/Unleash/unleash-edge/issues/25), [#26](https://github.com/Unleash/unleash-edge/issues/26), [#27](https://github.com/Unleash/unleash-edge/issues/27), [#28](https://github.com/Unleash/unleash-edge/issues/28), [#29](https://github.com/Unleash/unleash-edge/issues/29), [#3](https://github.com/Unleash/unleash-edge/issues/3), [#30](https://github.com/Unleash/unleash-edge/issues/30), [#33](https://github.com/Unleash/unleash-edge/issues/33), [#34](https://github.com/Unleash/unleash-edge/issues/34), [#36](https://github.com/Unleash/unleash-edge/issues/36), [#37](https://github.com/Unleash/unleash-edge/issues/37), [#38](https://github.com/Unleash/unleash-edge/issues/38), [#39](https://github.com/Unleash/unleash-edge/issues/39), [#4](https://github.com/Unleash/unleash-edge/issues/4), [#40](https://github.com/Unleash/unleash-edge/issues/40), [#41](https://github.com/Unleash/unleash-edge/issues/41), [#42](https://github.com/Unleash/unleash-edge/issues/42), [#43](https://github.com/Unleash/unleash-edge/issues/43), [#44](https://github.com/Unleash/unleash-edge/issues/44), [#45](https://github.com/Unleash/unleash-edge/issues/45), [#46](https://github.com/Unleash/unleash-edge/issues/46), [#5](https://github.com/Unleash/unleash-edge/issues/5), [#52](https://github.com/Unleash/unleash-edge/issues/52), [#53](https://github.com/Unleash/unleash-edge/issues/53), [#54](https://github.com/Unleash/unleash-edge/issues/54), [#55](https://github.com/Unleash/unleash-edge/issues/55), [#56](https://github.com/Unleash/unleash-edge/issues/56), [#57](https://github.com/Unleash/unleash-edge/issues/57), [#58](https://github.com/Unleash/unleash-edge/issues/58), [#59](https://github.com/Unleash/unleash-edge/issues/59), [#6](https://github.com/Unleash/unleash-edge/issues/6), [#60](https://github.com/Unleash/unleash-edge/issues/60), [#61](https://github.com/Unleash/unleash-edge/issues/61), [#62](https://github.com/Unleash/unleash-edge/issues/62), [#8](https://github.com/Unleash/unleash-edge/issues/8), [#9](https://github.com/Unleash/unleash-edge/issues/9)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#10](https://github.com/Unleash/unleash-edge/issues/10)**
    - use subcommands rather than ValueEnum ([`8fe7cab`](https://github.com/Unleash/unleash-edge/commit/8fe7cabbb496c34618cae77e82ddceeeb8cfb617))
 * **[#12](https://github.com/Unleash/unleash-edge/issues/12)**
    - add basic proxy endpoints and related test code ([`5f55517`](https://github.com/Unleash/unleash-edge/commit/5f55517e4407a7acf4b7906d82eee737bb58a53d))
 * **[#13](https://github.com/Unleash/unleash-edge/issues/13)**
    - update rust crate clap to 4.1.4 ([`4b9e889`](https://github.com/Unleash/unleash-edge/commit/4b9e889a3d42089f206b62b9eea45dcfd8bae2f3))
 * **[#14](https://github.com/Unleash/unleash-edge/issues/14)**
    - patch the way CORS headers are done, without this, the server crashes on startup with an unhelpful error message ([`71a9a23`](https://github.com/Unleash/unleash-edge/commit/71a9a2372d2e5110b628fe30438cf5b6760c8899))
 * **[#15](https://github.com/Unleash/unleash-edge/issues/15)**
    - internal backstage build info endpoint ([`0469918`](https://github.com/Unleash/unleash-edge/commit/0469918e24763a5fef41a706f6f88fde986f955d))
 * **[#16](https://github.com/Unleash/unleash-edge/issues/16)**
    - add client for getting features ([`9e99f4b`](https://github.com/Unleash/unleash-edge/commit/9e99f4b64b3d53b2e79381a2cb0d80ef4b010b2b))
 * **[#17](https://github.com/Unleash/unleash-edge/issues/17)**
    - update rust crate unleash-yggdrasil to 0.4.2 ([`be9428d`](https://github.com/Unleash/unleash-edge/commit/be9428d76742a3f5b2436b8b5cb61374609b98c3))
 * **[#18](https://github.com/Unleash/unleash-edge/issues/18)**
    - add enabled toggles routes ([`92aa64b`](https://github.com/Unleash/unleash-edge/commit/92aa64bc58e4193adc95370e651579feddea2811))
 * **[#20](https://github.com/Unleash/unleash-edge/issues/20)**
    - Added prometheus metrics from shadow ([`8f6fa05`](https://github.com/Unleash/unleash-edge/commit/8f6fa05435caae5cdc112fefa187b8e0681df2dd))
 * **[#22](https://github.com/Unleash/unleash-edge/issues/22)**
    - added etag middleware ([`b618ff1`](https://github.com/Unleash/unleash-edge/commit/b618ff1b1cd3ea30d2705b21db31be042d89309f))
 * **[#23](https://github.com/Unleash/unleash-edge/issues/23)**
    - update rust crate tokio to 1.25.0 ([`46a10d2`](https://github.com/Unleash/unleash-edge/commit/46a10d229bf2ccfd03f367a8e34e6f7f9f148013))
 * **[#25](https://github.com/Unleash/unleash-edge/issues/25)**
    - Implement redis datasource ([`0b2537f`](https://github.com/Unleash/unleash-edge/commit/0b2537f4bd397c666d458589bf30f9322b0c9214))
 * **[#26](https://github.com/Unleash/unleash-edge/issues/26)**
    - update README ([`1677111`](https://github.com/Unleash/unleash-edge/commit/16771118dbfdb4fc2dd819564b9d3f3355154134))
 * **[#27](https://github.com/Unleash/unleash-edge/issues/27)**
    - fix formatting ([`2d99d7e`](https://github.com/Unleash/unleash-edge/commit/2d99d7e01e602185337f79529aba9f9fd86cd634))
 * **[#28](https://github.com/Unleash/unleash-edge/issues/28)**
    - improve tests for redis provider ([`ea8cd1b`](https://github.com/Unleash/unleash-edge/commit/ea8cd1ba7fb36afb039f31ec4ba000a2b7271700))
 * **[#29](https://github.com/Unleash/unleash-edge/issues/29)**
    - implement an in memory data store ([`5ae644c`](https://github.com/Unleash/unleash-edge/commit/5ae644c8e4c98c588111a7461f359439c994209f))
 * **[#3](https://github.com/Unleash/unleash-edge/issues/3)**
    - Adds client features endpoint ([`4bf25a3`](https://github.com/Unleash/unleash-edge/commit/4bf25a3402c8e9a3c48c63118da1469a69a3bbdd))
 * **[#30](https://github.com/Unleash/unleash-edge/issues/30)**
    - implement simplify tokens ([`eab0878`](https://github.com/Unleash/unleash-edge/commit/eab0878ce2bf49a499f032a13c47f58a4b346cc7))
 * **[#33](https://github.com/Unleash/unleash-edge/issues/33)**
    - move server startup and traits to async ([`e58f4fc`](https://github.com/Unleash/unleash-edge/commit/e58f4fc3306ae71c1bcb8e8704d38eeb176cac96))
 * **[#34](https://github.com/Unleash/unleash-edge/issues/34)**
    - adds a call for validating tokens ([`0d037ec`](https://github.com/Unleash/unleash-edge/commit/0d037ec243b120f093b5a20efb3c5ddda6e25767))
 * **[#36](https://github.com/Unleash/unleash-edge/issues/36)**
    - Feat/implement data sync ([`862ee28`](https://github.com/Unleash/unleash-edge/commit/862ee288eab20367c5d4e487ddd679f72174e8ef))
 * **[#37](https://github.com/Unleash/unleash-edge/issues/37)**
    - allow any on CORS ([`5593376`](https://github.com/Unleash/unleash-edge/commit/5593376c3a89b28df6b6a8be2c93c1dc38a30c89))
 * **[#38](https://github.com/Unleash/unleash-edge/issues/38)**
    - features get refreshed. ([`2b0f832`](https://github.com/Unleash/unleash-edge/commit/2b0f8320e4120b8451ddd004b8c83b1c8b9193bc))
 * **[#39](https://github.com/Unleash/unleash-edge/issues/39)**
    - test auto-assign-pr action ([`286dfd5`](https://github.com/Unleash/unleash-edge/commit/286dfd536ff1c5d865829dcd98bda49da6ad9d36))
 * **[#4](https://github.com/Unleash/unleash-edge/issues/4)**
    - Add edge-token extractor to lock down access ([`e6bc817`](https://github.com/Unleash/unleash-edge/commit/e6bc817c21affd7e06883a9d56f85f254878a4c8))
 * **[#40](https://github.com/Unleash/unleash-edge/issues/40)**
    - switch to backing with HashMap<TokenString, EdgeToken> ([`3a8cd76`](https://github.com/Unleash/unleash-edge/commit/3a8cd761a8cd92696c9229df1a6c3614aae261fa))
 * **[#41](https://github.com/Unleash/unleash-edge/issues/41)**
    - expose correct route on frontend api ([`ca0a50d`](https://github.com/Unleash/unleash-edge/commit/ca0a50d711f8c504f2ad9671929abc663639264b))
 * **[#42](https://github.com/Unleash/unleash-edge/issues/42)**
    - update rust crate anyhow to 1.0.69 ([`0be62e8`](https://github.com/Unleash/unleash-edge/commit/0be62e8547f76508f9f14f949958b8529ae96b39))
 * **[#43](https://github.com/Unleash/unleash-edge/issues/43)**
    - update rust crate serde_json to 1.0.92 ([`cd86cdd`](https://github.com/Unleash/unleash-edge/commit/cd86cdd7c5f6a9a6577a10b01278e3b17e36811d))
 * **[#44](https://github.com/Unleash/unleash-edge/issues/44)**
    - Updated to only refresh tokens of type Client ([`d32e20b`](https://github.com/Unleash/unleash-edge/commit/d32e20bebc02fcc40670f508c86ab37ee8967b5f))
 * **[#45](https://github.com/Unleash/unleash-edge/issues/45)**
    - remove redis test that doesn't make sense anymore ([`ba72e09`](https://github.com/Unleash/unleash-edge/commit/ba72e090c400e7d2d7f276a89ecf79f3760c7c47))
 * **[#46](https://github.com/Unleash/unleash-edge/issues/46)**
    - redesign source/sink architecture ([`cdfa7c2`](https://github.com/Unleash/unleash-edge/commit/cdfa7c216c1b7066ab059259d319a8c8ce2dc82a))
 * **[#5](https://github.com/Unleash/unleash-edge/issues/5)**
    - update rust crate actix-web to 4.3.0 ([`042ae38`](https://github.com/Unleash/unleash-edge/commit/042ae381536614d76f387c8d24b82c9ed9cb93bc))
 * **[#52](https://github.com/Unleash/unleash-edge/issues/52)**
    - update rust crate serde_json to 1.0.93 ([`986a743`](https://github.com/Unleash/unleash-edge/commit/986a7433f687de3126cf05bf8d776cabf3a28290))
 * **[#53](https://github.com/Unleash/unleash-edge/issues/53)**
    - Task client metrics ([`81d49ef`](https://github.com/Unleash/unleash-edge/commit/81d49ef4c360a168a5c7445e56bab7e2cc78c020))
 * **[#54](https://github.com/Unleash/unleash-edge/issues/54)**
    - remove sinks for offline mode ([`9a34999`](https://github.com/Unleash/unleash-edge/commit/9a34999914d7c27b01b2ab7793863c8c139589fd))
 * **[#55](https://github.com/Unleash/unleash-edge/issues/55)**
    - update rust crate unleash-types to 0.8.2 ([`4f528b7`](https://github.com/Unleash/unleash-edge/commit/4f528b76b718405d151a06af6657376c9358a7a2))
 * **[#56](https://github.com/Unleash/unleash-edge/issues/56)**
    - update rust crate unleash-yggdrasil to 0.4.5 ([`2d4a743`](https://github.com/Unleash/unleash-edge/commit/2d4a74312db1e5adc0d042e52e47c4f7286a966d))
 * **[#57](https://github.com/Unleash/unleash-edge/issues/57)**
    - redesign source and sinks to store features by environment and filter the responses by project ([`869294b`](https://github.com/Unleash/unleash-edge/commit/869294b93591055b8b078943771915aef0bf33d8))
 * **[#58](https://github.com/Unleash/unleash-edge/issues/58)**
    - token validator ([`749b3ad`](https://github.com/Unleash/unleash-edge/commit/749b3ad08de04644d0182d891e4f097dc0c438f5))
 * **[#59](https://github.com/Unleash/unleash-edge/issues/59)**
    - Subsume keys to check ([`45d6b66`](https://github.com/Unleash/unleash-edge/commit/45d6b6641c941e391a16df3294427efe64863c3c))
 * **[#6](https://github.com/Unleash/unleash-edge/issues/6)**
    - update rust crate clap to 4.1.3 ([`9f817bd`](https://github.com/Unleash/unleash-edge/commit/9f817bd7f0039315ad40aa61319c6ff1543b5241))
 * **[#60](https://github.com/Unleash/unleash-edge/issues/60)**
    - add edge mode ([`e6fd6c5`](https://github.com/Unleash/unleash-edge/commit/e6fd6c5fda8adea94f06eaaf10033e9ae9a194a3))
 * **[#61](https://github.com/Unleash/unleash-edge/issues/61)**
    - Open api docs ([`49d7129`](https://github.com/Unleash/unleash-edge/commit/49d7129a02f9ff8d9a336db9718593396742bb0d))
 * **[#62](https://github.com/Unleash/unleash-edge/issues/62)**
    - update rust crate unleash-types to 0.8.3 ([`eea450a`](https://github.com/Unleash/unleash-edge/commit/eea450a47bfe5c32ea84994570223c1d5a746bc8))
 * **[#8](https://github.com/Unleash/unleash-edge/issues/8)**
    - update rust crate unleash-yggdrasil to 0.4.0 ([`fa8e961`](https://github.com/Unleash/unleash-edge/commit/fa8e9610dc74dd6868e36cdb6d2ae46c3aa17303))
 * **[#9](https://github.com/Unleash/unleash-edge/issues/9)**
    - Added cors middleware ([`3addbd6`](https://github.com/Unleash/unleash-edge/commit/3addbd639c12749c5d18775f95b1bfede106c4cf))
 * **Uncategorized**
    - added team developer to save spam ([`004aa95`](https://github.com/Unleash/unleash-edge/commit/004aa955e8bed7687090762efa0bcc53577ecf2c))
    - move obvious debug level logging to debug ([`76e8e2a`](https://github.com/Unleash/unleash-edge/commit/76e8e2a8d6e71bd1cf8920e00ce2373da9054a8e))
    - tokens are now used ([`b18c039`](https://github.com/Unleash/unleash-edge/commit/b18c039255180c8d18e786e783a40f5cf9724358))
    - make sure reqwest does not bring along openssl ([`93b0f22`](https://github.com/Unleash/unleash-edge/commit/93b0f22802f3fb16ac97174ccf8dc2574dafb9e0))
    - update to include openapi and hashes feature of types ([`bcc2051`](https://github.com/Unleash/unleash-edge/commit/bcc20510714f9c48985367e00fbd2eb6124e669a))
    - bump unleash-types ([`9132cc1`](https://github.com/Unleash/unleash-edge/commit/9132cc1410d1d4a14e08de15ee53c9fce1fc5c92))
    - Update unleash-types to 0.5.1 ([`02e201b`](https://github.com/Unleash/unleash-edge/commit/02e201b5142e6b95ced38f3636d3015ce4f79e03))
    - Update cargo keys with ownership and license ([`1d6a518`](https://github.com/Unleash/unleash-edge/commit/1d6a5188a6334b341db72f847f55450726da3bee))
    - add /api/client/features endpoint ([`c270685`](https://github.com/Unleash/unleash-edge/commit/c270685a08207e0ab283e563ad6f58ad4f859161))
    - Server with metrics and health check ready ([`231efc3`](https://github.com/Unleash/unleash-edge/commit/231efc30353f6af6f20b8431220101802ca5c2b3))
</details>
