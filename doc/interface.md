
The ringbuffer crate provides four traits. In this document their methods are described.

# DEFINITIONS

In this document, `front` refers to the item last pushed to the queue. `back` refers to the 
item pushed longest ago and thus the item that will be dequeud next. 

Relative indexing (like with get and get_mut) works from the `back` and counts towards the front.
`get(0)` == `back` and `get(1)` will be dequeued right after `get(0)` 

# RingBuffer

This trait must be implemented for any ringbuffer. It's the "base" trait.
With it you can't actually do anything really useful, but it provides some 
basic methods.

| function | description |
| --- | ---| 
| `len()` | number of items in the ringbuffer |
| `is_empty()` | returns true if the length is zero |
| `is_full()` | returns true if the length equals the capacity |
| `clear` | sets the length back to zero and removes all items from the ringbuffer |
| `capacity` | the maximum number of items in the ringbuffer |
    

# ReadableRingbuffer

With this trait it's possible to use the ringbuffer as the read end of a queue.

| function | description |
| --- | ---| 
| `skip()` | Like dequeue but drops the items it removes from the ringbuffer |
| `dequeue()` | Removes the item pushed longest ago (fifo) from the ringbuffer and returns it. |
 
# WritableRingbuffer

With this trait it's possible to use the ringbuffer as the write end of a queue.

| function | description |
| --- | ---| 
| `push(item)` | Adds an item to the ringbuffer. Overwrites the item pushed longest ago from the ringbuffer when it was full. |
 
# RingBufferExt

Provides general purpose methods to modify the ringbuffer as if it was an array. 
This trait is not implemented for Ringbuffers which can purely be used as a queue.

| function | description |
| --- | ---| 
| `contains(item)` | returns true if item is in the buffer |
| `front()` | Returns a reference to the item pushed most recently |
| `back()` | Returns a reference to the item which would be dequeued next |
| `back_mut()` | Like back but mutable |
| `front_mut()` | Like front but mutable |
| `iter()` | Returns an immutable iterator over the elements in the ringbuffer |
| `iter_mut()` | Like iter but mutable. Not actually an iterator due to lifetime constraints. |
| `to_vec()` | Converts the Ringbuffer to a vec. |
| `get(n)` | Returns a reference to the nth item from the readhead. This means that get(0) returns what will be dequeued next, and get(1) will be dequeued after that. |
| `get_absolute(n)` | Returns a reference to nth item relative to the start of the underlying non-circular buffer. |
| `get_mut(n)` | Like get but mutable |
| `get_absolute(n)` | Like get_absolute but mutable |



 