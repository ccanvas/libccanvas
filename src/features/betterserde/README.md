`#[cfg(feature = "betterserde")]`

# Better serde

Structs that serialises to a prettier format compared to the default structs.

E.g. default serialisation for `KeyEvent`.

```json
{
  "code": {
    "char": 'a'
  },
  "modifier": "ctrl"
}
```

The better serde serialisation for `KeyEvent`, with `KeyEventBetterSerde`.

```json
{
  "type": "char",
  "value": 'a',
  "modifier": "ctrl"
}
```

`KeyEventBetterSerde` implements `From<KeyEvent>` and `Into<KeyEvent>`.
