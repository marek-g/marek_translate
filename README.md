# Marek Translate

Rust project to allow easy usage of natural language translation libraries with a common API.

## Supported backends

- `marek_translate_locally` - bindings for [translateLocally](https://github.com/XapaJIaMnu/translateLocally) - wrapper over [bergamot-translator](https://github.com/browsermt/bergamot-translator) which is based on [marian](https://github.com/browsermt/marian-dev) - an offline translator that is used in Firefox

## Examples

- `marek_translate_test` - uses `marek_translate_locally` to translate sample text. To run it, you need to have `translateLocally` executable in the current folder 
