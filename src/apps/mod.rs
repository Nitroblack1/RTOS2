//! Applications Module
//!
//! This module contains all user applications in a modular structure.
//! Apps are automatically registered via the `#[app]` attribute macro.

// BENCHMARK MODE: Only essential apps for clean MPU comparison
// Disable other apps to get clean measurements
// pub mod counter; // Disabled for clean benchmark
// pub mod data_logger;
// pub mod fibonacci;
// pub mod gpio_monitor;
// pub mod led_blinker;
// pub mod math_calculator;
// pub mod network_stack;
// pub mod power_manager; // 🚀 10번째 앱 - 진짜 링크 타임 디스커버리 검증!
// pub mod sensor_reader; // NEW APP ADDED AUTOMATICALLY!
// pub mod timer;
// pub mod watchdog; // 링크 타임 디스커버리로 자동 발견! // 🚀 11번째 앱 - 최종 자동 등록 검증!

// Disable IPC apps for cleaner benchmark
// pub mod producer; // Disabled for benchmark focus
// pub mod consumer; // Disabled for benchmark focus
// pub mod shared_counter; // Race condition and synchronization test

// Phase 2: Memory protection testing
// pub mod memory_violator; // Memory violation testing app - safer version

// Performance benchmarking apps
pub mod benchmark_context_switch; // Context switch performance with/without MPU
// pub mod benchmark_ipc_latency; // IPC message latency and throughput measurement - temporarily disabled

// No manual registration arrays needed - apps automatically discovered!
