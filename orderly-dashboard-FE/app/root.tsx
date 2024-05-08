import { withEmotionCache } from '@emotion/react';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import { LocalizationProvider } from '@mui/x-date-pickers';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs';
import { Theme } from '@radix-ui/themes';
import radixTheme from '@radix-ui/themes/styles.css?url';
import { LinksFunction } from '@remix-run/node';
import type { MetaFunction } from '@remix-run/node';
import { Links, Meta, Scripts, ScrollRestoration, json, useLoaderData } from '@remix-run/react';
import { useContext, useEffect, useRef, useState } from 'react';

import { App, AppContext, AppContextType } from './App';
import { ClientStyleContext, ServerStyleContext } from './styles';

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
        <meta name="viewport" content="width=device-width, initial-scale=1" />
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
          appearance="dark"
          accentColor="iris"
          radius="small"
          className="flex flex-col flex-items-center"
        >
          <AppContext.Provider value={appState}>
            <LocalizationProvider dateAdapter={AdapterDayjs}>
              <ThemeProvider theme={darkTheme}>
                <App />
              </ThemeProvider>
            </LocalizationProvider>
          </AppContext.Provider>
        </Theme>
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
});
export default Root;
