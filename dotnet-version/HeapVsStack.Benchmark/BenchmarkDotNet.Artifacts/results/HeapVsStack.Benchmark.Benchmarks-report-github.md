```

BenchmarkDotNet v0.15.2, macOS Sequoia 15.3 (24D60) [Darwin 24.3.0]
Apple M2, 1 CPU, 8 logical and 8 physical cores
.NET SDK 9.0.102
  [Host]     : .NET 9.0.1 (9.0.124.61010), Arm64 RyuJIT AdvSIMD
  DefaultJob : .NET 9.0.1 (9.0.124.61010), Arm64 RyuJIT AdvSIMD


```
| Method    | Mean      | Error    | StdDev   |
|---------- |----------:|---------:|---------:|
| Scenario1 | 466.50 μs | 2.173 μs | 1.927 μs |
| Scenario2 |  91.58 μs | 1.129 μs | 1.056 μs |
