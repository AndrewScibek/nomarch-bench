# Current Stats #

Currently these stats are bad because im running non release version on windows. Release flag wont compile on windows so can test this later on unix

### PER TASK METRICS ###

  ---
Name     |   # times run |        # fails |   task/s |  fail/s
----------|--------------|-----------------|---------|----------------------------
   1: Pipeline 1 Complete |        63,762 |         0 (0%) |     2898 |    0.00
   2: Pipeline 1 Missing  |        63,568 |         0 (0%) |     2889 |    0.00
   3: Pipeline 2 Complete |        64,422 |         0 (0%) |     2928 |    0.00
   4: Pipeline 2 Missing  |        63,547 |         0 (0%) |     2888 |    0.00
 Aggregated               |       255,299 |         0 (0%) |    11604 |    0.00

 |Name                     |    Avg (ms) |        Min |         Max |     Median|
  |-------------------------|-------------|------------|-------------|-----------|
 |  1: Pipeline 1 Complete |      155.09 |        134 |         594 |        140|
  | 2: Pipeline 1 Missing  |       77.26 |         66 |         173 |         69|
 |  3: Pipeline 2 Complete |      386.26 |        211 |         468 |        360|
  | 4: Pipeline 2 Missing  |       77.18 |         66 |         173 |         69|
 |Aggregated               |      174.65 |         66 |         594 |        140|


 ### PER REQUEST METRICS ###

---

 Name                     |        # reqs |        # fails |    req/s |  fail/s
  |-------------------------|-------------|------------|-------------|-----------|
 POST Pipeline 1 Complete |       127,315 |         0 (0%) |     5787 |    0.00
 POST Pipeline 1 Missing  |        63,568 |         0 (0%) |     2889 |    0.00
 POST Pipeline 2 Complete |       319,927 |         0 (0%) |    14542 |    0.00
 POST Pipeline 2 Missing  |        63,547 |         0 (0%) |     2888 |    0.00
 Aggregated               |       574,357 |         0 (0%) |    26107 |    0.00


 Name                     |    Avg (ms) |        Min |        Max |      Median
 -------------------------|-------------|------------|-----------|----------------
 POST Pipeline 1 Complete |       77.27 |         30 |         515 |         69
 POST Pipeline 1 Missing  |       77.25 |         66 |         173 |         69
 POST Pipeline 2 Complete |       76.86 |          2 |         173 |         69
 POST Pipeline 2 Missing  |       77.17 |         66 |         173 |         69
 Aggregated               |       77.03 |          2 |         515 |         69

 ---

 Slowest page load within specified percentile of requests (in ms):
 ------------------------------------------------------------------------------
 Name                     |    50% |    75% |    98% |    99% |  99.9% | 99.99%
 -------------------------|--------|--------|--------|--------|--------|-------
 POST Pipeline 1 Complete |     69 |     71 |    170 |    170 |    170 |    170
 POST Pipeline 1 Missing  |     69 |     71 |    170 |    170 |    170 |    170
 POST Pipeline 2 Complete |     69 |     71 |    170 |    170 |    170 |    170
 POST Pipeline 2 Missing  |     69 |     71 |    170 |    170 |    170 |    170
 Aggregated               |     69 |     71 |    170 |    170 |    170 |    170

 Name                     |                                        Status codes
 -------------------------|----------------------------------------------------
 POST Pipeline 1 Complete |                                       127,315 [200]
 POST Pipeline 1 Missing  |                                        63,568 [200]
 POST Pipeline 2 Complete |                                       319,927 [200]
 POST Pipeline 2 Missing  |                                        63,547 [200]
 Aggregated               |                                       574,357 [200]