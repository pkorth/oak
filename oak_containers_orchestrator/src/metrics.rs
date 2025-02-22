//
// Copyright 2023 The Project Oak Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use anyhow::Result;
use oak_containers_orchestrator_client::LauncherClient;
use opentelemetry_api::{
    metrics::{AsyncInstrument, Meter, MeterProvider, ObservableCounter, ObservableGauge, Unit},
    KeyValue,
};
use std::{sync::Arc, time::Duration};

// It's not dead, it's just asynchronous.
#[allow(dead_code)]
pub struct SystemMetrics {
    cpu_seconds_total: ObservableCounter<u64>,
    context_switches_total: ObservableCounter<u64>,
    forks_total: ObservableCounter<u64>,
    procs_blocked: ObservableGauge<u64>,
    procs_running: ObservableGauge<u64>,
}

impl SystemMetrics {
    fn new(meter: Meter) -> Result<Self> {
        Ok(Self {
            cpu_seconds_total: meter
                .u64_observable_counter("cpu_seconds_total")
                .with_unit(Unit::new("seconds"))
                .with_callback(Self::cpu_seconds_total)
                .try_init()?,
            context_switches_total: meter
                .u64_observable_counter("context_switches_total")
                .with_callback(Self::context_switches_total)
                .try_init()?,
            forks_total: meter
                .u64_observable_counter("forks_total")
                .with_callback(Self::forks_total)
                .try_init()?,
            procs_blocked: meter
                .u64_observable_gauge("procs_blocked")
                .with_callback(Self::procs_blocked)
                .try_init()?,
            procs_running: meter
                .u64_observable_gauge("procs_running")
                .with_callback(Self::procs_running)
                .try_init()?,
        })
    }

    fn cpu_seconds_total(counter: &dyn AsyncInstrument<u64>) {
        let stats = procfs::KernelStats::new().unwrap();

        for (cpu, stats) in stats.cpu_time.iter().enumerate() {
            let attributes = |mode| {
                [
                    KeyValue::new("cpu", cpu.to_string()),
                    KeyValue::new("mode", mode),
                ]
            };

            counter.observe(stats.user, &attributes("user"));
            counter.observe(stats.idle, &attributes("idle"));
            counter.observe(stats.nice, &attributes("nice"));
            counter.observe(stats.system, &attributes("system"));
            if let Some(iowait) = stats.iowait {
                counter.observe(iowait, &attributes("iowait"));
            }
            if let Some(irq) = stats.irq {
                counter.observe(irq, &attributes("irq"));
            }
            if let Some(softirq) = stats.softirq {
                counter.observe(softirq, &attributes("softirq"));
            }
            if let Some(steal) = stats.steal {
                counter.observe(steal, &attributes("steal"));
            }
        }
    }

    fn context_switches_total(counter: &dyn AsyncInstrument<u64>) {
        let stats = procfs::KernelStats::new().unwrap();
        counter.observe(stats.ctxt, &[]);
    }

    fn forks_total(counter: &dyn AsyncInstrument<u64>) {
        let stats = procfs::KernelStats::new().unwrap();
        counter.observe(stats.processes, &[]);
    }

    fn procs_blocked(gauge: &dyn AsyncInstrument<u64>) {
        let stats = procfs::KernelStats::new().unwrap();
        if let Some(procs_blocked) = stats.procs_blocked {
            gauge.observe(procs_blocked.into(), &[]);
        }
    }

    fn procs_running(gauge: &dyn AsyncInstrument<u64>) {
        let stats = procfs::KernelStats::new().unwrap();
        if let Some(procs_running) = stats.procs_running {
            gauge.observe(procs_running.into(), &[]);
        }
    }
}

pub fn run(launcher_client: Arc<LauncherClient>) -> Result<SystemMetrics> {
    let metrics = opentelemetry_otlp::new_pipeline()
        .metrics(opentelemetry_sdk::runtime::Tokio)
        .with_exporter(launcher_client.openmetrics_builder())
        .with_period(Duration::from_secs(60))
        .build()?;
    SystemMetrics::new(metrics.meter("oak_containers_orchestrator"))
}
