//! Wasm guest lifecycle management for BULB boot animations.
//!
//! This module provides the runtime for executing Wasm-based boot animation
//! modules with fuel-based preemption. Guest modules receive boot state updates
//! and emit draw commands via the command buffer interface.

#[cfg(feature = "wasm")]
use wasmi::{Engine, Linker, Module, Store, Instance, Func, Caller, TypedFunc};

use crate::boot_state::BootStateWire;
use crate::draw_commands::{CommandBuffer, MAX_COMMAND_BUFFER_SIZE};

/// Default fuel budget per callback (50,000 instructions)
pub const DEFAULT_FUEL_BUDGET: u64 = 50_000;

/// Maximum Wasm linear memory (256 KiB for boot art)
pub const MAX_WASM_MEMORY: usize = 256 * 1024;

/// Result of a bulb module execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulbResult {
    /// Module executed successfully
    Ok,
    /// Module ran out of fuel (preempted)
    OutOfFuel,
    /// Module trapped (error)
    Trapped,
    /// Module not loaded
    NotLoaded,
}

/// Host state passed to Wasm callbacks
pub struct BulbHostState {
    /// Current boot state
    pub boot_state: BootStateWire,
    /// Command buffer for draw commands
    pub commands: CommandBuffer,
    /// Display width
    pub display_width: u16,
    /// Display height
    pub display_height: u16,
}

impl Default for BulbHostState {
    fn default() -> Self {
        Self {
            boot_state: BootStateWire::default(),
            commands: CommandBuffer::new(),
            display_width: 0,
            display_height: 0,
        }
    }
}

/// BULB module wrapper for Wasm-based boot animations
#[cfg(feature = "wasm")]
pub struct BulbModule {
    engine: Engine,
    store: Store<BulbHostState>,
    instance: Option<Instance>,
    // Cached function handles
    fn_init: Option<TypedFunc<u64, u32>>,
    fn_on_boot: Option<TypedFunc<(u32, u32), ()>>,
    fn_on_tick: Option<TypedFunc<u64, ()>>,
    fn_render: Option<TypedFunc<(), ()>>,
}

#[cfg(feature = "wasm")]
impl BulbModule {
    /// Create a new BULB module from Wasm bytes
    pub fn new(wasm_bytes: &[u8]) -> Result<Self, BulbError> {
        let mut config = wasmi::Config::default();
        config.consume_fuel(true);
        
        let engine = Engine::new(&config);
        let module = Module::new(&engine, wasm_bytes)
            .map_err(|_| BulbError::InvalidModule)?;
        
        let mut store = Store::new(&engine, BulbHostState::default());
        store.set_fuel(DEFAULT_FUEL_BUDGET).ok();
        
        let mut linker = Linker::new(&engine);
        
        // Register host functions
        Self::register_host_functions(&mut linker)?;
        
        let instance = linker
            .instantiate(&mut store, &module)
            .map_err(|_| BulbError::InstantiationFailed)?
            .start(&mut store)
            .map_err(|_| BulbError::StartFailed)?;
        
        // Look up exported functions
        let fn_init = instance
            .get_typed_func::<u64, u32>(&store, "bulb_init")
            .ok();
        let fn_on_boot = instance
            .get_typed_func::<(u32, u32), ()>(&store, "bulb_on_boot")
            .ok();
        let fn_on_tick = instance
            .get_typed_func::<u64, ()>(&store, "bulb_on_tick")
            .ok();
        let fn_render = instance
            .get_typed_func::<(), ()>(&store, "bulb_render")
            .ok();
        
        Ok(Self {
            engine,
            store,
            instance: Some(instance),
            fn_init,
            fn_on_boot,
            fn_on_tick,
            fn_render,
        })
    }
    
    fn register_host_functions(linker: &mut Linker<BulbHostState>) -> Result<(), BulbError> {
        // cmd_submit(ptr: u32, len: u32) - submit command buffer
        linker
            .func_wrap("env", "cmd_submit", |_caller: Caller<'_, BulbHostState>, _ptr: u32, _len: u32| {
                // Commands are already in linear memory; host will read them
            })
            .map_err(|_| BulbError::LinkFailed)?;
        
