import { Theme } from '@radix-ui/themes';
import radixTheme from '@radix-ui/themes/styles.css?url';
import { LinksFunction } from '@remix-run/node';
import { Links, Meta, Outlet, Scripts, ScrollRestoration } from '@remix-run/react';

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

export function Layout({ children }: { children: React.ReactNode }) {
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
          {children}
        </Theme>
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

export default function App() {
  return <Outlet />;
}
