#!/usr/bin/env python3
"""
MPU Performance Benchmark Data Analysis
STM32F446RE ARM Cortex-M4 Context Switch Overhead Analysis
"""

import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
from scipy import stats
import argparse

def load_benchmark_data(csv_file):
    """Load benchmark data from CSV file."""
    return pd.read_csv(csv_file)

def statistical_analysis(data):
    """Perform comprehensive statistical analysis."""
    overhead_cycles = data['overhead_cycles']
    overhead_percent = data['overhead_percent']

    stats_results = {
        'cycles': {
            'mean': np.mean(overhead_cycles),
            'median': np.median(overhead_cycles),
            'std': np.std(overhead_cycles, ddof=1),
            'min': np.min(overhead_cycles),
            'max': np.max(overhead_cycles),
            'cv': np.std(overhead_cycles, ddof=1) / np.mean(overhead_cycles) * 100,
            'ci_95': stats.t.interval(0.95, len(overhead_cycles)-1,
                                     loc=np.mean(overhead_cycles),
                                     scale=stats.sem(overhead_cycles)),
            'ci_99': stats.t.interval(0.99, len(overhead_cycles)-1,
                                     loc=np.mean(overhead_cycles),
                                     scale=stats.sem(overhead_cycles))
        },
        'percent': {
            'mean': np.mean(overhead_percent),
            'median': np.median(overhead_percent),
            'std': np.std(overhead_percent, ddof=1),
            'min': np.min(overhead_percent),
            'max': np.max(overhead_percent),
            'cv': np.std(overhead_percent, ddof=1) / np.mean(overhead_percent) * 100
        }
    }

    return stats_results

def generate_convergence_plot(data, output_file='convergence_plot.png'):
    """Generate convergence analysis plot."""
    plt.figure(figsize=(10, 6))

    # Plot raw data points
    plt.plot(data['test_iteration'], data['overhead_cycles'],
             'bo-', linewidth=2, markersize=8, label='Measured Overhead')

    # Add mean line
    mean_overhead = data['overhead_cycles'].mean()
    plt.axhline(y=mean_overhead, color='red', linestyle='--',
                label=f'Mean: {mean_overhead:.1f} cycles')

    # Add confidence intervals
    std_overhead = data['overhead_cycles'].std()
    plt.fill_between(data['test_iteration'],
                     mean_overhead - std_overhead,
                     mean_overhead + std_overhead,
                     alpha=0.3, color='red', label='±1σ')

    plt.xlabel('Test Iteration')
    plt.ylabel('MPU Overhead (cycles)')
    plt.title('MPU Context Switch Overhead Convergence Analysis')
    plt.legend()
    plt.grid(True, alpha=0.3)
    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    print(f"Convergence plot saved as {output_file}")

def generate_distribution_plot(data, output_file='distribution_plot.png'):
    """Generate distribution analysis plot."""
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))

    # Histogram
    ax1.hist(data['overhead_cycles'], bins=range(19, 28), alpha=0.7,
             color='skyblue', edgecolor='black')
    ax1.set_xlabel('MPU Overhead (cycles)')
    ax1.set_ylabel('Frequency')
    ax1.set_title('Distribution of MPU Overhead')
    ax1.grid(True, alpha=0.3)

    # Box plot
    ax2.boxplot(data['overhead_cycles'])
    ax2.set_ylabel('MPU Overhead (cycles)')
    ax2.set_title('Statistical Distribution')
    ax2.grid(True, alpha=0.3)

    plt.tight_layout()
    plt.savefig(output_file, dpi=300, bbox_inches='tight')
    print(f"Distribution plot saved as {output_file}")

