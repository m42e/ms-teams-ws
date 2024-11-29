local build_windows() = {
  name: 'build-windows',
  image: 'gitea.pb42.de/mutenix/build-docker:windows-v1.0.0',
  environment: {
    TARGET: 'x86_64-pc-windows-gnu',
  },
  commands: [
    'HOME=/root/ . "/root/.cargo/env"',
    'rustup default stable',
    'rustup target add $TARGET',
    'CARGO_TARGET_DIR=build/windows cargo build --release --target $TARGET',
  ],
};

local build_mac() = {
  name: 'build-mac',
  image: 'gitea.pb42.de/mutenix/build-docker:macos-v1.0.0',
  environment: {
    TARGET: 'x86_64-apple-darwin',
    CC: 'o64-clang',
    CXX: 'o64-clang++',
  },
  commands: [
    '. "/root/.cargo/env"',
    'rustup target add $TARGET',
    'CARGO_TARGET_DIR=build/mac cargo build --release --target $TARGET',
  ],
};

local build_it() = [
  build_windows(),
  build_mac(),
];

local publish_gitea() = {
  name: 'publish-gitea',
  image: 'rust:1.82',
  environment: {
    GITEA_TOKEN: { from_secret: 'gitea-token' },
  },
  settings: {
    user: { from_secret: 'gitea-username' },
    token: { from_secret: 'gitea-token' },
  },
  commands: [
    'export CARGO_REGISTRIES_GITEA_INDEX="sparse+https://gitea.pb42.de/api/packages/mutenix/cargo/"',
    'export CARGO_REGISTRIES_GITEA_TOKEN="Bearer $${GITEA_TOKEN}"',
    'export CARGO_REGISTRY_DEFAULT=gitea',
    'cargo publish',
  ],
};

local release_pipeline = {
  kind: 'pipeline',
  type: 'docker',
  name: 'release-pipeline',
  steps:
    build_it() +
    [
      publish_gitea(),
    ],
  trigger: {
    event: { include: ['tag'] },
    refs: { include: ['refs/tags/v*'] },
  },
  image_pull_secrets: ['dockerconfigjson'],
};

local main_pipeline = {
  kind: 'pipeline',
  type: 'docker',
  name: 'main-pipeline',
  steps:
    [
      publish_gitea(),
    ],
  trigger: {
    event: { include: ['push'] },
    branch: { include: ['main'] },
  },
  image_pull_secrets: ['dockerconfigjson'],
};

local pr_pipeline = {
  kind: 'pipeline',
  type: 'docker',
  name: 'pr-pipeline',
  steps:
    build_it(),
  trigger: {
    event: { include: ['pull_request'] },
    branch: { include: ['main'] },
  },
  image_pull_secrets: ['dockerconfigjson'],
};

[
  pr_pipeline,
  main_pipeline,
]
