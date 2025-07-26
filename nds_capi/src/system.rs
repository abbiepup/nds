/// Shuts the system down.
#[unsafe(no_mangle)]
pub extern "C" fn shutdown() -> ! {
    nds::system::shutdown();
}

#[unsafe(no_mangle)]
pub extern "C" fn is_dsi() -> bool {
    nds::system::is_dsi()
}