def generate_automotive_impact_analysis():
    """Generate automotive ECU impact analysis."""

    # Typical automotive ECU task characteristics
    ecu_tasks = {
        'Engine Control (1ms)': {'period_ms': 1, 'context_switches': 3},
        'CAN Handler (5ms)': {'period_ms': 5, 'context_switches': 1},
        'Sensor Processing (10ms)': {'period_ms': 10, 'context_switches': 6},
        'Dashboard Update (100ms)': {'period_ms': 100, 'context_switches': 15},
        'Diagnostic (1000ms)': {'period_ms': 1000, 'context_switches': 75}
    }

    cpu_freq_mhz = 180
    overhead_cycles = 20.7  # Mean measured overhead

    results = []
    for task_name, params in ecu_tasks.items():
        period_cycles = params['period_ms'] * cpu_freq_mhz * 1000
        overhead_per_period = params['context_switches'] * overhead_cycles
        impact_percent = (overhead_per_period / period_cycles) * 100

        results.append({
            'Task': task_name,
            'Period (ms)': params['period_ms'],
            'Context Switches': params['context_switches'],
            'Overhead (cycles)': overhead_per_period,
            'Impact (%)': impact_percent
        })

    return pd.DataFrame(results)

def print_statistical_summary(stats_results):
    """Print comprehensive statistical summary."""
    print("\n" + "="*60)
    print("MPU CONTEXT SWITCH OVERHEAD STATISTICAL ANALYSIS")
    print("="*60)

    cycles = stats_results['cycles']
    percent = stats_results['percent']

    print(f"\nOVERHEAD (CYCLES):")
    print(f"  Mean:               {cycles['mean']:.1f} cycles")
    print(f"  Median:             {cycles['median']:.1f} cycles")
    print(f"  Standard Deviation: {cycles['std']:.1f} cycles")
    print(f"  Min - Max:          {cycles['min']} - {cycles['max']} cycles")
    print(f"  Coefficient of Variation: {cycles['cv']:.1f}%")
    print(f"  95% Confidence Interval: {cycles['ci_95'][0]:.1f} - {cycles['ci_95'][1]:.1f}")
    print(f"  99% Confidence Interval: {cycles['ci_99'][0]:.1f} - {cycles['ci_99'][1]:.1f}")

    print(f"\nOVERHEAD (PERCENTAGE):")
    print(f"  Mean:               {percent['mean']:.2f}%")
    print(f"  Median:             {percent['median']:.2f}%")
    print(f"  Standard Deviation: {percent['std']:.2f}%")
    print(f"  Min - Max:          {percent['min']:.2f}% - {percent['max']:.2f}%")

    # Time analysis
    time_us = cycles['mean'] / 180  # 180MHz CPU
    print(f"\nTIME ANALYSIS:")
    print(f"  Mean Overhead Time: {time_us:.1f} microseconds")
    print(f"  @ 180MHz CPU frequency")

    print("\n" + "="*60)

def main():
    parser = argparse.ArgumentParser(description='Analyze MPU benchmark data')
    parser.add_argument('--csv', default='benchmark_data.csv',
                       help='CSV file containing benchmark data')
    parser.add_argument('--plots', action='store_true',
                       help='Generate analysis plots')
    args = parser.parse_args()

    # Load data
    try:
        data = load_benchmark_data(args.csv)
        print(f"Loaded {len(data)} benchmark measurements from {args.csv}")
    except FileNotFoundError:
        print(f"Error: Cannot find {args.csv}")
        return

    # Statistical analysis
    stats_results = statistical_analysis(data)
    print_statistical_summary(stats_results)

    # Automotive impact analysis
    automotive_analysis = generate_automotive_impact_analysis()
    print("\nAUTOMOTIVE ECU IMPACT ANALYSIS:")
    print("-" * 60)
    print(automotive_analysis.to_string(index=False, float_format='%.4f'))

    # Generate plots if requested
    if args.plots:
        generate_convergence_plot(data)
        generate_distribution_plot(data)
        print("\nPlots generated successfully!")

    # Export results
    automotive_analysis.to_csv('automotive_impact_analysis.csv', index=False)
    print(f"\nAutomotive impact analysis saved to automotive_impact_analysis.csv")

if __name__ == '__main__':
    main()