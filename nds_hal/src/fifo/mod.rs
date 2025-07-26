mod queue;

use queue::Queue;

static _QUEUE: Queue<256> = Queue::new();

pub fn send() {}
