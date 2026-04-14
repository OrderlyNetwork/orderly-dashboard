# Fonts

Place your ATYP font files in this directory.

## Required files

The poster template looks for these font weights:

| File (example)               | Weight | Usage              |
|------------------------------|--------|--------------------|
| `ATYP-Light.ttf`             | 300    | Timestamp          |
| `ATYP-Regular.ttf`           | 400    | Column headers     |
| `ATYP-Medium.ttf`            | 500    | X Account names    |
| `ATYP-Bold.ttf`              | 700    | Title, Points      |
| `ATYP-ExtraBold.ttf`         | 800    | Rank numbers       |

`.otf` files are also supported — update the `src` paths in `poster_template.html` if your filenames differ.

## Without the font

If no ATYP files are present, the poster falls back to `'Courier New', monospace`. The layout will still render correctly.
