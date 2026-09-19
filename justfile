format: format-biome format-prettier format-just

format-biome:
    biome check --write

format-prettier:
    prettier --write '**/*.md'

format-just:
    just --fmt

[working-directory('projects/site-with-violations')]
site-with-violations-dependencies:
    pwd
    pnpm i --frozen-lockfile

[working-directory('./projects/site-with-violations')]
site-with-violations-build: site-with-violations-dependencies
    pnpm build

[working-directory('./projects/site-with-violations')]
site-with-violations: site-with-violations-build
    pnpm preview 
