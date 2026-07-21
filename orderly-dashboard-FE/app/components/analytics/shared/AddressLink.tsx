import { Link } from '@remix-run/react';
import { type FC, type ReactNode } from 'react';

import { useIsEmbed } from '~/hooks/useIsEmbed';
import { base64UrlSafeEncode, DASHBOARD_ORIGIN } from '~/util';

export type AddressLinkProps = {
  address: string;
  brokerId?: string | null;
  className?: string;
  children: ReactNode;
};

const SOL_REGEX = /^[0-9a-zA-Z]{43,44}$/;

const encodeAddress = (address: string) =>
  address.match(SOL_REGEX) ? base64UrlSafeEncode(address) : address;

const buildPath = (address: string, brokerId?: string | null) => {
  const encoded = encodeAddress(address);
  return brokerId ? `/address/${encoded}?broker_id=${brokerId}` : `/address/${encoded}`;
};

/**
 * Internal link to an address detail page.
 *
 * - Inside the dashboard app: renders a Remix <Link> (client-side routing,
 *   same-tab navigation, respects current deployment origin).
 * - Inside an embedded widget (?embed=true): renders a top-level <a> pointing
 *   at the canonical dashboard.origin so the link breaks out of the iframe.
 */
export const AddressLink: FC<AddressLinkProps> = ({ address, brokerId, className, children }) => {
  const isEmbed = useIsEmbed();
  const path = buildPath(address, brokerId);

  if (isEmbed) {
    return (
      <a
        href={`${DASHBOARD_ORIGIN}${path}`}
        target="_blank"
        rel="noopener noreferrer"
        className={className}
      >
        {children}
      </a>
    );
  }

  return (
    <Link to={path} className={className}>
      {children}
    </Link>
  );
};
