Best is now 3bit2_foreach loop. Slower than some other hashers, but less collision, and added benefit of faster rc computation, and possibly SIMD

```
Running target/release/deps/hashing-c1433054d17cf6e2
Hashing Vec<u8>/3bit2fe time:   [6.0534 ms 6.0608 ms 6.0697 ms]
                        change: [-2.4957% -1.8814% -1.2931%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) high mild
  7 (7.00%) high severe
Hashing Vec<u8>/3bit2   time:   [6.3479 ms 6.3583 ms 6.3703 ms]
                        change: [-2.0568% -1.4827% -0.9707%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
Hashing Vec<u8>/3bit2_1 time:   [6.5638 ms 6.5717 ms 6.5799 ms]
                        change: [-0.8986% -0.5747% -0.2778%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
Hashing Vec<u8>/3bit2_2 time:   [6.6446 ms 6.6688 ms 6.6977 ms]
                        change: [-2.1950% -1.6487% -1.0892%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe
Hashing Vec<u8>/t1ha0   time:   [5.4948 ms 5.5035 ms 5.5131 ms]
                        change: [-3.0920% -2.3625% -1.6899%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
Hashing Vec<u8>/fnv     time:   [6.0560 ms 6.0632 ms 6.0711 ms]
                        change: [-3.4718% -2.9239% -2.4169%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) high mild
  4 (4.00%) high severe
Hashing Vec<u8>/xxhash  time:   [7.1078 ms 7.1207 ms 7.1350 ms]
                        change: [+10.242% +10.675% +11.105%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  8 (8.00%) high mild
  1 (1.00%) high severe
Hashing Vec<u8>/seahash time:   [6.6311 ms 6.6444 ms 6.6618 ms]
                        change: [-1.6486% -1.2753% -0.9152%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  5 (5.00%) high mild
  7 (7.00%) high severe
Hashing Vec<u8>/wyhash  time:   [6.0328 ms 6.0393 ms 6.0462 ms]
                        change: [-14.637% -10.697% -6.9262%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
```
