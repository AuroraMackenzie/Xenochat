# Secret Management

## Rule
- Plain API keys are not allowed in Xenochat configuration.
- API key entries must use encrypted format: `enc:v1:<nonce>:<ciphertext>`.
- Runtime decrypts keys with `XENOCHAT_MASTER_KEY`.

## CLI workflow
1. `export XENOCHAT_MASTER_KEY='strong-master-passphrase'`
2. `xenochat-cli seal-key 'my-api-key'`
3. Put the resulting `enc:v1:...` value into `api.api_keys`.
4. Validate with `xenochat-cli check-config configs/xenochat.toml`.

## Runtime behavior
- If config contains plaintext API keys, startup validation fails.
- If encrypted keys exist but `XENOCHAT_MASTER_KEY` is missing, API bootstrap fails.
- `/health` remains public; protected API routes require bearer auth.

## Security notes
- Master key should be injected by environment or secret manager, not written to source.
- Rotate `XENOCHAT_MASTER_KEY` and encrypted API keys together.
