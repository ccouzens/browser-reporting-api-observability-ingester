[parallel]
format: format-biome format-just format-superhtml format-toml observability-server-format

[parallel]
typecheck: site-with-violations-typecheck exploratory-server-typecheck

format-biome:
    biome check --write

format-just:
    just --fmt

format-superhtml:
    superhtml fmt projects/site-with-violations/*.html

format-toml:
    tombi format .    

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
    tsc

[working-directory('./projects/site-with-violations')]
site-with-violations: site-with-violations-dependencies
    pnpm exec vite build
    pnpm exec vite preview

[working-directory('projects/exploratory-server')]
exploratory-server-typecheck:
    pnpm i --frozen-lockfile
    tsc

[working-directory('projects/exploratory-server')]
exploratory-server:
    node src/main.mts
