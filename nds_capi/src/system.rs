/// Shuts the system down.
#[unsafe(no_mangle)]
pub extern "C" fn shutdown() -> ! {
    nds_hal::system::shutdown();
}
