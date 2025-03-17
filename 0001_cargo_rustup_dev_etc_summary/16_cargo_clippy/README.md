# rust(clippy 문서)
- https://doc.rust-lang.org/clippy/

# Command line

- You can configure lint levels on the command line by adding -A/W/D clippy::lint_name like this:
  - A/W/D 클리피:lint_name을 추가하여 명령 줄에 보푸라기 레벨을 구성할 수 있습니다:

```bash
cargo clippy -- -Aclippy::style -Wclippy::double_neg -Dclippy::perf
```

- For CI all warnings can be elevated to errors which will in turn fail the build and cause Clippy to exit with a code other than 0.
  - CI의 경우 모든 경고를 오류로 높일 수 있으며, 이로 인해 빌드가 실패하고 클리피가 0이 아닌 코드로 종료됩니다.

```bash
cargo clippy -- -Dwarnings
```

- https://doc.rust-lang.org/clippy/usage.html
