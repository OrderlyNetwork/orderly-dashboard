import { ArrowLeftIcon } from '@radix-ui/react-icons';
import { LoaderFunctionArgs } from '@remix-run/node';
import { json, Link, useLoaderData } from '@remix-run/react';
import { FC, useMemo } from 'react';

import { Spinner } from '~/components';
import { MarketDetailView } from '~/components/MarketDetail';
import { useSymbols } from '~/hooks';

export function loader({ params }: LoaderFunctionArgs) {
  return json({ symbol: params.symbol ?? '' });
}

export const MarketDetailRoute: FC = () => {
  const { symbol: rawSymbol } = useLoaderData<typeof loader>();
  const symbols = useSymbols();

  const fullSymbol = useMemo(() => {
    if (!symbols) return null;
    // rawSymbol is the friendly base token (e.g. "BTC")
    // Find the matching full symbol (e.g. "PERP_BTC_USDC")
    const match = symbols.find((s) => {
      const parts = s.symbol.split('_');
      const baseToken = parts.length >= 2 ? parts[1] : s.symbol;
      return baseToken.toUpperCase() === rawSymbol.toUpperCase();
    });
    return match?.symbol ?? null;
  }, [symbols, rawSymbol]);

  if (!symbols) {
    return (
      <div className="flex justify-center py-12">
        <Spinner size="2.5rem" />
      </div>
    );
  }

  if (!fullSymbol) {
    return (
      <div className="flex flex-col items-center justify-center gap-4 p-8 text-center">
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

  return <MarketDetailView symbol={fullSymbol} baseToken={rawSymbol} />;
};

export default MarketDetailRoute;
