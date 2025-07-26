#[unsafe(no_mangle)]
pub extern "C" fn send() {
    nds::fifo::send(todo!());
}
