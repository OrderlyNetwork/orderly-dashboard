-- Your SQL goes here
CREATE TABLE IF NOT EXISTS symbols
(
    symbol      varchar(128) primary key not null,
    symbol_hash varchar(256)             not null
);

insert into symbols(symbol, symbol_hash) values
                                             ('PERP_BTC_USDC','0x5a8133e52befca724670dbf2cade550c522c2410dd5b1189df675e99388f509d'),
                                             ('PERP_ETH_USDC','0x7e83089239db756ee233fa8972addfea16ae653db0f692e4851aed546b21caeb'),
                                             ('PERP_NEAR_USDC','0xa2adc016e890b4fbbf161c7eaeb615b893e4fbeceae918fa7bf16cc40d46610b'),
                                             ('PERP_WOO_USDC','0x5d0471b083610a6f3b572fc8b0f759c5628e74159816681fb7d927b9263de60b'),
                                             ('PERP_SOL_USDC','0x2f1991e99a4e22a9e95ff1b67aee336b4047dc47612e36674fa23eb8c6017f2e'),
                                             ('PERP_OP_USDC','0xd44817bf72a4d9b5e277bfec92619466999b1adbd9f3c52621d1651ac354b09c'),
                                             ('PERP_BNB_USDC','0x76bba29822652c557a30fe45ff09e7e244e3819699df0c0995622c12db16e72d'),
                                             ('PERP_XRP_USDC','0x2aa4f612cf7a91de02395cadb419d3bf578130509b35b69c05738860e5b74637'),
                                             ('PERP_MATIC_USDC','0xa84558a42cda72af9bb348e8fc6cdfca9b3ddd885f1b8877abbc33beafc8bfec'),
                                             ('PERP_LINK_USDC','0xcaf4dffbbf83b8f5c74bb2946baeb3da1c6c7fc6290a899b18e95bb6f11c0503'),
                                             ('PERP_ARB_USDC','0x3e5bb1a69a9094f1b2ccad4f39a7d70e2a29f08c2c0eac87b970ea650ac12ec2'),
                                             ('PERP_BCH_USDC','0xc3d5ec779f548bc3d82ab3438416db751e7e1946827b31eeb1bd08e367278281'),
                                             ('PERP_APT_USDC','0xe31e58f63b7cc1ad056bda9f1be47bf0ad0891a03d3a759f68c7814241a48907'),
                                             ('PERP_SUI_USDC','0x01bec50d553af75d1a2204c760570f374c438885070eb995500c7a08fc5a9ec2');
