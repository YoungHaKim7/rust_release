#  Rust Reflection MVP pull request merged into rust-lang/main, marking progress on compile-time reflection
- https://github.com/rust-lang/rust/pull/146923

- project goal issue: rust-lang/rust-project-goals#406
tracking issue: #146922

- The design currently implemented by this PR is
  - `TypeId::info` (method, usually used as `id.info()` returns a Type struct
  - the `Type` struct has fields that contain information about the type
  - the most notable field is `kind`, which is a non-exhaustive enum over all possible type kinds and their specific information. So it has a `Tuple(Tuple)` variant, where the only field is a `Tuple` struct type that contains more information (The list of type ids that make up the tuple).
  - To get nested type information (like the type of fields) you need to call `TypeId::info` again.
  - There is only one language intrinsic to go from `TypeId` to `Type`, and it does all the work

# C++26 에서 만든거 힌트
- This is how C++ does it (see https://lemire.me/blog/2025/06/22/c26-will-include-compile-time-reflection-why-should-you-care/ and https://isocpp.org/files/papers/P2996R13.html#member-queries)

- 4.4.15 members_of, static_data_members_of, nonstatic_data_members_of, bases_of, enumerators_of
```cpp
namespace std::meta {
  consteval auto members_of(info r) -> vector<info>;
  consteval auto bases_of(info type_class) -> vector<info>;

  consteval auto static_data_members_of(info type_class) -> vector<info>;
  consteval auto nonstatic_data_members_of(info type_class) -> vector<info>;

  consteval auto enumerators_of(info type_enum) -> vector<info>;
}
```
