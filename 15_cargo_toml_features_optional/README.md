# `Cargo.toml` 설정하기 관련

- https://doc.rust-lang.org/cargo/reference/features.html

# Optional dependencies

- Dependencies can be marked “optional”, which means they will not be compiled by default. For example, let’s say that our 2D image processing library uses an external package to handle GIF images. This can be expressed like this:
  - 종속성은 "선택 사항"으로 표시될 수 있으며, 이는 기본적으로 컴파일되지 않는다는 것을 의미합니다. 예를 들어, 우리의 2D 이미지 처리 라이브러리가 GIF 이미지를 처리하기 위해 외부 패키지를 사용한다고 가정해 보겠습니다. 이는 다음과 같이 표현될 수 있습니다:

```toml
[dependencies]
gif = { version = "0.11.1", optional = true }

# By default, this optional dependency implicitly defines a feature that looks like this:

[features]
gif = ["dep:gif"]
```

- This means that this dependency will only be included if the gif feature is enabled. The same `cfg(feature = "gif")` syntax can be used in the code, and the dependency can be enabled just like any feature such as `--features gif` (see Command-line feature options below).
  - 이는 gif 기능이 활성화된 경우에만 이 종속성이 포함된다는 것을 의미합니다. 코드에서 동일한 `cfg(feature = "gif")` 구문을 사용할 수 있으며, 종속성은 `--features gif` 와 같은 기능과 마찬가지로 활성화될 수 있습니다(아래 명령줄 기능 옵션 참조).

- In some cases, you may not want to expose a feature that has the same name as the optional dependency. For example, perhaps the optional dependency is an internal detail, or you want to group multiple optional dependencies together, or you just want to use a better name. If you specify the optional dependency with the dep: prefix anywhere in the [features] table, that disables the implicit feature.
  - 경우에 따라 선택적 종속성과 동일한 이름을 가진 기능을 노출하지 않을 수 있습니다. 예를 들어, 선택적 종속성이 내부 세부 정보이거나 여러 선택적 종속성을 함께 그룹화하거나 더 나은 이름을 사용하기를 원할 수 있습니다. [기능] 테이블의 어느 위치에서나 dep: 접두사를 사용하여 선택적 종속성을 지정하면 암시적 기능이 비활성화됩니다.

- Note: The dep: syntax is only available starting with Rust 1.60. Previous versions can only use the implicit feature name.
  - 참고: dep: 구문은 Rust 1.60부터는 사용할 수 있습니다. 이전 버전에서는 암시적 기능 이름만 사용할 수 있습니다

- For example, let’s say in order to support the AVIF image format, our library needs two other dependencies to be enabled:
  - 예를 들어, AVIF 이미지 형식을 지원하려면 라이브러리를 활성화해야 하는 두 가지 다른 종속성이 필요하다고 가정해 보겠습니다:

```toml
[dependencies]
ravif = { version = "0.6.3", optional = true }
rgb = { version = "0.8.25", optional = true }

[features]
avif = ["dep:ravif", "dep:rgb"]
```

- In this example, the avif feature will enable the two listed dependencies. This also avoids creating the implicit ravif and rgb features, since we don’t want users to enable those individually as they are internal details to our crate.
  - 이 예에서 avif 기능은 나열된 두 가지 종속성을 활성화합니다. 이는 또한 암묵적인 ravif 및 rgb 기능이 상자에 대한 내부 세부 정보이기 때문에 사용자가 개별적으로 활성화하는 것을 원하지 않기 때문에 생성되는 것을 방지합니다
