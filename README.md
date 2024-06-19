# A quick benchmark on MSM vs the naive approach
To run bench yourself, try `nix run .#defaultPackage.x86_64-linux` (change to your system if needed).
```bash
GG1Projective - Size: 1 - MSM Average Time: 89.227µs - MSM Std Dev: 143.57µs - Naive Average Time: 73.962µs - Naive Std Dev: 23.435µs - Naive to MSM Ratio: 0.8289194974615307
G1Projective - Size: 2 - MSM Average Time: 138.717µs - MSM Std Dev: 26.985µs - Naive Average Time: 146.213µs - Naive Std Dev: 28.999µs - Naive to MSM Ratio: 1.0540380775247447
G1Projective - Size: 3 - MSM Average Time: 237.419µs - MSM Std Dev: 95.963µs - Naive Average Time: 299.41µs - Naive Std Dev: 125.289µs - Naive to MSM Ratio: 1.2611037869757686
G1Projective - Size: 4 - MSM Average Time: 215.551µs - MSM Std Dev: 53.659µs - Naive Average Time: 301.684µs - Naive Std Dev: 71.184µs - Naive to MSM Ratio: 1.3995945275132102
G1Projective - Size: 5 - MSM Average Time: 353.056µs - MSM Std Dev: 193.706µs - Naive Average Time: 520.482µs - Naive Std Dev: 303.578µs - Naive to MSM Ratio: 1.4742193872926674
G1Projective - Size: 6 - MSM Average Time: 276.117µs - MSM Std Dev: 42.683µs - Naive Average Time: 442.348µs - Naive Std Dev: 68.399µs - Naive to MSM Ratio: 1.6020310230807955
G1Projective - Size: 7 - MSM Average Time: 384.568µs - MSM Std Dev: 146.076µs - Naive Average Time: 620.618µs - Naive Std Dev: 231.989µs - Naive to MSM Ratio: 1.6138056208524891
G1Projective - Size: 8 - MSM Average Time: 407.197µs - MSM Std Dev: 140.839µs - Naive Average Time: 664.402µs - Naive Std Dev: 241.357µs - Naive to MSM Ratio: 1.6316475809006452
G1Projective - Size: 9 - MSM Average Time: 462.284µs - MSM Std Dev: 225.745µs - Naive Average Time: 787.702µs - Naive Std Dev: 379.309µs - Naive to MSM Ratio: 1.7039352432703705
G1Projective - Size: 10 - MSM Average Time: 444.315µs - MSM Std Dev: 158.736µs - Naive Average Time: 767.493µs - Naive Std Dev: 273.417µs - Naive to MSM Ratio: 1.7273623442827724
G1Projective - Size: 11 - MSM Average Time: 518.557µs - MSM Std Dev: 217.475µs - Naive Average Time: 894.322µs - Naive Std Dev: 357.481µs - Naive to MSM Ratio: 1.7246358645240543
G1Projective - Size: 12 - MSM Average Time: 510.383µs - MSM Std Dev: 158.998µs - Naive Average Time: 892.138µs - Naive Std Dev: 262.996µs - Naive to MSM Ratio: 1.7479774992505628
G1Projective - Size: 13 - MSM Average Time: 603.11µs - MSM Std Dev: 280.101µs - Naive Average Time: 1.044818ms - Naive Std Dev: 431.964µs - Naive to MSM Ratio: 1.7323838105818177
G1Projective - Size: 14 - MSM Average Time: 692.788µs - MSM Std Dev: 401.064µs - Naive Average Time: 1.233279ms - Naive Std Dev: 638.84µs - Naive to MSM Ratio: 1.7801679590293134
G1Projective - Size: 15 - MSM Average Time: 664.615µs - MSM Std Dev: 302.706µs - Naive Average Time: 1.209615ms - Naive Std Dev: 576.482µs - Naive to MSM Ratio: 1.8200236226988558
G1Projective - Size: 16 - MSM Average Time: 731.069µs - MSM Std Dev: 304.914µs - Naive Average Time: 1.25427ms - Naive Std Dev: 466.456µs - Naive to MSM Ratio: 1.7156656895587148
G1Projective - Size: 17 - MSM Average Time: 750.054µs - MSM Std Dev: 343.722µs - Naive Average Time: 1.34371ms - Naive Std Dev: 642.827µs - Naive to MSM Ratio: 1.7914843464603882
G1Projective - Size: 18 - MSM Average Time: 666.669µs - MSM Std Dev: 165.008µs - Naive Average Time: 1.205121ms - Naive Std Dev: 240.499µs - Naive to MSM Ratio: 1.807675173136894
G1Projective - Size: 19 - MSM Average Time: 756.848µs - MSM Std Dev: 234.984µs - Naive Average Time: 1.38854ms - Naive Std Dev: 417.681µs - Naive to MSM Ratio: 1.8346352239815658
G1Projective - Size: 20 - MSM Average Time: 747.381µs - MSM Std Dev: 212.11µs - Naive Average Time: 1.39963ms - Naive Std Dev: 331.789µs - Naive to MSM Ratio: 1.8727128465936382
G1Projective - Size: 21 - MSM Average Time: 842.568µs - MSM Std Dev: 315.952µs - Naive Average Time: 1.521557ms - Naive Std Dev: 462.367µs - Naive to MSM Ratio: 1.8058566192877015
G1Projective - Size: 22 - MSM Average Time: 899.885µs - MSM Std Dev: 361.528µs - Naive Average Time: 1.633544ms - Naive Std Dev: 550.958µs - Naive to MSM Ratio: 1.8152808414408508
G1Projective - Size: 23 - MSM Average Time: 953µs - MSM Std Dev: 357.979µs - Naive Average Time: 1.77712ms - Naive Std Dev: 667.148µs - Naive to MSM Ratio: 1.8647639034627492
G1Projective - Size: 24 - MSM Average Time: 999.119µs - MSM Std Dev: 398.572µs - Naive Average Time: 1.870514ms - Naive Std Dev: 666.365µs - Naive to MSM Ratio: 1.872163375934198
G1Projective - Size: 25 - MSM Average Time: 953.29µs - MSM Std Dev: 318.983µs - Naive Average Time: 1.762488ms - Naive Std Dev: 508.834µs - Naive to MSM Ratio: 1.8488476748943135
G1Projective - Size: 26 - MSM Average Time: 913.857µs - MSM Std Dev: 301.124µs - Naive Average Time: 1.755803ms - Naive Std Dev: 356.746µs - Naive to MSM Ratio: 1.9213104457261914
G1Projective - Size: 27 - MSM Average Time: 957.959µs - MSM Std Dev: 293.55µs - Naive Average Time: 1.913943ms - Naive Std Dev: 559.702µs - Naive to MSM Ratio: 1.9979383251266494
G1Projective - Size: 28 - MSM Average Time: 1.024603ms - MSM Std Dev: 391.231µs - Naive Average Time: 2.042804ms - Naive Std Dev: 728.606µs - Naive to MSM Ratio: 1.9937517262783733
G1Projective - Size: 29 - MSM Average Time: 1.097588ms - MSM Std Dev: 361.214µs - Naive Average Time: 2.064051ms - Naive Std Dev: 531.304µs - Naive to MSM Ratio: 1.8805334970863383
G1Projective - Size: 30 - MSM Average Time: 1.094718ms - MSM Std Dev: 420.536µs - Naive Average Time: 2.164049ms - Naive Std Dev: 637.704µs - Naive to MSM Ratio: 1.9768095527798026
G1Projective - Size: 31 - MSM Average Time: 1.211046ms - MSM Std Dev: 456.441µs - Naive Average Time: 2.253952ms - Naive Std Dev: 685.804µs - Naive to MSM Ratio: 1.8611613431694585
G1Projective - Size: 32 - MSM Average Time: 378.103µs - MSM Std Dev: 94.762µs - Naive Average Time: 2.537102ms - Naive Std Dev: 775.129µs - Naive to MSM Ratio: 6.710081644419642
G1Projective - Size: 33 - MSM Average Time: 407.255µs - MSM Std Dev: 116.415µs - Naive Average Time: 2.895892ms - Naive Std Dev: 1.214541ms - Naive to MSM Ratio: 7.1107586156093845
G1Projective - Size: 34 - MSM Average Time: 373.978µs - MSM Std Dev: 107.666µs - Naive Average Time: 2.566948ms - Naive Std Dev: 710.144µs - Naive to MSM Ratio: 6.863901085090567
G1Projective - Size: 35 - MSM Average Time: 409.644µs - MSM Std Dev: 107.123µs - Naive Average Time: 2.722874ms - Naive Std Dev: 889.675µs - Naive to MSM Ratio: 6.646927576139282
G1Projective - Size: 36 - MSM Average Time: 409.801µs - MSM Std Dev: 94.569µs - Naive Average Time: 2.74043ms - Naive Std Dev: 686.504µs - Naive to MSM Ratio: 6.687221358659447
G1Projective - Size: 37 - MSM Average Time: 417.378µs - MSM Std Dev: 113.631µs - Naive Average Time: 2.750595ms - Naive Std Dev: 726.508µs - Naive to MSM Ratio: 6.590177249399825
G1Projective - Size: 38 - MSM Average Time: 421.027µs - MSM Std Dev: 103.049µs - Naive Average Time: 3.13278ms - Naive Std Dev: 1.049968ms - Naive to MSM Ratio: 7.440805459032319
G1Projective - Size: 39 - MSM Average Time: 422.283µs - MSM Std Dev: 106.348µs - Naive Average Time: 2.859391ms - Naive Std Dev: 707.113µs - Naive to MSM Ratio: 6.771267136020157
G1Projective - Size: 40 - MSM Average Time: 413.494µs - MSM Std Dev: 94.613µs - Naive Average Time: 3.112845ms - Naive Std Dev: 995.884µs - Naive to MSM Ratio: 7.528150348009887
G1Projective - Size: 41 - MSM Average Time: 418.806µs - MSM Std Dev: 84.434µs - Naive Average Time: 3.031095ms - Naive Std Dev: 686.378µs - Naive to MSM Ratio: 7.237467944585321

G2Projective - Size: 1 - MSM Average Time: 144.613µs - MSM Std Dev: 46.011µs - Naive Average Time: 142.431µs - Naive Std Dev: 45.13µs - Naive to MSM Ratio: 0.9849114533271559
G2Projective - Size: 2 - MSM Average Time: 315.644µs - MSM Std Dev: 110.285µs - Naive Average Time: 281.303µs - Naive Std Dev: 100.9µs - Naive to MSM Ratio: 0.8912033810241918
G2Projective - Size: 3 - MSM Average Time: 376.412µs - MSM Std Dev: 9.271µs - Naive Average Time: 394.095µs - Naive Std Dev: 8.516µs - Naive to MSM Ratio: 1.0469777796669606
G2Projective - Size: 4 - MSM Average Time: 441.175µs - MSM Std Dev: 22.629µs - Naive Average Time: 508.536µs - Naive Std Dev: 25.463µs - Naive to MSM Ratio: 1.1526854422848076
G2Projective - Size: 5 - MSM Average Time: 506.701µs - MSM Std Dev: 20.031µs - Naive Average Time: 622.616µs - Naive Std Dev: 18.435µs - Naive to MSM Ratio: 1.2287641034850927
G2Projective - Size: 6 - MSM Average Time: 624.002µs - MSM Std Dev: 201.498µs - Naive Average Time: 801.969µs - Naive Std Dev: 265.223µs - Naive to MSM Ratio: 1.2852026115300912
G2Projective - Size: 7 - MSM Average Time: 770.791µs - MSM Std Dev: 335.292µs - Naive Average Time: 1.007379ms - Naive Std Dev: 409.463µs - Naive to MSM Ratio: 1.3069418298864413
G2Projective - Size: 8 - MSM Average Time: 733.107µs - MSM Std Dev: 116.905µs - Naive Average Time: 1.01835ms - Naive Std Dev: 161.719µs - Naive to MSM Ratio: 1.389087813920751
G2Projective - Size: 9 - MSM Average Time: 798.928µs - MSM Std Dev: 70.953µs - Naive Average Time: 1.11135ms - Naive Std Dev: 22.782µs - Naive to MSM Ratio: 1.3910515090220896
G2Projective - Size: 10 - MSM Average Time: 922.229µs - MSM Std Dev: 289.113µs - Naive Average Time: 1.291421ms - Naive Std Dev: 300.129µs - Naive to MSM Ratio: 1.4003257325458212
G2Projective - Size: 11 - MSM Average Time: 1.012195ms - MSM Std Dev: 332.042µs - Naive Average Time: 1.435639ms - Naive Std Dev: 387.908µs - Naive to MSM Ratio: 1.4183423154629295
G2Projective - Size: 12 - MSM Average Time: 1.226671ms - MSM Std Dev: 498.316µs - Naive Average Time: 1.654394ms - Naive Std Dev: 570.547µs - Naive to MSM Ratio: 1.3486859964896862
G2Projective - Size: 13 - MSM Average Time: 1.306968ms - MSM Std Dev: 561.092µs - Naive Average Time: 1.812887ms - Naive Std Dev: 632.553µs - Naive to MSM Ratio: 1.3870936396300444
G2Projective - Size: 14 - MSM Average Time: 1.252443ms - MSM Std Dev: 284.915µs - Naive Average Time: 1.764936ms - Naive Std Dev: 145.809µs - Naive to MSM Ratio: 1.409194669937075
G2Projective - Size: 15 - MSM Average Time: 1.267646ms - MSM Std Dev: 238.452µs - Naive Average Time: 1.862515ms - Naive Std Dev: 67.69µs - Naive to MSM Ratio: 1.4692706007828684
G2Projective - Size: 16 - MSM Average Time: 1.43821ms - MSM Std Dev: 541.339µs - Naive Average Time: 2.143904ms - Naive Std Dev: 573.79µs - Naive to MSM Ratio: 1.4906752143289228
G2Projective - Size: 17 - MSM Average Time: 1.403898ms - MSM Std Dev: 421.679µs - Naive Average Time: 2.237836ms - Naive Std Dev: 595.05µs - Naive to MSM Ratio: 1.5940160894879827
G2Projective - Size: 18 - MSM Average Time: 1.469629ms - MSM Std Dev: 484.995µs - Naive Average Time: 2.35469ms - Naive Std Dev: 570.406µs - Naive to MSM Ratio: 1.6022343053927215
G2Projective - Size: 19 - MSM Average Time: 1.54958ms - MSM Std Dev: 441.282µs - Naive Average Time: 2.449319ms - Naive Std Dev: 473.167µs - Naive to MSM Ratio: 1.580634107306496
G2Projective - Size: 20 - MSM Average Time: 1.728941ms - MSM Std Dev: 593.403µs - Naive Average Time: 2.573185ms - Naive Std Dev: 354.304µs - Naive to MSM Ratio: 1.4883012202267167
G2Projective - Size: 21 - MSM Average Time: 1.667489ms - MSM Std Dev: 394.965µs - Naive Average Time: 2.668279ms - Naive Std Dev: 457.089µs - Naive to MSM Ratio: 1.600177872237838
G2Projective - Size: 22 - MSM Average Time: 1.76643ms - MSM Std Dev: 513.105µs - Naive Average Time: 2.824621ms - Naive Std Dev: 517.264µs - Naive to MSM Ratio: 1.5990562886726336
G2Projective - Size: 23 - MSM Average Time: 1.714572ms - MSM Std Dev: 426.207µs - Naive Average Time: 2.900628ms - Naive Std Dev: 462.603µs - Naive to MSM Ratio: 1.691750477670229
G2Projective - Size: 24 - MSM Average Time: 1.829671ms - MSM Std Dev: 561.375µs - Naive Average Time: 3.071933ms - Naive Std Dev: 629.851µs - Naive to MSM Ratio: 1.678953757260185
G2Projective - Size: 25 - MSM Average Time: 1.913209ms - MSM Std Dev: 516.435µs - Naive Average Time: 3.220811ms - Naive Std Dev: 670.15µs - Naive to MSM Ratio: 1.6834600924415473
G2Projective - Size: 26 - MSM Average Time: 1.906439ms - MSM Std Dev: 404.358µs - Naive Average Time: 3.26207ms - Naive Std Dev: 290.872µs - Naive to MSM Ratio: 1.7110801866726395
G2Projective - Size: 27 - MSM Average Time: 1.957683ms - MSM Std Dev: 468.982µs - Naive Average Time: 3.422174ms - Naive Std Dev: 560.927µs - Naive to MSM Ratio: 1.7480736155955792
G2Projective - Size: 28 - MSM Average Time: 2.254738ms - MSM Std Dev: 812.268µs - Naive Average Time: 3.629794ms - Naive Std Dev: 667.566µs - Naive to MSM Ratio: 1.6098517876578122
G2Projective - Size: 29 - MSM Average Time: 2.134949ms - MSM Std Dev: 656.273µs - Naive Average Time: 3.740288ms - Naive Std Dev: 875.936µs - Naive to MSM Ratio: 1.751933184352413
G2Projective - Size: 30 - MSM Average Time: 2.255676ms - MSM Std Dev: 618.002µs - Naive Average Time: 3.817414ms - Naive Std Dev: 610.123µs - Naive to MSM Ratio: 1.6923591863370449
G2Projective - Size: 31 - MSM Average Time: 2.186656ms - MSM Std Dev: 404.656µs - Naive Average Time: 3.872118ms - Naive Std Dev: 201.205µs - Naive to MSM Ratio: 1.7707943087527256
G2Projective - Size: 32 - MSM Average Time: 842.82µs - MSM Std Dev: 219.579µs - Naive Average Time: 4.562372ms - Naive Std Dev: 1.366164ms - Naive to MSM Ratio: 5.413222277591894
G2Projective - Size: 33 - MSM Average Time: 842.69µs - MSM Std Dev: 276.422µs - Naive Average Time: 4.466157ms - Naive Std Dev: 903.727µs - Naive to MSM Ratio: 5.299881332399815
G2Projective - Size: 34 - MSM Average Time: 884.668µs - MSM Std Dev: 245.906µs - Naive Average Time: 4.866759ms - Naive Std Dev: 1.52571ms - Naive to MSM Ratio: 5.50122644879209
G2Projective - Size: 35 - MSM Average Time: 788.714µs - MSM Std Dev: 189.603µs - Naive Average Time: 4.66107ms - Naive Std Dev: 1.092735ms - Naive to MSM Ratio: 5.909708715706834
G2Projective - Size: 36 - MSM Average Time: 892.497µs - MSM Std Dev: 212.425µs - Naive Average Time: 4.852224ms - Naive Std Dev: 950.856µs - Naive to MSM Ratio: 5.4366838207859525
G2Projective - Size: 37 - MSM Average Time: 907.903µs - MSM Std Dev: 233.314µs - Naive Average Time: 5.189886ms - Naive Std Dev: 1.4087ms - Naive to MSM Ratio: 5.71634414689675
G2Projective - Size: 38 - MSM Average Time: 1.050559ms - MSM Std Dev: 264.688µs - Naive Average Time: 5.531868ms - Naive Std Dev: 1.502463ms - Naive to MSM Ratio: 5.265642386577051
G2Projective - Size: 39 - MSM Average Time: 1.105243ms - MSM Std Dev: 266.71µs - Naive Average Time: 5.851321ms - Naive Std Dev: 1.720769ms - Naive to MSM Ratio: 5.294148888524967
G2Projective - Size: 40 - MSM Average Time: 1.012361ms - MSM Std Dev: 248.703µs - Naive Average Time: 5.91932ms - Naive Std Dev: 1.521406ms - Naive to MSM Ratio: 5.847044680701845
G2Projective - Size: 41 - MSM Average Time: 1.015959ms - MSM Std Dev: 250.828µs - Naive Average Time: 5.682167ms - Naive Std Dev: 886.491µs - Naive to MSM Ratio: 5.592909753247916
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