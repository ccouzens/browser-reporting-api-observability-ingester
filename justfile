format: format-biome format-prettier
    
format-biome:
    pnpm dlx @biomejs/biome@2.4.15 check --write

format-prettier:
    pnpm dlx prettier@3.8.3 . --write '**/*.md'
