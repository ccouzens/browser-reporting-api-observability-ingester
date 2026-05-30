format: format-biome format-prettier format-just

format-biome:
    pnpm dlx @biomejs/biome@2.4.15 check --write

format-prettier:
    pnpm dlx prettier@3.8.3 --write '**/*.md'

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
