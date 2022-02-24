# Compare several different ways for string replacement
## Intro

As in our project we need same struct different fields name and contents in json and bson, we need to convert between DB and Api struct. 

So first way is using `impl From` and loop to handle 10000 records, but think it is more slower than other ways(but if we have more fields need to be change that can be a choice).

`api2db_struct`, `db2api_struct` are using `impl From` to convert structs. (multi fields name and contents changed)

`replace_oid2str`, `replace_str2oid` are using `unsafe u8` to convert bson::ObjectId and string format in json string

`regex_replace_all`, `str replace`, `replace_string_add`, `replace_u8_unsafe_morecap` are compare with single str replace benchmarks.


## Result

### 10000 Records
```bash
running 8 tests  
test tests::bench_api2db_struct             ... bench:  10,777,840 ns/iter (+/- 1,541,968)  
test tests::bench_db2api_struct             ... bench:   7,903,435 ns/iter (+/- 2,178,199)  
test tests::bench_regex_replace_all         ... bench:   3,315,995 ns/iter (+/- 662,795)  
test tests::bench_replace                   ... bench:   4,143,910 ns/iter (+/- 947,399)  
test tests::bench_replace_oid2str           ... bench:   2,073,625 ns/iter (+/- 968,135)  
test tests::bench_replace_str2oid           ... bench:   1,804,675 ns/iter (+/- 580,960)  
test tests::bench_replace_string_add        ... bench:   1,700,280 ns/iter (+/- 617,699)  
test tests::bench_replace_u8_unsafe_morecap ... bench:   1,705,825 ns/iter (+/- 579,968)  
```

### 1000 Records
```bash
running 8 tests
test tests::bench_api2db_struct             ... bench:     706,121 ns/iter (+/- 224,431)
test tests::bench_db2api_struct             ... bench:     534,377 ns/iter (+/- 77,413)
test tests::bench_regex_replace_all         ... bench:     539,595 ns/iter (+/- 48,844)
test tests::bench_replace                   ... bench:     124,147 ns/iter (+/- 37,775)
test tests::bench_replace_oid2str           ... bench:     124,498 ns/iter (+/- 12,935)
test tests::bench_replace_str2oid           ... bench:      84,601 ns/iter (+/- 8,789)
test tests::bench_replace_string_add        ... bench:      83,253 ns/iter (+/- 15,767)
test tests::bench_replace_u8_unsafe_morecap ... bench:      80,980 ns/iter (+/- 26,040)
```

### 100 Records
```bash
running 8 tests
test tests::bench_api2db_struct             ... bench:      72,963 ns/iter (+/- 16,666)
test tests::bench_db2api_struct             ... bench:      55,798 ns/iter (+/- 8,361)
test tests::bench_regex_replace_all         ... bench:      28,692 ns/iter (+/- 2,772)
test tests::bench_replace                   ... bench:      12,974 ns/iter (+/- 2,994)
test tests::bench_replace_oid2str           ... bench:      45,609 ns/iter (+/- 35,226)
test tests::bench_replace_str2oid           ... bench:      30,017 ns/iter (+/- 10,950)
test tests::bench_replace_string_add        ... bench:      27,815 ns/iter (+/- 10,362)
test tests::bench_replace_u8_unsafe_morecap ... bench:      26,710 ns/iter (+/- 5,008)
```bash
