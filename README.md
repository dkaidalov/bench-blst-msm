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