# A quick benchmark on MSM vs the naive approach
To run bench yourself, try `nix run .#defaultPackage.x86_64-linux` (change to your system if needed).
```bash
G1Projective - Size: 1 - MSM Average Time: 76.501µs - MSM Std Dev: 48.893µs - Naive Average Time: 68.721µs - Naive Std Dev: 5.105µs
G1Projective - Size: 2 - MSM Average Time: 203.589µs - MSM Std Dev: 71.813µs - Naive Average Time: 213.692µs - Naive Std Dev: 77.599µs
G1Projective - Size: 3 - MSM Average Time: 170.338µs - MSM Std Dev: 24.764µs - Naive Average Time: 213.559µs - Naive Std Dev: 29.102µs
G1Projective - Size: 4 - MSM Average Time: 209.752µs - MSM Std Dev: 50.904µs - Naive Average Time: 287.762µs - Naive Std Dev: 65.233µs
G1Projective - Size: 5 - MSM Average Time: 279.057µs - MSM Std Dev: 100.749µs - Naive Average Time: 415.006µs - Naive Std Dev: 151.252µs
G1Projective - Size: 6 - MSM Average Time: 343.181µs - MSM Std Dev: 124.088µs - Naive Average Time: 523.404µs - Naive Std Dev: 189.193µs
G1Projective - Size: 7 - MSM Average Time: 367.07µs - MSM Std Dev: 139.428µs - Naive Average Time: 582.385µs - Naive Std Dev: 230.407µs
G1Projective - Size: 8 - MSM Average Time: 369.737µs - MSM Std Dev: 114.676µs - Naive Average Time: 605.777µs - Naive Std Dev: 170.706µs
G1Projective - Size: 9 - MSM Average Time: 430.267µs - MSM Std Dev: 159.061µs - Naive Average Time: 714.973µs - Naive Std Dev: 262.218µs
G1Projective - Size: 10 - MSM Average Time: 470.595µs - MSM Std Dev: 158.85µs - Naive Average Time: 786.199µs - Naive Std Dev: 254.281µs
G1Projective - Size: 11 - MSM Average Time: 533.167µs - MSM Std Dev: 205.718µs - Naive Average Time: 879.059µs - Naive Std Dev: 329.424µs
G1Projective - Size: 12 - MSM Average Time: 520.543µs - MSM Std Dev: 184.232µs - Naive Average Time: 853.424µs - Naive Std Dev: 251.546µs
G1Projective - Size: 13 - MSM Average Time: 576.109µs - MSM Std Dev: 212.759µs - Naive Average Time: 985.955µs - Naive Std Dev: 342.892µs
G1Projective - Size: 14 - MSM Average Time: 647.951µs - MSM Std Dev: 246.193µs - Naive Average Time: 1.11806ms - Naive Std Dev: 422.341µs
G1Projective - Size: 15 - MSM Average Time: 730.727µs - MSM Std Dev: 280.162µs - Naive Average Time: 1.23822ms - Naive Std Dev: 486.606µs
G2Projective - Size: 1 - MSM Average Time: 118.56µs - MSM Std Dev: 5.301µs - Naive Average Time: 116.826µs - Naive Std Dev: 4.49µs
G2Projective - Size: 2 - MSM Average Time: 271.783µs - MSM Std Dev: 12.298µs - Naive Average Time: 238.591µs - Naive Std Dev: 10.182µs
G2Projective - Size: 3 - MSM Average Time: 358.1µs - MSM Std Dev: 49.804µs - Naive Average Time: 373.889µs - Naive Std Dev: 32.152µs
G2Projective - Size: 4 - MSM Average Time: 523.827µs - MSM Std Dev: 220.015µs - Naive Average Time: 600.169µs - Naive Std Dev: 252.307µs
G2Projective - Size: 5 - MSM Average Time: 620.186µs - MSM Std Dev: 273.169µs - Naive Average Time: 759.977µs - Naive Std Dev: 341.587µs
G2Projective - Size: 6 - MSM Average Time: 597.058µs - MSM Std Dev: 127.818µs - Naive Average Time: 751.046µs - Naive Std Dev: 76.362µs
G2Projective - Size: 7 - MSM Average Time: 719.443µs - MSM Std Dev: 253.549µs - Naive Average Time: 946.492µs - Naive Std Dev: 319.19µs
G2Projective - Size: 8 - MSM Average Time: 739.582µs - MSM Std Dev: 198.524µs - Naive Average Time: 1.029942ms - Naive Std Dev: 290.045µs
G2Projective - Size: 9 - MSM Average Time: 806.656µs - MSM Std Dev: 203.134µs - Naive Average Time: 1.126647ms - Naive Std Dev: 254.444µs
G2Projective - Size: 10 - MSM Average Time: 927.643µs - MSM Std Dev: 325.004µs - Naive Average Time: 1.273538ms - Naive Std Dev: 305.803µs
G2Projective - Size: 11 - MSM Average Time: 997.178µs - MSM Std Dev: 341.916µs - Naive Average Time: 1.405458ms - Naive Std Dev: 376.615µs
G2Projective - Size: 12 - MSM Average Time: 1.14713ms - MSM Std Dev: 349.707µs - Naive Average Time: 1.678636ms - Naive Std Dev: 475.224µs
G2Projective - Size: 13 - MSM Average Time: 1.341609ms - MSM Std Dev: 524.559µs - Naive Average Time: 1.856596ms - Naive Std Dev: 610.486µs
G2Projective - Size: 14 - MSM Average Time: 1.286984ms - MSM Std Dev: 452.273µs - Naive Average Time: 1.835485ms - Naive Std Dev: 551.973µs
G2Projective - Size: 15 - MSM Average Time: 1.650094ms - MSM Std Dev: 677.018µs - Naive Average Time: 2.466159ms - Naive Std Dev: 986.483µs
```

