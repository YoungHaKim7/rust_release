# Result

```
My Enum _ RefVectorRefVector([32])

```


```rs


enum MyEnum<'a> {               // Discriminant [수학] 판별식.
    Nothing,                    // 0
    RefVector(&'a Vec<i32>),    // 1
}


```
