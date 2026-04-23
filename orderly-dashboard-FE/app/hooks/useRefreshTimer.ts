import { useState, useEffect, useCallback } from 'react';

export function useRefreshTimer(onRefresh?: () => void) {
  const [lastRefresh, setLastRefresh] = useState<Date>(() => new Date());
  const [secondsAgo, setSecondsAgo] = useState(0);

  useEffect(() => {
    const id = setInterval(() => {
      setSecondsAgo(Math.floor((Date.now() - lastRefresh.getTime()) / 1000));
    }, 1000);
    return () => clearInterval(id);
  }, [lastRefresh]);

  const refresh = useCallback(() => {
    setLastRefresh(new Date());
    setSecondsAgo(0);
    onRefresh?.();
  }, [onRefresh]);

  return { secondsAgo, refresh };
}
