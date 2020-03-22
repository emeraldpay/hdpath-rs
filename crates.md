Common structures and utilities to operate HD Path defined by Bitcoin's BIP-32 standard.

The main specification for the Hierarchical Deterministic Wallets is [BIP-32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki),
and HD Path is a part of it which specifies the format for the hierarchy path.

The crate doesn't try to implement Key Derivation specification, but instead implements all common
functionality for creating, parsing and displaying an HD Path, especially standard paths defined
by BIP-44 and related.

The common structure, defined by BIP-43, is `m/purpose'/coin_type'/account'/change/address_index`, for example `m/44'/0'/0'/0/0`

All supported standards:

 - [BIP-32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
 - [BIP-43](https://github.com/bitcoin/bips/blob/master/bip-0043.mediawiki)
 - [BIP-44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki)
 - [BIP-49](https://github.com/bitcoin/bips/blob/master/bip-0049.mediawiki)
 - [BIP-84](https://github.com/bitcoin/bips/blob/master/bip-0084.mediawiki)

# Examples

## Basic usage
```
use hdpath::StandardHDPath;
use std::convert::TryFrom;

let hdpath = StandardHDPath::try_from("m/44'/0'/0'/0/0").unwrap();
//prints "m/44'/0'/0'/0/0"
println!("{:?}", hdpath);

//prints "0", which is account id
println!("{:?}", hdpath.account());

//prints: "purpose: Pubkey, coin: 0, account: 0, change: 0, index: 0"
println!("purpose: {:?}, coin: {}, account: {}, change: {}, index: {}",
    hdpath.purpose(),
    hdpath.coin_type(),
    hdpath.account(),
    hdpath.change(),
    hdpath.index())
```

## Create from values
```
use hdpath::{StandardHDPath, Purpose};

let hdpath = StandardHDPath::new(Purpose::Witness, 0, 1, 0, 101);
//prints "m/84'/0'/1'/0/101"
println!("{:?}", hdpath);
```

## Verify before create

Please note that values for HD Path are limited to `2^31-1` because the highest bit is reserved
for marking a _hardened_ value. Therefore, if you're getting individual values from some user
input, you should verify the value before passing to `::new`. Otherwise the constructor may
fail with _panic_ if an invalid value was passed.

```
use hdpath::{StandardHDPath, PathValue, Purpose};

fn user_path(index: u32) -> Result<StandardHDPath, ()> {
  let user_id = 1234 as u32;
  if PathValue::is_ok(index) {
    Ok(StandardHDPath::new(Purpose::Witness, 0, user_id, 0, index))
  } else {
    Err(())
  }
}
 ```