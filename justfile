format: format-biome format-just format-prettier format-superhtml

format-biome:
    biome check --write

format-just:
    just --fmt

format-prettier:
    prettier --write '**/*.md'

format-superhtml:
    superhtml fmt projects/site-with-violations/*.html

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
