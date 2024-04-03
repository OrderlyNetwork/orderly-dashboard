CREATE TABLE IF NOT EXISTS broker_info
(
    broker_id       text  not null,
    broker_hash     text  not null,
    constraint pr_broker_id primary key (broker_id)
);

insert into broker_info(broker_id, broker_hash)
values('woofi_dex','0x083098c593f395bea1de45dda552d9f14e8fcb0be3faaa7a1903c5477d7ba7fd'),
      ('woofi_pro','0x6ca2f644ef7bd6d75953318c7f2580014941e753b3c6d54da56b3bf75dd14dfc'),
      ('root','0xd6c66cad06fe14fdb6ce9297d80d32f24d7428996d0045cbf90cc345c677ba16'),
      ('orderly','0x95d85ced8adb371760e4b6437896a075632fbd6cefe699f8125a8bc1d9b19e5b'),
      ('busywhale','0x04dce91619b1a6f642194df4a31de20fe5d0fc5920f1780a19d0094397dfd995'),
      ('0xfin','0x2a2d73ab742ed2fe81315477dcf1c4c6cb0426d85c89517f19a251ccaf29a0ed'),
      ('emdx_dex','0x025fccca3e9cd02d2f69c53f9ec2180704a242c91da89d84333eab3eea9a640e'),
      ('logx','0x2793c4a20928c2351d9628a027b024fb7752cdab5604d8aba4cb5fe3f2d4fe98'),
      ('prime_protocol','0xddfd58008dd33c0e4690cce427ead641783602f40b5a7e303c354c2eb29ff411'),
      ('bitoro_network','0x056711bcb7dbcd9847d2288a447beb392f6fcc6d283400e262c5db37678c8d81'),
      ('coolwallet','0x97e23ddcbfe2af8e707db55e977131bda6d1b41af8ae95faf730c7f8b6c4ef09'),
      ('quick_perps','0xe35176f8bf408451c6aa9a7ee3bb877d56dfcbb69d6b4b91ea4079670018c75d'),
      ('empyreal','0x530b138f98a5c29d5c29f85eb633bbeb9c950987a8bae8d01cd579530d390de6'),
      ('galar_fin','0x031245a7b6223a16ae99decad1fb3632f08adb8324354ada4c5ca3382b10d404'),
      ('what_exchange','0xcd5853ffaba572f2d589c5c8be2668efee814657ab495c263b71fb51fbd02673'),
      ('ibx','0x7a5f6b12f1adef5d3c490f6822df71a29e4dad07fccd7d7e61f36c8567dd3d22'),
      ('unibot','0x66718e7e7d40635b4d46a04ff9adc1e11e5396cbcc185679dde19515689c9e0d');