import { withEmotionCache } from '@emotion/react';
import { MantineProvider } from '@mantine/core';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import { Theme } from '@radix-ui/themes';
import radixTheme from '@radix-ui/themes/styles.css?url';
import { LinksFunction } from '@remix-run/node';
import type { MetaFunction } from '@remix-run/node';
import { Links, Meta, Scripts, ScrollRestoration, json, useLoaderData } from '@remix-run/react';
import dayjs from 'dayjs';
import localizedFormat from 'dayjs/plugin/localizedFormat';
import { useContext, useEffect, useRef, useState } from 'react';

import { App, AppContext, AppContextType } from './App';
import { ClientStyleContext, ServerStyleContext } from './styles';

import favicon from '~/assets/favicon.svg?url';
import globalCss from '~/global.css?url';
import uno from '~/styles/uno.css?url';

import '@mantine/core/styles.css';
import '@mantine/dates/styles.css';

dayjs.extend(localizedFormat);

export const links: LinksFunction = () => [
  { rel: 'stylesheet', href: globalCss },
  {
    rel: 'stylesheet',
    href: radixTheme
  },
  { rel: 'stylesheet', href: uno },
  { rel: 'icon', href: favicon, type: 'image/svg+xml' }
];

export const meta: MetaFunction = () => {
  return [
    { title: 'Orderly Dashboard' },
    { name: 'description', content: 'Orderly dashboard' },
    {
      name: 'viewport',
      content: 'width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no'
    }
  ];
};

export function loader() {
  return json({
    queryServiceUrl: process.env.QUERY_SERVICE_URL!,
    evmApiUrl: process.env.EVM_API_URL!
  });
}

const darkTheme = createTheme({
  palette: {
    mode: 'dark'
  }
});

const Root = withEmotionCache((_, emotionCache) => {
  const { queryServiceUrl, evmApiUrl } = useLoaderData<typeof loader>();
  const [appState] = useState<AppContextType>({ queryServiceUrl, evmApiUrl });

  const serverStyleData = useContext(ServerStyleContext);
  const clientStyleData = useContext(ClientStyleContext);
  const reinjectStylesRef = useRef(true);

  // Only executed on client
  // When a top level ErrorBoundary or CatchBoundary are rendered,
  // the document head gets removed, so we have to create the style tags
  useEffect(() => {
    if (!reinjectStylesRef.current) {
      return;
    }
    // re-link sheet container
    emotionCache.sheet.container = document.head;

    // re-inject tags
    const tags = emotionCache.sheet.tags;
    emotionCache.sheet.flush();
    tags.forEach((tag) => {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      (emotionCache.sheet as any)._insertTag(tag);
    });

    // reset cache to re-apply global styles
    clientStyleData.reset();
    // ensure we only do this once per mount
    reinjectStylesRef.current = false;
  }, [clientStyleData, emotionCache.sheet]);

  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta
          name="viewport"
          content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no"
        />
        <Meta />
        <Links />
        {serverStyleData?.map(({ key, ids, css }) => (
          <style
            key={key}
            data-emotion={`${key} ${ids.join(' ')}`}
            // eslint-disable-next-line react/no-danger
            dangerouslySetInnerHTML={{ __html: css }}
          />
        ))}
      </head>
      <body>
        <Theme
          hasBackground={false}
          appearance="dark"
          accentColor="iris"
          radius="medium"
          className="flex flex-col flex-items-center min-h-screen"
        >
          <MantineProvider
            forceColorScheme="dark"
            defaultColorScheme="dark"
            classNamesPrefix="app"
            withGlobalClasses={false}
          >
            <AppContext.Provider value={appState}>
              <ThemeProvider theme={darkTheme}>
                <App />
              </ThemeProvider>
            </AppContext.Provider>
          </MantineProvider>
        </Theme>
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
});
export default Root;
