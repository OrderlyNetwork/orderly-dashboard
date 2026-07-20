# Orderly Dashboard FE

Remix web app that visualizes analytics from the Orderly Network (perpetual futures).
Part of a larger Rust/TS monorepo (`orderly-dashboard-{indexer,analyzer,query-service,FE}`).
The other three components are Rust services; this is the only TypeScript app.

## Stack

Remix 2.9 + Vite + Express (custom `server.js`) · React 18 · TypeScript 5.4.5 strict ·
SWR (all data fetching) · UnoCSS · Radix Themes v3 + Mantine v8 + MUI v7 (coexist) ·
chart.js + chartjs-chart-financial + react-chartjs-2 · @tanstack/react-table + react-virtual ·
ts-pattern (versioned event unions) · dayjs · motion · @tarnadas/fixed-number ·
@ethersproject/keccak256 (symbol hashing).

## Commands

```sh
yarn dev          # node ./server.js  (Express wrapping Vite middleware — NOT `vite dev`)
yarn build        # remix vite:build
yarn start        # NODE_ENV=production node ./server.js
yarn lint         # eslint --cache
yarn typecheck    # tsc  (noEmit; Vite does the bundling)
yarn format       # prettier --write . && yarn eslint --fix .

cargo run -p orderly-dashboard-fe   # regenerate app/types/api.ts from Rust (run from this folder)
```

## Project layout

```
app/
  root.tsx                  # HTML shell, MUI/Radix/Mantine providers, env loader
  App.tsx                   # AppContext (queryServiceUrl, evmApiUrl) + embed/full layout switch
  entry.{client,server}.tsx # Remix entries (server uses Emotion SSR)
  routes/                   # Remix file-based routes (see "Routes" below)
  components/
    DashboardLayout.tsx     # Sidebar + Topbar + Outlet shell
    analytics/
      Sidebar.tsx           # Primary nav (dashboards/markets/leaderboard/explorer)
      views/                # DashboardsView, MarketsView, LeaderboardView, ExplorerView
      widgets/              # ~30 dashboard widgets (VolumeChart, TVL, FundingRates, ...)
      shared/               # formatters.ts, primitives.tsx (Period/Granularity selectors), chartConfig.ts
    MarketDetail/           # Per-symbol page: PriceChart, Orderbook, FundingChart, LiquidationHeatmap, ...
    {Markets,Leaderboard,WhaleLeaderboard,Positions,EventsTable,PnLStats,PortfolioChart,
     BrokerSelectionModal,TaxExportModal,SearchInput,Spinner}.tsx
  hooks/                    # SWR data hooks (one per resource)
    usePublicInfo.ts        # marketSummary, marketDetail, accountState, portfolio, topAddresses, whaleContext, ...
    useOrderlyMetrics.ts    # DATA_API analytics hooks (TVL, fees, builders, staking, distributors, ...)
    useEvents.ts            # /events_v2 with versioned v1/v2/v3 handling
    usePositions.ts useLeaderboard.ts useSymbols.ts useSearchAddress.ts useTokens.ts useBrokers.ts ...
  types/api.ts              # AUTO-GENERATED — do not hand-edit
  types/{dashboard,leaderboard,index}.ts
  utils/data-api.ts         # SSR loader helpers for DashboardsView
  util.ts                   # base64UrlSafeEncode/Decode for Solana address URLs
server.js  remix.config.js  vite.config.ts  unocss.config.ts  tsconfig.json  env.d.ts
```

Path alias: `~/*` → `app/*`.

## Environment variables

`vite.config.ts` sets `envPrefix: ['VITE_', 'DATA_API_']`.

| Var | Used via | Set in | Purpose |
|---|---|---|---|
| `QUERY_SERVICE_URL` | `useAppState().queryServiceUrl` | `.env` → `root.tsx` loader → `AppContext` | Dashboard Query Service (Rust) |
| `EVM_API_URL` | `useAppState().evmApiUrl` | `.env` → `root.tsx` loader → `AppContext` | Orderly EVM REST API |
| `DATA_API_URL` | `import.meta.env.DATA_API_URL` | `.env` (Vite inlines at build) | Orderly Data API (analytics) |

Defaults point to mainnet (`api-evm.orderly.network`, `orderly-dashboard-query-service.orderly.network`,
`data-api.orderly.network`); testnet values are commented in `.env`.

## Backend APIs

Three services are consumed. Every response uses the Orderly envelope `{ success, data, ... }` —
hooks always unwrap `.data` and throw on `success === false`.

### 1. Orderly EVM API (`EVM_API_URL`, e.g. `https://api-evm.orderly.network`)

