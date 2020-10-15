window.BENCHMARK_DATA = {
  "lastUpdate": 1602734940555,
  "repoUrl": "https://github.com/echeran/icu4x",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "elango@google.com",
            "name": "Elango",
            "username": "echeran"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6d3e0a1561fd72797ea50b64077e2c4281d85dd5",
          "message": "Merge pull request #17 from echeran/ci-bench-fixed-decimal\n\nTesting PR - in personal fork - enable utils/fixed-decimal benchmark",
          "timestamp": "2020-10-14T21:04:09-07:00",
          "tree_id": "0f8610177898fa1eed700f55e45ecb6f2e6d3a01",
          "url": "https://github.com/echeran/icu4x/commit/6d3e0a1561fd72797ea50b64077e2c4281d85dd5"
        },
        "date": 1602734939850,
        "tool": "cargo",
        "benches": [
          {
            "name": "plurals/operands/overview",
            "value": 1297,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "plurals/parser/overview",
            "value": 6734,
            "range": "± 365",
            "unit": "ns/iter"
          },
          {
            "name": "plurals/pluralrules/overview",
            "value": 124679,
            "range": "± 10786",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}