`#[cfg(feature = "config")]`

# ccanvas config

An easy way to manage config files.

## Quickstart

```rs
#[derive(Serialize, Deserialize, CcanvasConfig)]
struct MyConfig {
    value1: String,
    value2: u32,
    // ... 
}
```

In your code, you can use `MyConfig::load()` to load the config file.


## Behaviour

Assuming you have only **one config struct per component**. The config file path is determined as follows.

|Cargo project name|Config file path|
|---|---|
|ccanvas-NAME|~/.config/ccanvas/NAME.jsonc|
|ccanvas-NAME-SUBNAME|~/.config/ccanvas/NAME/SUBNAME.jsonc|

Where NAME/SUBNAME represents a hierarchy - the SUBNAME component is under the NAME bundle.

**If you have more than one config struct in your component**, or wish to load a config struct under a different name, you could *lie to the compiler* by implementing `CcanvasConfig` under a different crate name, for example.

```rs
impl CcanvasConfig for MyConfig {
    const CNAME: &'static str: "ccanvas-fakecratename";
}
```
