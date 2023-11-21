# Source
- Create a Dummy GitHub CLI in Rust!| Ghamza
  - https://youtu.be/pyeUkQg8z9A?si=rY1ztAy7drxEWNAH


# Result

- WindowsOS 테스트 함(15초 대박 ㅋ)
  - 무려 2.8배 빠름 대박 ~~~(쓰레드 4개로 test)


```


$ Measure-Command { cargo run | Out-Host }

   Compiling windows_x86_64_msvc v0.48.5
   Compiling proc-macro2 v1.0.69
   Compiling unicode-ident v1.0.12
   Compiling utf8parse v0.2.1
   Compiling anstyle v1.0.4
   Compiling colorchoice v1.0.0
   Compiling strsim v0.10.0
   Compiling anstyle-parse v0.2.2
   Compiling clap_lex v0.6.0
   Compiling heck v0.4.1
   Compiling windows-targets v0.48.5
   Compiling windows-sys v0.48.0
   Compiling quote v1.0.33
   Compiling syn v2.0.39
   Compiling anstyle-wincon v3.0.1
   Compiling anstyle-query v1.0.0
   Compiling anstream v0.6.4
   Compiling clap_builder v4.4.8
   Compiling clap_derive v4.4.7
   Compiling clap v4.4.8
   Compiling Rust_Dummy_Github v0.1.0 (D:\young_linux\11111\Rust_Dummy_Github)
warning: crate `Rust_Dummy_Github` should have a snake case name
  |
  = help: convert the identifier to snake case: `rust_dummy_github`
  = note: `#[warn(non_snake_case)]` on by default

warning: `Rust_Dummy_Github` (bin "Rust_Dummy_Github") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 14.45s
     Running `target\debug\Rust_Dummy_Github.exe`
Usage: Rust_Dummy_Github.exe <COMMAND>

Commands:
  pr    Manage pull requests
  auth  Login or Logout
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
error: process didn't exit successfully: `target\debug\Rust_Dummy_Github.exe` (
exit code: 2)

Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 15
Milliseconds      : 219
Ticks             : 152199947
TotalDays         : 0.000176157346064815
TotalHours        : 0.00422777630555556
TotalMinutes      : 0.253666578333333
TotalSeconds      : 15.2199947
TotalMilliseconds : 15219.9947

```

- Windows 에서 그냥 Compile했을때 속도(42초)

```

$ Measure-Command { cargo run | Out-Host }


Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 42
Milliseconds      : 663
Ticks             : 426634138
TotalDays         : 0.000493789511574074
TotalHours        : 0.0118509482777778
TotalMinutes      : 0.711056896666667
TotalSeconds      : 42.6634138
TotalMilliseconds : 42663.4138
  
```

# Windows Test환경 정보

- 쓰레드 4개로 Test함
```toml
[build]
rustflags = ["-Z", "threads=4"]
```


- ```systeminfo```  powershell.exe


```
$ systeminfo

OS 이름:                 Microsoft Windows 10 Home
OS 버전:                 10.0.19045 N/A 빌드 19045
OS 제조업체:             Microsoft Corporation
OS 구성:                 독립 실행형 워크스테이션
OS 빌드 종류:            Multiprocessor Free

시스템 모델:             PMBSB09A Samsung DeskTop
시스템 종류:             x64-based PC
프로세서:                프로세서 1개 설치됨
                         [01]: Intel64 Family 6 Model 94 Stepping 3 GenuineIntel ~2900Mhz
BIOS 버전:               American Megatrends Inc. P05MBS.097.180424.XJ, 2018-04-24

시스템 로캘:             ko;한국어
입력 로캘:               ko;한국어
표준 시간대:             (UTC+09:00) 서울
총 실제 메모리:          4,014MB
사용 가능한 실제 메모리: 304MB
가상 메모리: 최대 크기:  13,742MB
가상 메모리: 사용 가능:  4,927MB
가상 메모리: 사용 중:    8,815MB
```
