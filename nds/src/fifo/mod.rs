mod queue;

use queue::Queue;

static _QUEUE: Queue<256> = Queue::new();

pub fn send(_command: Command) {}

#[repr(u8)]
#[non_exhaustive]
pub enum Command {
    IsDsi,
    Payload(u8),
}
