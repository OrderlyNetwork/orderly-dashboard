import { Theme } from '@radix-ui/themes';
import radixTheme from '@radix-ui/themes/styles.css?url';
import { LinksFunction } from '@remix-run/node';
import type { MetaFunction } from '@remix-run/node';
import { Links, Meta, Scripts, ScrollRestoration, json, useLoaderData } from '@remix-run/react';
import { useState } from 'react';

import { App, AppContext, AppContextType } from './App';

import globalCss from '~/global.css?url';
import uno from '~/styles/uno.css?url';

export const links: LinksFunction = () => [
  { rel: 'stylesheet', href: globalCss },
  {
    rel: 'stylesheet',
    href: radixTheme
  },
  { rel: 'stylesheet', href: uno }
];

export const meta: MetaFunction = () => {
  return [{ title: 'Orderly Dashboard' }, { name: 'description', content: 'Orderly dashboard' }];
};

export function loader() {
  return json({
    queryServiceUrl: process.env.QUERY_SERVICE_URL!,
    evmApiUrl: process.env.EVM_API_URL!
  });
}

export default function Root() {
  const { queryServiceUrl, evmApiUrl } = useLoaderData<typeof loader>();
  const [appState] = useState<AppContextType>({ queryServiceUrl, evmApiUrl });
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body>
        <Theme
          appearance="dark"
          accentColor="iris"
          radius="small"
          className="flex flex-col flex-items-center"
        >
          <AppContext.Provider value={appState}>
            <App />
          </AppContext.Provider>
        </Theme>
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}
