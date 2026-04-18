# Result


```bash
res 15 : Ok(21)
res 15 : Ok(13)
Memory address: 0x7ffd1234 (decimal: 2147291700)

```

## Simple Explanation

When you write `from_str_radix("15", 16)`, you're telling Rust to read `"15"` as a **base-16 number**.
- 'from_str_radix("15", 16)'를 쓸 때, Rust에게 '15'를 **base-16 숫자**로 읽으라고 말하는 것입니다.

### Step by Step:

```
String:  "1"    "5"
         ↓      ↓
Base 16: 16¹   16⁰
         ↓      ↓
Value:   16    1
         ×     ×
Digit:   1     5
         ↓     ↓
        16  +  5  =  21
```

### Think of it this way:

**In base 10** (everyday numbers):
- The number **"15"** means: 1 ten + 5 ones = **15**

**In base 16** (hexadecimal):
- The number **"15"** means: 1 sixteen + 5 ones = **21**

### Why 16?

Because the **base** tells you what each position is worth:
- Base 10: positions are worth 1, 10, 100, 1000...
- Base 16: positions are worth 1, 16, 256, 4096...
- Base 2: positions are worth 1, 2, 4, 8, 16...

Your string `"15"` has two digits:
- Left digit (`1`) × 16 = 16
- Right digit (`5`) × 1 = 5
- **Total = 21**

That's why you get 21!

### 이렇게 생각하세요:

**밑변 10** (일상 숫자)에서:
- 숫자 **"15"**는 다음을 의미합니다: 10 + 1 = **15**

**16** (16진수)에서:
- 숫자 **"15"**는 다음을 의미합니다: 1 16 + 5 1 = **21**

### 왜 16살인가요?

**베이스**가 각 포지션의 가치를 알려주기 때문입니다:
- 기본 10: 포지션은 1, 10, 100, 1000의 가치가 있습니다...
- 베이스 16: 포지션은 1, 16, 256, 4096의 가치가 있습니다...
- 기본 2: 포지션은 1, 2, 4, 8, 16의 가치가 있습니다...

문자열 '15'에는 두 자리 숫자가 있습니다:
- 왼쪽 숫자 ('''1''') × 16 = 16
- 오른쪽 숫자 ('''5''') × 1 = 5
- **총 = 21**

그래서 21개를 얻게 되는 거예요!



# add
- Added memory address parsing from log:

```rust
let log_line = "Program loaded at address 0x7ffd1234";
let hex_str = log_line.split("0x").last().unwrap();
let addr = usize::from_str_radix(hex_str, 16).unwrap();
```

Output: `0x7ffd1234 (decimal: 2147291700)`
