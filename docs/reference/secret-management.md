# Secret Management

## Rule
- Plain API keys are not allowed in Xenochat configuration.
- API key entries must use encrypted format: `enc:v1:<nonce>:<ciphertext>`.
- Runtime decrypts keys with the resolved master key:
  - first `XENOCHAT_MASTER_KEY`
  - then (macOS only) Keychain lookup.

## CLI workflow
1. `export XENOCHAT_MASTER_KEY='strong-master-passphrase'`
2. `xenochat-cli seal-key 'my-api-key'`
3. Put the resulting `enc:v1:...` value into `api.api_keys`.
4. Validate with `xenochat-cli check-config configs/xenochat.toml`.
5. Verify master key availability with `xenochat-cli master-key-check`.

## macOS Keychain workflow (optional)
1. `security add-generic-password -U -a "$USER" -s "xenochat.master-key" -w "strong-master-passphrase"`
2. Omit `XENOCHAT_MASTER_KEY` and run `xenochat-cli master-key-check`.
3. Optional overrides:
   - `XENOCHAT_KEYCHAIN_SERVICE` (default: `xenochat.master-key`)
   - `XENOCHAT_KEYCHAIN_ACCOUNT` (default: `$USER` or `xenochat`)

## Runtime behavior
- If config contains plaintext API keys, startup validation fails.
- If encrypted keys exist but no master key can be resolved from env/Keychain, API bootstrap fails.
- `/health` remains public; protected API routes require bearer auth.

## Security notes
- Master key should be injected by environment or secret manager, not written to source.
- Rotate `XENOCHAT_MASTER_KEY` and encrypted API keys together.
