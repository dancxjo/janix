#![no_std]

pub trait HardwareBridge {
fn log(&self, msg: &str);
fn ticks(&self) -> u64;
fn idle(&self);
fn shutdown(&self) -> !;
fn irq_disable(&self);
fn irq_enable(&self);
}
