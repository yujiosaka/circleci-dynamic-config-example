# circleci-dynamic-config-example

An example app in Rust for CircleCI's [dynamic config](https://circleci.com/docs/dynamic-config) demo.

Dynamic config allows you to dynamically generate CircleCI's config depending on dynamic conditions.

This is useful especially for monorepos because you don't want to trigger all CircleCI workflows on every push.

## About the example app

This is a client-server application in Rust.

The client code is also written in Rust and it generates [WebAssembly](https://developer.mozilla.org/ja/docs/WebAssembly) files.

Here is the code structure:

<img width="290" alt="Screen Shot 2022-10-05 at 14 30 11" src="https://user-images.githubusercontent.com/2261067/193988105-033feb49-f1c4-4f38-afff-ad41f6645737.png">

Both `client` and `server` directories depends on the Rust library in `shared` directory.

Therefore we want to trigger CircleCI wofkflows under the following conditions:

- Trigger `client` job when a new commit is pushed in `client` directory
- Trigger `server` job when a new commit is pushed in `server` directory
- Trigger all jobs when a new commit is pushed in `shared` directory

## Setup

First of all, you need to enable dynamic config in CircleCI's project setting.

Go to `[Project Settings]` > `[Advanced]` and turn on `Enable dynamic config using setup workflows`.

<img width="994" alt="Screen Shot 2022-10-05 at 14 34 02" src="https://user-images.githubusercontent.com/2261067/193988541-3301ce53-3d61-475e-bfac-5e5c13ef3a8f.png">

You have to use CircleCI version 2.1 at the top of your .circleci/config.yml file.

```yml
version: 2.1
```

Also you need to add following lines in `.circleci/config.yml` to use dynamic config for the workflow.

```yml
setup: true
```

## Configuration

For the monorepo use case, we have a [path-filtering](https://circleci.com/developer/ja/orbs/orb/circleci/path-filtering) orb already available.

In order to use the orb, add the orbs stanza below your version.

```yml
orbs:
  path-filtering: circleci/path-filtering@0.1.3
```

Then you can map the updated files and directory to pipeline parameters

```yml
workflows:
  always-run:
    jobs:
      - path-filtering/filter:
          name: check-updated-files
          mapping: |
            client/.* run-build-client-job true
            server/.* run-build-server-job true
            shared/.* run-build-shared-job true
          base-revision: main
          config-path: .circleci/continue_config.yml
```

Note that the file specified in `config-path` is the dynamic config conditionally triggered.
Then you can create `.circleci/continue_config.yml` and map the parameters like below

```yml
parameters:
  run-build-client-job:
    type: boolean
    default: false
  run-build-server-job:
    type: boolean
    default: false
  run-build-shared-job:
    type: boolean
    default: false
```

Finally you can declare workflows based on the conditions of the parameters.

```yml
workflows:
  client:
    when:
      or:
        [
          << pipeline.parameters.run-build-client-job >>,
          << pipeline.parameters.run-build-shared-job >>,
        ]
    jobs:
      - rust/lint-test-build:
          with_cache: false
          release: true
          version: 1.64.0
          working_directory: client
  server:
    when:
      or:
        [
          << pipeline.parameters.run-build-server-job >>,
          << pipeline.parameters.run-build-shared-job >>,
        ]
    jobs:
      - rust/lint-test-build:
          with_cache: false
          release: true
          version: 1.64.0
          working_directory: server
  shared:
    when: << pipeline.parameters.run-build-shared-job >>
    jobs:
      - rust/lint-test-build:
          with_cache: false
          release: true
          version: 1.64.0
          working_directory: shared
```

See [.circleci](https://github.com/yujiosaka/circleci-dynamic-config-example/blob/main/.circleci) directory for the complete example.

# Trigger pipelines

You can add a file under `server` and push the commit.

```bash
$ cd server
$ touch test
$ git add test
$ git commit -m "feat: add test file to server"
$ git push
```

You can see only the `server` workflow is triggered.

<img width="1233" alt="Screen Shot 2022-10-05 at 14 48 27" src="https://user-images.githubusercontent.com/2261067/193990258-f669e235-f57d-4df3-956f-c0b31fb0e425.png">

You can also try adding a file under `shared` and push the commit.

```bash
$ cd shared
$ touch test
$ git add test
$ git commit -m "feat: add test file to shared"
$ git push
```

This time you can see all `server`, `client` and `shared` workflows are triggered.

<img width="1238" alt="Screen Shot 2022-10-05 at 14 48 37" src="https://user-images.githubusercontent.com/2261067/193990261-c3801c4a-f363-444f-8366-4d3c608e22d2.png">

# Play

All Rust/Cargo command are wrapped by [npm](https://www.npmjs.com/) so please install npm packages first.

```bash
$ npm install
```

Then you can simply start server.

```bash
$ npm start
```

Open [localhost:8080](http://localhost:8080) you can test functions on both Server and Web Assembly

<img width="507" alt="Screen Shot 2022-10-05 at 14 42 39" src="https://user-images.githubusercontent.com/2261067/193989509-71668908-bb3f-4178-aa6c-0615fae04b49.png">
