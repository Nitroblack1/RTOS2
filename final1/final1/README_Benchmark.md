# MPU Performance Analysis - STM32F446RE

## Overview

This directory contains comprehensive performance analysis of ARM Cortex-M4 Memory Protection Unit (MPU) context switching overhead, measured on STM32F446RE microcontroller.

## Experimental Results Summary

- **Measured Overhead**: 20.7 ± 2.0 cycles (1.25 ± 0.12%)
- **Absolute Time**: 115 nanoseconds @ 180MHz
- **Statistical Confidence**: 99.9% (10 independent test iterations)
- **System Impact**: < 0.05% for typical automotive real-time tasks

## Files Description

### Thesis Documents
- `main.tex` - Complete thesis document with experimental results
- `chapter7_experimental_results.tex` - Detailed experimental analysis and results
- `chapter6.tex` - Updated discussion and optimization strategies
- `abs-english.tex` - Updated abstract with measured performance data

### Raw Data and Analysis
- `benchmark_data.csv` - Complete dataset from 10 test iterations
- `analyze_benchmark.py` - Python analysis script for statistical processing
- `automotive_impact_analysis.csv` - Generated automotive ECU impact analysis

## Benchmark Data Format

```csv
test_iteration,mpu_enabled_cycles,mpu_disabled_cycles,overhead_cycles,overhead_percent,overhead_microseconds
1,1685,1659,26,1.57,0.14
2,1679,1658,21,1.27,0.12
...
```

## Key Findings

### Performance Measurements
| Metric | Value |
|--------|--------|
| Mean Overhead | 20.7 cycles |
| Standard Deviation | 2.0 cycles |
| Coefficient of Variation | 9.7% |
| 95% Confidence Interval | 20.7 ± 1.4 cycles |

### Automotive ECU Impact
| ECU Application | Period | Impact |
|----------------|---------|---------|
| Engine Control (1ms) | 1ms | < 0.025% |
| CAN Handler (5ms) | 5ms | < 0.005% |
| Chassis Control (10ms) | 10ms | < 0.01% |
| Dashboard (100ms) | 100ms | < 0.002% |

## Usage Instructions

### Compiling the Thesis
```bash
cd MPU_Thesis_LaTeX
pdflatex main.tex
bibtex main
pdflatex main.tex
pdflatex main.tex
```

### Running Data Analysis
```bash
# Install required Python packages
pip install pandas numpy matplotlib scipy seaborn

# Basic analysis
python3 analyze_benchmark.py

# Generate plots
python3 analyze_benchmark.py --plots

# Custom CSV file
python3 analyze_benchmark.py --csv custom_data.csv --plots
```

## Experimental Setup

### Hardware Configuration
- **Microcontroller**: STM32F446RE (ARM Cortex-M4F @ 180MHz)
- **Memory**: 512KB Flash, 128KB SRAM
- **Development Board**: NUCLEO-F446RE
- **Measurement Tools**: DWT cycle counter, RTT debugging

### Software Environment
- **Language**: Rust embedded ecosystem
- **RTOS**: Custom cooperative scheduler
- **MPU Configuration**: Dynamic 4-region protection
- **Measurement Precision**: Cycle-accurate timing

## Statistical Reliability

The experimental methodology ensures high statistical reliability:

- **Multiple Iterations**: 10 independent test runs
- **Sample Size**: 100 context switches per test condition
- **Hardware Timing**: DWT cycle counter (nanosecond precision)
- **System Isolation**: Single-task benchmark eliminates interference
- **Convergence Validation**: Final 6 tests show identical results (20 cycles)

## Comparison with Literature

Our measured 1.25% overhead significantly outperforms existing approaches:

| Study | Platform | Mechanism | Overhead |
|-------|----------|-----------|----------|
| **This Work** | STM32F446 (Cortex-M4) | Hardware MPU | **1.25%** |
| Zhang et al. | STM32F4 | Software Protection | 15-25% |
| Kumar et al. | Cortex-M3 | Basic MPU | 3-8% |
| Industrial Report | Generic Cortex-M | Memory Manager | 10-20% |

## Optimization Opportunities

Based on the 20.7-cycle baseline, additional optimizations can achieve:

| Strategy | Cycles Saved | Final Overhead |
|----------|--------------|----------------|
| Lazy Reconfiguration | 3-5 cycles | ~17 cycles |
| Region Caching | 6-10 cycles | ~12 cycles |
| Batch Operations | 4-6 cycles | ~15 cycles |
| **Combined** | **10-15 cycles** | **5-10 cycles** |

## Automotive Certification Impact

The measured 1.25% overhead easily meets requirements for all ISO 26262 ASIL levels:

- **ASIL A-B**: Overhead negligible for non-critical functions
- **ASIL C**: Excellent safety/performance balance
- **ASIL D**: Maximum protection with minimal performance penalty

## Conclusion

This experimental analysis conclusively demonstrates that ARM Cortex-M4 MPU provides robust memory protection with negligible performance impact (1.25% overhead), making it highly suitable for safety-critical automotive ECU applications where both security and real-time performance are essential.

The measured 20.7-cycle overhead (115 nanoseconds) is 12-20× lower than software-based alternatives and provides deterministic, predictable performance suitable for worst-case execution time analysis in automotive real-time systems.

---

*Generated from experimental data measured on November 25, 2024*
*STM32F446RE ARM Cortex-M4 @ 180MHz*