Public Orderly perp-trading API. No auth.

REST endpoints used:
- `GET /v1/public/info` — list perp symbols (`useSymbols`)
- `GET /v1/public/info/{symbol}` — symbol info incl. `base_imr` (max leverage = `1/base_imr`)
- `GET /v1/public/futures/{symbol}` and `/v1/public/futures_market` — market snapshot
- `GET /v1/public/market_info/{price_changes,traders_open_interests,funding_history}`
- `GET /v1/public/insurancefund` — insurance fund balance + positions
- `GET /v1/public/account?account_id=...` — account_id → {address, broker_id}
- `GET /v1/get_all_accounts?address=...&broker_id=...&chain_type={EVM|SOL}` — all (broker, account) pairs
- `GET /v1/broker/leaderboard/daily` — trading leaderboard

Public Info endpoint `POST /v1/public/query` (body: `{ type, ...params }`) — single endpoint for:
`marketSummary` (w1), `marketDetail` (w2), `marketTrades` (w1), `orderbook` (w1), `candles` (w2),
`fundingRateHistory` (w2), `fundingComparison` (w2), `liquidations` (w2),
`topAddresses` (w10), `whaleContext` (w3), `platformPositions` (w20),
`accountState` (w5), `portfolio` (w5), `accounts` (w5), and more
(see `app/hooks/usePublicInfo.ts` for canonical wrappers and weight annotations).

### 2. Dashboard Query Service (`QUERY_SERVICE_URL`, e.g. `https://orderly-dashboard-query-service.orderly.network`)

Project's own Rust service. Indexed on-chain events and rankings.
- `POST /events_v2` — historical trading events for an `account_id`. Body: `{ account_id, event_type?, from_time?, to_time?, trading_event_next_cursor? }`.
  Response events come in **v1 / v2 / v3 variants** (`LiquidationResult{,V2,V3}`, `AdlResult{,V2,V3}`, `SettlementResult{,V3}`, `MarginTransferV3`). `useEvents.ts` flattens these with `ts-pattern` `.exhaustive()`. `event_type` ∈ `TRANSACTION | PERPTRADE | SETTLEMENT | LIQUIDATION | ADL | MARGINTRANSFER`. Time range is capped at 31 days (`MAX_TIME_RANGE_SECONDS`).
- `GET /ranking/positions` — open-positions leaderboard (`usePositions`)
- `GET /symbols` — every known symbol + its hash (`useAllSymbols`, fallback for symbols no longer active)

### 3. Orderly Data API (`DATA_API_URL`, e.g. `https://data-api.orderly.network`)

Separate analytics service. Read directly via `import.meta.env.DATA_API_URL` (not `AppContext`).
- `/orderly/api/v1/dashboard/orderly/{main, tvl-by-chain, by-symbol/{daily,weekly}, funding-rates}`
- `/orderly/api/v1/dashboard/{fund-flows/{by-broker,by-chain}, staking/daily}`
- `/orderly/api/v1/metrics/{dex-users, overview, volume-segments, stake-users, stake-vs-supply, omnivault-tvl}`
- `/orderly/api/v1/distributors/{stats, invitees}`

Canonical hooks: `app/hooks/useOrderlyMetrics.ts` and SSR helpers in `app/utils/data-api.ts`.

## Routes

| Path | File | Notes |
|---|---|---|
| `/` | `routes/_index.tsx` | Dashboards. SSR-loader fetches 90-day data via `data-api.ts`. |
| `/markets` | `routes/markets.tsx` | Markets table (`Markets.tsx`). |
| `/markets/:symbol` | `routes/markets_.$symbol.tsx` | `:symbol` is the **base token** (e.g. `BTC`), resolved client-side to full `PERP_BTC_USDC` via `useSymbols`. |
| `/leaderboard` | `routes/leaderboard.tsx` | Tabs: trading / positions / whales. |
| `/explorer` | `routes/explorer.tsx` | Address/account-ID search. |
| `/search?q=...&chain_namespace={evm|sol}` | `routes/search.tsx` | Lists all broker accounts for an address. |
| `/address/:address` | `routes/address_.$address.tsx` | Events / Positions tabs + Tax Export + Portfolio chart. |
| `/widget/:widgetId` | `routes/widget.$widgetId.tsx` | Single embeddable widget. `WIDGET_META` registry at line ~69. Supports `?embed=true` for chrome-less render. |

`App.tsx` switches layout: if path starts with `/widget` AND `?embed=true`, only the `<Outlet />` renders (no Sidebar/Topbar).

