## Address Lookup Table Updater

Implements a Solana programm with the following behaviour:

1. If no address lookup table (ALT) is found at the specified account, create one.
2. Add the specified account addresses to the ALT table, either an existing one or the one created in step 1.


## Solana program requirements

Solana program implements single instruction with the following expectations:

### account references involved in the transaction, indexed from zero
- 0, address lookup table account
- 1, authority account
- 2, payer account
- 3, system program account
- 4 and onwards, account references that should be added to the ALT table

### instruction data
no instruction data used
