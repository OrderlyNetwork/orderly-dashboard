import { ArrowLeftIcon } from '@radix-ui/react-icons';
import { LoaderFunctionArgs } from '@remix-run/node';
import { json, Link, useLoaderData } from '@remix-run/react';
import { FC, useMemo } from 'react';

import { Spinner } from '~/components';
import { MarketDetailView } from '~/components/MarketDetail';
import { useSymbols } from '~/hooks';
import { getBaseToken, getBroker } from '~/hooks/useSymbols';

export function loader({ params }: LoaderFunctionArgs) {
  return json({ symbol: params.symbol ?? '' });
}

export const MarketDetailRoute: FC = () => {
  const { symbol: rawSymbol } = useLoaderData<typeof loader>();
  const symbols = useSymbols();

  const resolved = useMemo(() => {
    if (!symbols) return null;
    // rawSymbol is the URL slug: either `BTC` (canonical) or `BTC_mythos`
    // (broker variant). Split off an optional broker suffix.
    const slugParts = rawSymbol.split('_');
    const baseFromSlug = slugParts[0];
    const slugBroker = slugParts.length > 1 ? slugParts.slice(1).join('_') : null;

    const baseMatches = symbols.filter((s) => {
      const base = getBaseToken(s.symbol);
      return base.toUpperCase() === baseFromSlug.toUpperCase();
    });

    if (baseMatches.length === 0) return null;

    // If the slug includes a broker suffix, pin to that exact broker.
    if (slugBroker) {
      const withBroker = baseMatches.find(
        (s) => (getBroker(s.symbol) ?? '').toLowerCase() === slugBroker.toLowerCase()
      );
      return withBroker ?? null;
    }

    // No broker in URL: prefer the canonical (no-broker) symbol.
    const canonical = baseMatches.find((s) => getBroker(s.symbol) === null);
    if (canonical) return canonical;

    // No canonical exists — fall back to the first broker variant so the URL
    // is still useful. Warn so the issue is discoverable.
    if (baseMatches.length > 1) {
      console.warn(
        `[markets_.$symbol] /markets/${rawSymbol} matched ${baseMatches.length} brokered markets; falling back to first match. Use /markets/BASE_broker to disambiguate.`
      );
    }
    return baseMatches[0];
  }, [symbols, rawSymbol]);

  if (!symbols) {
    return (
      <div className="flex justify-center py-12" data-page-status="loading">
        <Spinner size="2.5rem" />
      </div>
    );
  }

  if (!resolved) {
    return (
      <div
        className="flex flex-col items-center justify-center gap-4 p-8 text-center"
        data-page-status="not-found"
      >
        <div className="text-xl font-semibold text-gray-300">Market Not Found</div>
        <div className="text-gray-500 max-w-md">
          No market found for symbol{' '}
          <code className="bg-gray-800 px-2 py-1 rounded text-sm" style={{ color: '#D4B2FF' }}>
            {rawSymbol}
          </code>
        </div>
        <Link
          to="/markets"
          className="inline-flex items-center gap-2 text-gray-400 hover:text-white transition-colors duration-200 no-underline mt-4"
        >
          <ArrowLeftIcon width="16" height="16" />
          <span className="text-sm">Back to Markets</span>
        </Link>
      </div>
    );
  }

  return <MarketDetailView symbol={resolved.symbol} baseToken={getBaseToken(resolved.symbol)} />;
};

export default MarketDetailRoute;