        // host_get_boot_state(ptr: u32, len: u32) -> u32
        linker
            .func_wrap("env", "host_get_boot_state", |caller: Caller<'_, BulbHostState>, ptr: u32, len: u32| -> u32 {
                let bytes = caller.data().boot_state.to_bytes();
                let copy_len = (len as usize).min(bytes.len());
                // Would write to Wasm memory at ptr
                copy_len as u32
            })
            .map_err(|_| BulbError::LinkFailed)?;
        
        // host_get_display_info(ptr: u32) -> u32
        linker
            .func_wrap("env", "host_get_display_info", |caller: Caller<'_, BulbHostState>, _ptr: u32| -> u32 {
                let w = caller.data().display_width as u32;
                let h = caller.data().display_height as u32;
                (w << 16) | h
            })
            .map_err(|_| BulbError::LinkFailed)?;
        
        Ok(())
    }
    
    /// Initialize the module with a seed value
    pub fn init(&mut self, seed: u64) -> BulbResult {
        let Some(ref fn_init) = self.fn_init else {
            return BulbResult::NotLoaded;
        };
        
        self.store.set_fuel(DEFAULT_FUEL_BUDGET).ok();
        
        match fn_init.call(&mut self.store, seed) {
            Ok(_) => BulbResult::Ok,
            Err(e) if e.to_string().contains("fuel") => BulbResult::OutOfFuel,
            Err(_) => BulbResult::Trapped,
        }
    }
    
    /// Notify module of boot state change
    pub fn on_boot(&mut self, state: BootStateWire) -> BulbResult {
        self.store.data_mut().boot_state = state;
        
        let Some(ref fn_on_boot) = self.fn_on_boot else {
            return BulbResult::NotLoaded;
        };
        
        self.store.set_fuel(DEFAULT_FUEL_BUDGET).ok();
        
        // Pass pointer to boot state in Wasm memory (would need memory write)
        match fn_on_boot.call(&mut self.store, (0, 8)) {
            Ok(_) => BulbResult::Ok,
            Err(e) if e.to_string().contains("fuel") => BulbResult::OutOfFuel,
            Err(_) => BulbResult::Trapped,
        }
    }
    
    /// Tick the animation
    pub fn on_tick(&mut self, now_ms: u64) -> BulbResult {
        let Some(ref fn_on_tick) = self.fn_on_tick else {
            return BulbResult::Ok; // Optional function
        };
        
        self.store.set_fuel(DEFAULT_FUEL_BUDGET).ok();
        
        match fn_on_tick.call(&mut self.store, now_ms) {
            Ok(_) => BulbResult::Ok,
            Err(e) if e.to_string().contains("fuel") => BulbResult::OutOfFuel,
            Err(_) => BulbResult::Trapped,
        }
    }
    
    /// Render the current frame
    pub fn render(&mut self) -> BulbResult {
        let Some(ref fn_render) = self.fn_render else {
            return BulbResult::NotLoaded;
        };
        
        self.store.data_mut().commands.clear();
        self.store.set_fuel(DEFAULT_FUEL_BUDGET).ok();
        
        match fn_render.call(&mut self.store, ()) {
            Ok(_) => BulbResult::Ok,
            Err(e) if e.to_string().contains("fuel") => BulbResult::OutOfFuel,
            Err(_) => BulbResult::Trapped,
        }
    }
    
    /// Get the command buffer after rendering
    pub fn commands(&self) -> &[u8] {
        self.store.data().commands.as_bytes()
    }
}

/// Errors during BULB module operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulbError {
    /// Invalid Wasm module
    InvalidModule,
    /// Failed to instantiate module
    InstantiationFailed,
    /// Failed to start module
    StartFailed,
    /// Failed to link host functions
    LinkFailed,
}
