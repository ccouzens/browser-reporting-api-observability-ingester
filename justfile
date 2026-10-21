[parallel]
format: format-biome format-just format-superhtml observability-server-format

[parallel]
typecheck: site-with-violations-typecheck exploratory-server-typecheck

format-biome:
    biome check --write

format-just:
    just --fmt

format-superhtml:
    superhtml fmt projects/site-with-violations/*.html

https-server:
    cloudflared tunnel --url http://localhost:4173

[working-directory('projects/observability-server')]
observability-server-format:
    cargo fmt

[working-directory('projects/observability-server')]
observability-server:
    cargo run

[parallel]
site-with-violations-and-exploratory-server: site-with-violations exploratory-server

[parallel]
site-with-violations-and-observability-server: site-with-violations observability-server

[working-directory('projects/site-with-violations')]
site-with-violations-dependencies:
    pnpm i --frozen-lockfile

[working-directory('./projects/site-with-violations')]
site-with-violations-typecheck: site-with-violations-dependencies
    pnpm typecheck

[working-directory('./projects/site-with-violations')]
site-with-violations-build: site-with-violations-dependencies
    pnpm build

[working-directory('./projects/site-with-violations')]
site-with-violations: site-with-violations-build
    pnpm preview

[working-directory('projects/exploratory-server')]
exploratory-server-dependencies:
    pwd
    pnpm i --frozen-lockfile

[working-directory('projects/exploratory-server')]
exploratory-server-typecheck: exploratory-server-dependencies
    pnpm typecheck

[working-directory('projects/exploratory-server')]
exploratory-server: exploratory-server-dependencies
    pnpm preview
