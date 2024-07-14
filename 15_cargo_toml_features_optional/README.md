# `Cargo.toml` 설정하기 관련

- https://doc.rust-lang.org/cargo/reference/features.html

# Optional dependencies

- Dependencies can be marked “optional”, which means they will not be compiled by default. For example, let’s say that our 2D image processing library uses an external package to handle GIF images. This can be expressed like this:

```toml
[dependencies]
gif = { version = "0.11.1", optional = true }

# By default, this optional dependency implicitly defines a feature that looks like this:

[features]
gif = ["dep:gif"]
```

- This means that this dependency will only be included if the gif feature is enabled. The same cfg(feature = "gif") syntax can be used in the code, and the dependency can be enabled just like any feature such as --features gif (see Command-line feature options below).

In some cases, you may not want to expose a feature that has the same name as the optional dependency. For example, perhaps the optional dependency is an internal detail, or you want to group multiple optional dependencies together, or you just want to use a better name. If you specify the optional dependency with the dep: prefix anywhere in the [features] table, that disables the implicit feature.

- Note: The dep: syntax is only available starting with Rust 1.60. Previous versions can only use the implicit feature name.

For example, let’s say in order to support the AVIF image format, our library needs two other dependencies to be enabled:

```toml
[dependencies]
ravif = { version = "0.6.3", optional = true }
rgb = { version = "0.8.25", optional = true }

[features]
avif = ["dep:ravif", "dep:rgb"]
```

In this example, the avif feature will enable the two listed dependencies. This also avoids creating the implicit ravif and rgb features, since we don’t want users to enable those individually as they are internal details to our crate.
