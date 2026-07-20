import { useSearchParams } from '@remix-run/react';

export const useIsEmbed = () => {
  const [searchParams] = useSearchParams();
  return searchParams.get('embed') === 'true';
};
