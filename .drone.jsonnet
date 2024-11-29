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
    'CARGO_TARGET_DIR=windows cargo build --release --target $TARGET',
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
    'CARGO_TARGET_DIR=mac cargo build --release --target $TARGET',
  ],
};

local build_it() = [
  build_windows(),
  build_mac(),
];

local binaries = [
  'artifacts/rmute-teams-mac',
  'artifacts/rmute-teams-windows.exe',
];

local package(version) = {
  name: 'prerelease',
  image: 'gitea.pb42.de/matthias/drone-gitea-package',
  settings: {
    user: { from_secret: 'gitea-username' },
    token: { from_secret: 'gitea-token' },
    file: binaries,
    version: version,
  },
  depends_on: ['build-mac', 'build-windows'],
};

local release_pipeline = {
  kind: 'pipeline',
  type: 'docker',
  name: 'release-pipeline',
  steps:
    build_it() +
    [
      package('${DRONE_TAG}'),
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
    build_it() +
    [
      package('main'),
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
    build_it() +
    [
      package('pr-${DRONE_PULL_REQUEST}'),
    ],
  trigger: {
    event: { include: ['pull_request'] },
    branch: { include: ['main'] },
  },
  image_pull_secrets: ['dockerconfigjson'],
};

[
  pr_pipeline,
  main_pipeline,
  release_pipeline,
]
