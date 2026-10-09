# Result(WinOS11 test)261009

```pwsh
> $env:CGO_ENABLED = "1"

>> $env:CC = "clang"
>>
>> go clean -cache
>> go build -o app.exe .
# abi_go
  lld-link: warning: section name . debug_abbrev is longer than 8 characters and will use a non-standard string table
  lld-link: warning: section name .debug_addr is longer than 8 characters and will use a non-standard string table
  lld-link: warning: section name .debug_frame is longer than 8 characters and will use a non-standard string table
  lld-link: warning: section name .debug_gdb_scripts is longer than 8 characters and will use a non-standard string table
  lld-link: warning: section name .debug_info is longer than 8 characters and will use a non-standard string table
  lld-link: warning: section name .debug_line is longer than 8 characters and will use a non-standard string table
  lld-link: warning: section name .debug_loclists is longer than 8 characters and will use a non-standard string table
  lld-link: warning: section name .debug_rnglists is longer than 8 characters and will use a non-standard string table

PS C:\abi_rust\examples\go> .\app.exe
Rust ABI called from Go
10 + 20 = 30
20 - 10 = 10
10 * 20 = 200
```