## Data-fetching conventions

- **SWR everywhere.** Standard options: `{ revalidateOnFocus: false, shouldRetryOnError: false, dedupingInterval: 60000, refreshInterval: 60000 }`. Live data (orderbook/trades) uses shorter intervals (5–15s).
- **Envelope unwrap.** Every API returns `{ success, data, ... }`. Hooks check `success`, throw on failure, return `data` only.
- **Base URLs** come from `useAppState()` (which reads `AppContext` populated by `root.tsx`'s server loader) — except `DATA_API_URL`, which is read via `import.meta.env` directly inside `useOrderlyMetrics.ts` and `utils/data-api.ts`.
- **Remix loaders** (server-side) use `process.env.QUERY_SERVICE_URL`/`process.env.EVM_API_URL` directly (see `root.tsx`); they forward these to the client via `AppContext`.

## Code style

- Prettier: single quotes, no trailing commas, 100-col, 2-space indent.
- ESLint: react + jsx-a11y + react-hooks + @typescript-eslint + import.
- Imports are **alphabetized within groups** with `newlines-between: always` — keep that ordering when adding imports.
- Unused vars must be `_`-prefixed (`varsIgnorePattern: '^_.*'`).
- `tsconfig.json` is strict, ES2022, Bundler resolution, `noEmit`.
- **Do not add comments** unless explicitly requested.

## Critical gotchas

1. **`app/types/api.ts` is auto-generated** from Rust via `typescript-type-def`. Never hand-edit; regenerate via `cargo run -p orderly-dashboard-fe` from this folder. The `types.TradingEventInnerData` union drives the versioned-event handling in `useEvents.ts`.
2. **Dev server is `node ./server.js`**, not `vite dev`. Express creates the Vite dev server in middleware mode and serves the Remix SSR build.
3. **Versioned trading events.** `/events_v2` returns `Settlement{,V3}`, `Liquidation{,V2,V3}`, `Adl{,V2,V3}`, `MarginTransferV3`. Any new variant MUST be added to the `ts-pattern` match in `app/hooks/useEvents.ts` (the `.exhaustive()` call will fail TypeScript compilation if a case is missing).
4. **Symbols are keccak256-hashed** on-chain. Use `getSymbolName`, `getSymbolBaseTick`, `getMaxFractionDigits` from `app/hooks/useSymbols.ts`. Active symbols come from `/v1/public/info`; the query-service `/symbols` is a fallback for delisted ones.
5. **Market URLs use the base token** (`/markets/BTC`), not the full `PERP_BTC_USDC` symbol. Resolution happens in `routes/markets_.$symbol.tsx`.
6. **Account IDs (`0x` + 64 hex) need a two-step resolve**: `/v1/public/account?account_id=...` → `{address, broker_id}`, then `/v1/get_all_accounts` for sub-accounts. See `routes/address_.$address.tsx:142`.
7. **Solana addresses** (43–44 base58 chars) are base64url-safe-encoded in URL paths. Use `base64UrlSafeEncode/Decode` from `app/util.ts`. `chain_namespace` is `'evm' | 'sol'`.
8. **SSR bundle quirks**: `remix.config.js` sets `serverDependenciesToBundle: ['@radix-ui/themes']`; `vite.config.ts` sets `ssr.noExternal: ['@mui/**/*']` in production only.
9. **Three UI libraries coexist** (Radix Themes, Mantine, MUI). Radix is the primary skin (dark mode, `accentColor="iris"`); Mantine is used for dates/inputs; MUI is wrapped in a dark `ThemeProvider`. Don't introduce a fourth.

## Useful file pointers

- `app/App.tsx:6` — `AppContextType` (`queryServiceUrl`, `evmApiUrl`)
- `app/root.tsx:67` — env-var loader → `AppContext.Provider`
- `app/components/DashboardLayout.tsx:8` — `PATH_TO_NAV` mapping (drives Sidebar active state)
- `app/components/analytics/Sidebar.tsx:113` — external links
- `app/components/analytics/shared/{formatters,primitives,chartConfig}` — reuse these before writing new format/chart helpers
- `app/hooks/usePublicInfo.ts` — canonical wrappers for Orderly EVM API + Public Info query
- `app/hooks/useEvents.ts:294` — versioned-event flattening (ts-pattern exhaustive)
- `app/hooks/useOrderlyMetrics.ts` — all `DATA_API_URL` analytics hooks
- `app/utils/data-api.ts` — SSR loaders for `DashboardsView`
- `app/routes/widget.$widgetId.tsx:69` — `WIDGET_META` registry (add new embeddable widgets here)