Criterion benchmarks:
```
msm_g1/blst_msm_g1/1    time:   [65.440 µs 65.515 µs 65.591 µs]
msm_g1/naive_msm_g1/1   time:   [63.654 µs 63.965 µs 64.551 µs]

msm_g1/blst_msm_g1/2    time:   [118.19 µs 118.27 µs 118.35 µs]
msm_g1/naive_msm_g1/2   time:   [127.77 µs 128.13 µs 128.52 µs]

msm_g1/blst_msm_g1/3    time:   [147.95 µs 148.15 µs 148.38 µs]
msm_g1/naive_msm_g1/3   time:   [192.60 µs 193.60 µs 194.75 µs]

msm_g1/blst_msm_g1/4    time:   [176.60 µs 176.93 µs 177.35 µs]
msm_g1/naive_msm_g1/4   time:   [254.92 µs 255.68 µs 256.65 µs]

msm_g1/blst_msm_g1/5    time:   [207.07 µs 207.27 µs 207.48 µs]
msm_g1/naive_msm_g1/5   time:   [318.14 µs 319.78 µs 322.78 µs]

msm_g1/blst_msm_g1/6    time:   [235.18 µs 235.38 µs 235.60 µs]
msm_g1/naive_msm_g1/6   time:   [381.57 µs 382.84 µs 384.92 µs]

msm_g1/blst_msm_g1/7    time:   [263.73 µs 264.04 µs 264.41 µs]
msm_g1/naive_msm_g1/7   time:   [452.07 µs 457.37 µs 464.28 µs]

msm_g1/blst_msm_g1/8    time:   [298.10 µs 300.09 µs 302.86 µs]
msm_g1/naive_msm_g1/8   time:   [512.66 µs 515.07 µs 517.98 µs]

msm_g1/blst_msm_g1/9    time:   [327.15 µs 328.27 µs 329.63 µs]
msm_g1/naive_msm_g1/9   time:   [592.55 µs 597.56 µs 602.66 µs]

msm_g1/blst_msm_g1/10   time:   [356.33 µs 357.50 µs 359.16 µs]
msm_g1/naive_msm_g1/10  time:   [641.47 µs 644.10 µs 647.03 µs]

msm_g1/blst_msm_g1/11   time:   [383.83 µs 384.44 µs 385.37 µs]
msm_g1/naive_msm_g1/11  time:   [697.56 µs 697.95 µs 698.35 µs]

msm_g1/blst_msm_g1/12   time:   [411.25 µs 412.54 µs 413.98 µs]
msm_g1/naive_msm_g1/12  time:   [764.39 µs 766.27 µs 768.88 µs]

msm_g1/blst_msm_g1/13   time:   [437.52 µs 437.69 µs 437.87 µs]
msm_g1/naive_msm_g1/13  time:   [830.68 µs 835.00 µs 840.41 µs]

msm_g1/blst_msm_g1/14   time:   [482.22 µs 489.38 µs 497.52 µs]
msm_g1/naive_msm_g1/14  time:   [940.43 µs 943.49 µs 946.86 µs]

msm_g1/blst_msm_g1/15   time:   [519.09 µs 521.77 µs 525.01 µs]
msm_g1/naive_msm_g1/15  time:   [977.30 µs 982.38 µs 989.91 µs]

msm_g1/blst_msm_g1/16   time:   [590.32 µs 594.51 µs 600.19 µs]
msm_g1/naive_msm_g1/16  time:   [1.0511 ms 1.0542 ms 1.0581 ms]

msm_g1/blst_msm_g1/17   time:   [600.45 µs 603.51 µs 607.27 µs]
msm_g1/naive_msm_g1/17  time:   [1.1022 ms 1.1105 ms 1.1195 ms]

msm_g1/blst_msm_g1/18   time:   [624.73 µs 627.50 µs 630.48 µs]
msm_g1/naive_msm_g1/18  time:   [1.2077 ms 1.2297 ms 1.2497 ms]

msm_g1/blst_msm_g1/19   time:   [653.60 µs 657.08 µs 661.40 µs]
msm_g1/naive_msm_g1/19  time:   [1.2643 ms 1.2721 ms 1.2816 ms]

msm_g1/blst_msm_g1/20   time:   [682.46 µs 685.99 µs 689.59 µs]
msm_g1/naive_msm_g1/20  time:   [1.3012 ms 1.3035 ms 1.3061 ms]

msm_g1/blst_msm_g1/21   time:   [684.61 µs 686.25 µs 688.33 µs]
msm_g1/naive_msm_g1/21  time:   [1.3331 ms 1.3348 ms 1.3368 ms]

msm_g1/blst_msm_g1/22   time:   [705.40 µs 713.53 µs 724.16 µs]
msm_g1/naive_msm_g1/22  time:   [1.3992 ms 1.4048 ms 1.4123 ms]

msm_g1/blst_msm_g1/23   time:   [720.92 µs 724.14 µs 727.81 µs]
msm_g1/naive_msm_g1/23  time:   [1.4583 ms 1.4633 ms 1.4715 ms]

msm_g1/blst_msm_g1/24   time:   [734.53 µs 736.28 µs 738.45 µs]
msm_g1/naive_msm_g1/24  time:   [1.5225 ms 1.5242 ms 1.5265 ms]

msm_g1/blst_msm_g1/25   time:   [766.10 µs 768.41 µs 771.13 µs]
msm_g1/naive_msm_g1/25  time:   [1.5854 ms 1.5862 ms 1.5871 ms]

msm_g1/blst_msm_g1/26   time:   [779.60 µs 782.51 µs 788.30 µs]
msm_g1/naive_msm_g1/26  time:   [1.6521 ms 1.6624 ms 1.6769 ms]

msm_g1/blst_msm_g1/27   time:   [809.50 µs 811.85 µs 815.43 µs]
msm_g1/naive_msm_g1/27  time:   [1.7139 ms 1.7252 ms 1.7405 ms]

msm_g1/blst_msm_g1/28   time:   [832.86 µs 840.26 µs 848.27 µs]
msm_g1/naive_msm_g1/28  time:   [1.7830 ms 1.7850 ms 1.7874 ms]

msm_g1/blst_msm_g1/29   time:   [906.73 µs 911.70 µs 916.96 µs]
msm_g1/naive_msm_g1/29  time:   [1.9243 ms 1.9387 ms 1.9534 ms]

msm_g1/blst_msm_g1/30   time:   [935.51 µs 942.25 µs 949.50 µs]
msm_g1/naive_msm_g1/30  time:   [2.0355 ms 2.0459 ms 2.0573 ms]

msm_g1/blst_msm_g1/31   time:   [953.61 µs 962.80 µs 972.33 µs]
msm_g1/naive_msm_g1/31  time:   [2.1051 ms 2.1269 ms 2.1608 ms]

msm_g1/blst_msm_g1/32   time:   [322.23 µs 327.55 µs 333.14 µs]
msm_g1/naive_msm_g1/32  time:   [2.1987 ms 2.2107 ms 2.2240 ms]

msm_g1/blst_msm_g1/33   time:   [302.16 µs 317.17 µs 332.76 µs]
msm_g1/naive_msm_g1/33  time:   [2.1608 ms 2.1695 ms 2.1802 ms]

msm_g1/blst_msm_g1/34   time:   [324.97 µs 333.31 µs 341.82 µs]
msm_g1/naive_msm_g1/34  time:   [2.3525 ms 2.3816 ms 2.4177 ms]

msm_g1/blst_msm_g1/35   time:   [361.69 µs 368.30 µs 374.79 µs]
msm_g1/naive_msm_g1/35  time:   [2.3242 ms 2.3296 ms 2.3351 ms]

msm_g1/blst_msm_g1/36   time:   [332.74 µs 348.71 µs 364.02 µs]
msm_g1/naive_msm_g1/36  time:   [2.3552 ms 2.3654 ms 2.3773 ms]

msm_g1/blst_msm_g1/37   time:   [362.15 µs 373.39 µs 383.56 µs]
msm_g1/naive_msm_g1/37  time:   [2.4274 ms 2.4350 ms 2.4436 ms]

msm_g1/blst_msm_g1/38   time:   [365.44 µs 376.32 µs 386.67 µs]
msm_g1/naive_msm_g1/38  time:   [2.4723 ms 2.4822 ms 2.4964 ms]

msm_g1/blst_msm_g1/39   time:   [353.72 µs 368.08 µs 381.12 µs]
msm_g1/naive_msm_g1/39  time:   [2.5414 ms 2.5462 ms 2.5521 ms]

msm_g1/blst_msm_g1/40   time:   [397.98 µs 407.80 µs 417.39 µs]
msm_g1/naive_msm_g1/40  time:   [2.5962 ms 2.6054 ms 2.6162 ms]

msm_g1/blst_msm_g1/41   time:   [358.73 µs 378.07 µs 397.10 µs]
msm_g1/naive_msm_g1/41  time:   [2.6497 ms 2.6591 ms 2.6709 ms]


===================================================================

msm_g2/blst_msm_g2/1    time:   [130.79 µs 131.08 µs 131.37 µs]
msm_g2/naive_msm_g2/1   time:   [127.49 µs 128.78 µs 130.41 µs]

msm_g2/blst_msm_g2/2    time:   [281.34 µs 282.01 µs 282.77 µs]
msm_g2/naive_msm_g2/2   time:   [255.11 µs 258.75 µs 264.49 µs]

msm_g2/blst_msm_g2/3    time:   [356.00 µs 358.74 µs 361.86 µs]
msm_g2/naive_msm_g2/3   time:   [380.37 µs 381.51 µs 382.92 µs]

msm_g2/blst_msm_g2/4    time:   [423.17 µs 425.23 µs 427.89 µs]
msm_g2/naive_msm_g2/4   time:   [508.57 µs 510.44 µs 512.82 µs]

msm_g2/blst_msm_g2/5    time:   [495.00 µs 498.04 µs 501.54 µs]
msm_g2/naive_msm_g2/5   time:   [626.29 µs 628.14 µs 630.04 µs]

msm_g2/blst_msm_g2/6    time:   [562.57 µs 563.78 µs 565.33 µs]
msm_g2/naive_msm_g2/6   time:   [750.41 µs 755.53 µs 761.99 µs]

msm_g2/blst_msm_g2/7    time:   [637.84 µs 640.76 µs 643.95 µs]
msm_g2/naive_msm_g2/7   time:   [884.03 µs 886.33 µs 888.88 µs]

msm_g2/blst_msm_g2/8    time:   [708.08 µs 711.34 µs 715.90 µs]
msm_g2/naive_msm_g2/8   time:   [1.0038 ms 1.0073 ms 1.0118 ms]

msm_g2/blst_msm_g2/9    time:   [787.35 µs 793.62 µs 800.29 µs]
msm_g2/naive_msm_g2/9   time:   [1.1366 ms 1.1421 ms 1.1496 ms]

msm_g2/blst_msm_g2/10   time:   [863.02 µs 865.28 µs 867.78 µs]
msm_g2/naive_msm_g2/10  time:   [1.2667 ms 1.2720 ms 1.2779 ms]

msm_g2/blst_msm_g2/11   time:   [936.26 µs 941.11 µs 946.98 µs]
msm_g2/naive_msm_g2/11  time:   [1.3970 ms 1.4029 ms 1.4093 ms]

msm_g2/blst_msm_g2/12   time:   [989.58 µs 992.17 µs 995.00 µs]
msm_g2/naive_msm_g2/12  time:   [1.5040 ms 1.5088 ms 1.5149 ms]

msm_g2/blst_msm_g2/13   time:   [1.0663 ms 1.0723 ms 1.0801 ms]
msm_g2/naive_msm_g2/13  time:   [1.6279 ms 1.6384 ms 1.6503 ms]

msm_g2/blst_msm_g2/14   time:   [1.1410 ms 1.1431 ms 1.1452 ms]
msm_g2/naive_msm_g2/14  time:   [1.7542 ms 1.7620 ms 1.7720 ms]

msm_g2/blst_msm_g2/15   time:   [1.2108 ms 1.2175 ms 1.2280 ms]
msm_g2/naive_msm_g2/15  time:   [1.8947 ms 1.9027 ms 1.9122 ms]

msm_g2/blst_msm_g2/16   time:   [1.1995 ms 1.2091 ms 1.2240 ms]
msm_g2/naive_msm_g2/16  time:   [2.0006 ms 2.0048 ms 2.0106 ms]

msm_g2/blst_msm_g2/17   time:   [1.2464 ms 1.2531 ms 1.2616 ms]
msm_g2/naive_msm_g2/17  time:   [2.1436 ms 2.1577 ms 2.1760 ms]

msm_g2/blst_msm_g2/18   time:   [1.2874 ms 1.2899 ms 1.2926 ms]
msm_g2/naive_msm_g2/18  time:   [2.2635 ms 2.2736 ms 2.2899 ms]

msm_g2/blst_msm_g2/19   time:   [1.3350 ms 1.3380 ms 1.3422 ms]
msm_g2/naive_msm_g2/19  time:   [2.3338 ms 2.3392 ms 2.3454 ms]

msm_g2/blst_msm_g2/20   time:   [1.3755 ms 1.3796 ms 1.3843 ms]
msm_g2/naive_msm_g2/20  time:   [2.4673 ms 2.4865 ms 2.5103 ms]

msm_g2/blst_msm_g2/21   time:   [1.4749 ms 1.4796 ms 1.4855 ms]
msm_g2/naive_msm_g2/21  time:   [2.6430 ms 2.6553 ms 2.6692 ms]

msm_g2/blst_msm_g2/22   time:   [1.5342 ms 1.5415 ms 1.5501 ms]
msm_g2/naive_msm_g2/22  time:   [2.7882 ms 2.7919 ms 2.7958 ms]

msm_g2/blst_msm_g2/23   time:   [1.6175 ms 1.6215 ms 1.6256 ms]
msm_g2/naive_msm_g2/23  time:   [2.8990 ms 2.9193 ms 2.9450 ms]

msm_g2/blst_msm_g2/24   time:   [1.6405 ms 1.6432 ms 1.6460 ms]
msm_g2/naive_msm_g2/24  time:   [3.0091 ms 3.0131 ms 3.0174 ms]

msm_g2/blst_msm_g2/25   time:   [1.7311 ms 1.7399 ms 1.7515 ms]
msm_g2/naive_msm_g2/25  time:   [3.1444 ms 3.1584 ms 3.1766 ms]

msm_g2/blst_msm_g2/26   time:   [1.7763 ms 1.7845 ms 1.7946 ms]
msm_g2/naive_msm_g2/26  time:   [3.2594 ms 3.2637 ms 3.2681 ms]

msm_g2/blst_msm_g2/27   time:   [1.8289 ms 1.8356 ms 1.8436 ms]
msm_g2/naive_msm_g2/27  time:   [3.3189 ms 3.3221 ms 3.3255 ms]

msm_g2/blst_msm_g2/28   time:   [1.8165 ms 1.8189 ms 1.8215 ms]
msm_g2/naive_msm_g2/28  time:   [3.4294 ms 3.4344 ms 3.4401 ms]

msm_g2/blst_msm_g2/29   time:   [1.8918 ms 1.8970 ms 1.9034 ms]
msm_g2/naive_msm_g2/29  time:   [3.5385 ms 3.5432 ms 3.5502 ms]

msm_g2/blst_msm_g2/30   time:   [1.9231 ms 1.9246 ms 1.9262 ms]
msm_g2/naive_msm_g2/30  time:   [3.6693 ms 3.6766 ms 3.6881 ms]

msm_g2/blst_msm_g2/31   time:   [2.0009 ms 2.0096 ms 2.0201 ms]
msm_g2/naive_msm_g2/31  time:   [3.7883 ms 3.7951 ms 3.8054 ms]

msm_g2/blst_msm_g2/32   time:   [801.85 µs 817.52 µs 831.76 µs]
msm_g2/naive_msm_g2/32  time:   [4.0272 ms 4.0659 ms 4.1119 ms]

msm_g2/blst_msm_g2/33   time:   [806.27 µs 823.81 µs 839.82 µs]
msm_g2/naive_msm_g2/33  time:   [4.2425 ms 4.2757 ms 4.3138 ms]

msm_g2/blst_msm_g2/34   time:   [849.62 µs 862.59 µs 874.06 µs]
msm_g2/naive_msm_g2/34  time:   [4.3680 ms 4.3826 ms 4.3989 ms]

msm_g2/blst_msm_g2/35   time:   [896.76 µs 919.98 µs 939.49 µs]
msm_g2/naive_msm_g2/35  time:   [4.5131 ms 4.5346 ms 4.5599 ms]

msm_g2/blst_msm_g2/36   time:   [924.64 µs 936.50 µs 947.44 µs]
msm_g2/naive_msm_g2/36  time:   [4.6243 ms 4.6610 ms 4.7087 ms]

msm_g2/blst_msm_g2/37   time:   [903.17 µs 930.37 µs 954.96 µs]
msm_g2/naive_msm_g2/37  time:   [4.7268 ms 4.7409 ms 4.7552 ms]

msm_g2/blst_msm_g2/38   time:   [933.70 µs 959.11 µs 982.50 µs]
msm_g2/naive_msm_g2/38  time:   [4.9295 ms 4.9497 ms 4.9716 ms]

msm_g2/blst_msm_g2/39   time:   [888.38 µs 926.10 µs 963.41 µs]
msm_g2/naive_msm_g2/39  time:   [5.0258 ms 5.0385 ms 5.0526 ms]

msm_g2/blst_msm_g2/40   time:   [1.0077 ms 1.0182 ms 1.0289 ms]
msm_g2/naive_msm_g2/40  time:   [5.1006 ms 5.1100 ms 5.1198 ms]

msm_g2/blst_msm_g2/41   time:   [935.58 µs 972.47 µs 1.0085 ms]
msm_g2/naive_msm_g2/41  time:   [5.1966 ms 5.2143 ms 5.2333 ms]

```