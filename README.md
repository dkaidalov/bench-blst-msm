# A quick benchmark of the blst multi-scalar multiplication
This repository contains the code for benchmarking a multi-scalar multiplication implemented in the blst library.
To run bench yourself, try `nix run .#defaultPackage.x86_64-linux` (change to your system if needed).
The results show time of optimized MSM from blst for different sizes and compares it with the naive implementation of the MSM using just basic group operations from blst.
These benchmarks are performed on Dell XPS 15 9520 (CPU: 12th Gen Intel® Core™ i7-12700H×20, RAM: 32GB).
Each experiment repeated 1000 times and the average time is calculated. The standard deviation of time is also provided. 

| Group | MSM Size | MSM Optimized Average | MSM Optimized Std Dev | MSM Naive Average | MSM Naive Std Dev | Ratio Naive/Optimized |
|-------|:--------:|:----------------------|:----------------------|:------------------|:------------------|:----------------------|
| G1    |    10    | 458.868µs             | 188.026µs             | 786.975µs         | 311.395µs         | 1.72                  |
| G1    |    15    | 644.282µs             | 265.247µs             | 1.165067ms        | 447.837µs         | 1.81                  |
| G1    |    20    | 788.458µs             | 277.743µs             | 1.466808ms        | 471.744µs         | 1.86                  |
| G1    |    25    | 950.157µs             | 362.686µs             | 1.81392ms         | 563.815µs         | 1.91                  |
| G1    |    30    | 1.120236ms            | 434.512µs             | 2.189296ms        | 704.02µs          | 1.95                  |
| G1    |    31    | 1.123129ms            | 385.718µs             | 2.210026ms        | 625.684µs         | 1.97                  |
| G1    |    32    | 384.287µs             | 103.169µs             | 2.657671ms        | 868.939µs         | 6.92                  |
| G1    |    35    | 410.971µs             | 228.224µs             | 2.812808ms        | 895.628µs         | 6.84                  |
| G1    |    40    | 426.34µs              | 110.784µs             | 3.205399ms        | 934.251µs         | 7.52                  |
| G1    |    45    | 434.77µs              | 116.371µs             | 3.378552ms        | 901.476µs         | 7.77                  |
| G1    |    50    | 468.243µs             | 135.395µs             | 3.925931ms        | 1.161414ms        | 8.38                  |
| G1    |   100    | 762.195µs             | 186.046µs             | 7.320717ms        | 1.719718ms        | 9.60                  |
| G1    |   200    | 1.04952ms             | 220.825µs             | 14.189035ms       | 2.335194ms        | 13.52                 |
| G1    |   300    | 1.14375ms             | 190.516µs             | 20.92099ms        | 2.911534ms        | 18.29                 |
| G1    |   400    | 1.331455ms            | 168.184µs             | 27.399861ms       | 3.911241ms        | 20.58                 |
| G1    |   1000   | 2.562269ms            | 391.922µs             | 68.641894ms       | 6.022551ms        | 26.79                 |
| G1    |   2000   | 4.127192ms            | 383.457µs             | 134.367257ms      | 9.099272ms        | 32.56                 |
| G1    |   3000   | 5.52611ms             | 527.489µs             | 198.691855ms      | 8.385754ms        | 35.96                 |
| G1    |   4000   | 6.812057ms            | 517.766µs             | 264.086233ms      | 9.276148ms        | 38.77                 |

| Group | MSM Size | MSM Optimized Average | MSM Optimized Std Dev | MSM Naive Average | MSM Naive Std Dev | Ratio Naive/Optimized |
|-------|:--------:|:----------------------|:----------------------|:------------------|:------------------|:----------------------|
| G2    |    10    | 999.204µs             | 399.284µs             | 1.37437ms         | 451.075µs         | 1.38                  |
| G2    |    15    | 1.515134ms            | 592.938µs             | 2.110275ms        | 656.651µs         | 1.39                  |
| G2    |    20    | 1.86185ms             | 684.265µs             | 2.899563ms        | 935.934µs         | 1.56                  |
| G2    |    25    | 2.070751ms            | 702.785µs             | 3.392467ms        | 878.851µs         | 1.64                  |
| G2    |    30    | 2.275359ms            | 695.791µs             | 3.955736ms        | 931.11µs          | 1.74                  |
| G2    |    31    | 2.294361ms            | 671.435µs             | 3.985855ms        | 810.147µs         | 1.74                  |
| G2    |    32    | 910.976µs             | 302.449µs             | 4.811201ms        | 1.444121ms        | 5.28                  |
| G2    |    35    | 911.583µs             | 235.506µs             | 4.966205ms        | 1.50386ms         | 5.45                  |
| G2    |    40    | 990.162µs             | 255.695µs             | 5.686321ms        | 1.721548ms        | 5.74                  |
| G2    |    45    | 1.082024ms            | 313.514µs             | 6.45859ms         | 1.729205ms        | 5.97                  |
| G2    |    50    | 1.13853ms             | 327.653µs             | 7.464311ms        | 2.263119ms        | 6.56                  |
| G2    |   100    | 1.357369ms            | 249.737µs             | 13.537626ms       | 3.023728ms        | 9.97                  |
| G2    |   200    | 1.876261ms            | 292.948µs             | 26.548527ms       | 3.612677ms        | 14.15                 |
| G2    |   300    | 2.483203ms            | 283.939µs             | 39.086306ms       | 3.882969ms        | 15.74                 |
| G2    |   400    | 2.999871ms            | 283.437µs             | 51.902103ms       | 4.605094ms        | 17.30                 |
| G2    |   1000   | 6.518347ms            | 412.777µs             | 131.694619ms      | 8.095745ms        | 20.20                 |
| G2    |   2000   | 10.018081ms           | 848.752µs             | 257.035749ms      | 9.46314ms         | 25.66                 |
| G2    |   3000   | 13.531338ms           | 937.157µs             | 379.183887ms      | 11.254449ms       | 28.02                 |
| G2    |   4000   | 17.535896ms           | 1.210955ms            | 506.637451ms      | 15.697435ms       | 28.89                 |


Same benchmarks obtained using Criterion bench library: 
```
msm_g1/blst_msm_g1/10   time:   [349.52 µs 350.32 µs 351.52 µs]
msm_g1/naive_msm_g1/10  time:   [635.47 µs 635.83 µs 636.35 µs]
msm_g1/blst_msm_g1/15   time:   [494.00 µs 494.35 µs 494.74 µs]
msm_g1/naive_msm_g1/15  time:   [975.13 µs 984.57 µs 994.95 µs]
msm_g1/blst_msm_g1/20   time:   [643.95 µs 644.40 µs 644.91 µs]
msm_g1/naive_msm_g1/20  time:   [1.3178 ms 1.3228 ms 1.3315 ms]
msm_g1/blst_msm_g1/25   time:   [773.95 µs 775.97 µs 778.80 µs]
msm_g1/naive_msm_g1/25  time:   [1.6146 ms 1.6221 ms 1.6329 ms]
msm_g1/blst_msm_g1/30   time:   [887.42 µs 893.22 µs 898.89 µs]
msm_g1/naive_msm_g1/30  time:   [1.9605 ms 1.9674 ms 1.9746 ms]
msm_g1/blst_msm_g1/31   time:   [909.54 µs 911.23 µs 913.22 µs]
msm_g1/naive_msm_g1/31  time:   [1.9837 ms 1.9852 ms 1.9869 ms]
msm_g1/blst_msm_g1/32   time:   [314.88 µs 324.35 µs 332.72 µs]
msm_g1/naive_msm_g1/32  time:   [2.1204 ms 2.1444 ms 2.1738 ms]
msm_g1/blst_msm_g1/35   time:   [361.68 µs 367.62 µs 373.57 µs]
msm_g1/naive_msm_g1/35  time:   [2.2576 ms 2.2617 ms 2.2670 ms]
msm_g1/blst_msm_g1/40   time:   [392.76 µs 403.22 µs 413.45 µs]
msm_g1/naive_msm_g1/40  time:   [2.5910 ms 2.5968 ms 2.6045 ms]
msm_g1/blst_msm_g1/45   time:   [421.16 µs 433.96 µs 445.20 µs]
msm_g1/naive_msm_g1/45  time:   [2.9736 ms 3.0074 ms 3.0510 ms]
msm_g1/blst_msm_g1/50   time:   [412.37 µs 421.85 µs 431.35 µs]
msm_g1/naive_msm_g1/50  time:   [3.2860 ms 3.3086 ms 3.3333 ms]
msm_g1/blst_msm_g1/100  time:   [629.92 µs 660.35 µs 688.64 µs]
msm_g1/naive_msm_g1/100 time:   [6.3732 ms 6.3944 ms 6.4234 ms]
msm_g1/blst_msm_g1/200  time:   [602.74 µs 612.27 µs 624.40 µs]
msm_g1/naive_msm_g1/200 time:   [13.362 ms 13.412 ms 13.469 ms]
msm_g1/blst_msm_g1/300  time:   [1.3384 ms 1.3944 ms 1.4402 ms]
msm_g1/naive_msm_g1/300 time:   [19.149 ms 19.241 ms 19.354 ms]
msm_g1/blst_msm_g1/400  time:   [1.7999 ms 1.8191 ms 1.8361 ms]
msm_g1/naive_msm_g1/400 time:   [25.423 ms 25.459 ms 25.507 ms]
msm_g1/blst_msm_g1/1000 time:   [3.8232 ms 3.9299 ms 4.0296 ms]
msm_g1/naive_msm_g1/1000 time:   [64.355 ms 64.505 ms 64.665 ms]
msm_g1/blst_msm_g1/2000  time:   [6.0725 ms 6.2299 ms 6.3818 ms]
msm_g1/naive_msm_g1/2000 time:   [129.14 ms 129.92 ms 130.78 ms]
msm_g1/blst_msm_g1/3000  time:   [6.9584 ms 7.1566 ms 7.3517 ms]
msm_g1/naive_msm_g1/3000 time:   [191.87 ms 192.54 ms 193.32 ms]
msm_g1/blst_msm_g1/4000  time:   [8.0331 ms 8.1328 ms 8.2355 ms]
msm_g1/naive_msm_g1/4000 time:   [254.54 ms 254.81 ms 255.25 ms]

msm_g2/blst_msm_g2/10   time:   [828.05 µs 828.34 µs 828.72 µs]
msm_g2/naive_msm_g2/10  time:   [1.2227 ms 1.2243 ms 1.2267 ms]
msm_g2/blst_msm_g2/15   time:   [1.1788 ms 1.1803 ms 1.1824 ms]
msm_g2/naive_msm_g2/15  time:   [1.8348 ms 1.8353 ms 1.8359 ms]
msm_g2/blst_msm_g2/20   time:   [1.3618 ms 1.3691 ms 1.3795 ms]
msm_g2/naive_msm_g2/20  time:   [2.4745 ms 2.5022 ms 2.5382 ms]
msm_g2/blst_msm_g2/25   time:   [1.6644 ms 1.6731 ms 1.6837 ms]
msm_g2/naive_msm_g2/25  time:   [3.0598 ms 3.0635 ms 3.0685 ms]
msm_g2/blst_msm_g2/30   time:   [1.9095 ms 1.9138 ms 1.9202 ms]
msm_g2/naive_msm_g2/30  time:   [3.6610 ms 3.6745 ms 3.6989 ms]
msm_g2/blst_msm_g2/31   time:   [1.9832 ms 1.9862 ms 1.9903 ms]
msm_g2/naive_msm_g2/31  time:   [3.8772 ms 3.9049 ms 3.9363 ms]
msm_g2/blst_msm_g2/32   time:   [834.71 µs 847.70 µs 860.36 µs]
msm_g2/naive_msm_g2/32  time:   [3.9200 ms 3.9231 ms 3.9266 ms]
msm_g2/blst_msm_g2/35   time:   [907.88 µs 920.52 µs 932.61 µs]
msm_g2/naive_msm_g2/35  time:   [4.3317 ms 4.3487 ms 4.3680 ms]
msm_g2/blst_msm_g2/40   time:   [941.66 µs 963.54 µs 983.24 µs]
msm_g2/naive_msm_g2/40  time:   [4.9002 ms 4.9042 ms 4.9092 ms]
msm_g2/blst_msm_g2/45   time:   [1.0406 ms 1.0552 ms 1.0701 ms]
msm_g2/naive_msm_g2/45  time:   [5.4863 ms 5.4891 ms 5.4922 ms]
msm_g2/blst_msm_g2/50   time:   [1.1725 ms 1.1861 ms 1.1992 ms]
msm_g2/naive_msm_g2/50  time:   [6.1779 ms 6.1990 ms 6.2238 ms]
msm_g2/blst_msm_g2/100  time:   [1.7489 ms 1.7633 ms 1.7767 ms]
msm_g2/naive_msm_g2/100 time:   [12.227 ms 12.234 ms 12.242 ms]
msm_g2/blst_msm_g2/200  time:   [3.0242 ms 3.0456 ms 3.0668 ms]
msm_g2/naive_msm_g2/200 time:   [24.471 ms 24.486 ms 24.503 ms]
msm_g2/blst_msm_g2/300  time:   [4.1847 ms 4.2365 ms 4.2836 ms]
msm_g2/naive_msm_g2/300 time:   [36.662 ms 36.682 ms 36.706 ms]
msm_g2/blst_msm_g2/400  time:   [5.0882 ms 5.1361 ms 5.1818 ms]
msm_g2/naive_msm_g2/400 time:   [48.806 ms 48.839 ms 48.885 ms]
msm_g2/blst_msm_g2/1000 time:   [8.5995 ms 8.6923 ms 8.7827 ms]
msm_g2/naive_msm_g2/1000 time:   [122.18 ms 122.31 ms 122.47 ms]
msm_g2/blst_msm_g2/2000  time:   [10.947 ms 11.083 ms 11.221 ms]
msm_g2/naive_msm_g2/2000 time:   [244.68 ms 244.78 ms 244.89 ms]
msm_g2/blst_msm_g2/3000  time:   [13.731 ms 13.809 ms 13.888 ms]
msm_g2/naive_msm_g2/3000 time:   [366.17 ms 366.37 ms 366.62 ms]
msm_g2/blst_msm_g2/4000  time:   [17.331 ms 17.414 ms 17.499 ms]
msm_g2/naive_msm_g2/4000 time:   [491.21 ms 492.36 ms 493.63 ms]
```