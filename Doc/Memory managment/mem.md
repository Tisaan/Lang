# Region-Based Memory Management

## Core Idea
- **No explicit `region` statements**: Regions are **implicitly tied to lexical blocks and functions.**
- **Memory is automatically freed** when the block/function ends.
- **Optional `free_in` parameter** for extending lifetimes beyond the current block.

## Syntax and Rules

- **Implicit Regions:**
  - Every `{}` block or function defines a region.
  - Memory allocated in a block is freed when the block ends.
	```rust
	fn example() {
    	let data = alloc(100); // Allocated in this block's region
    	{
        	let temp = alloc(50); // Allocated in this nested block's region
    	} // temp is freed here
	} // data is freed here
	```
- **Extending Lifetimes:**
  - Use `free_in: outer` to move memory to the enclosing region.
	```rust
	fn example() {
    	let long_lived = alloc(200, free_in: outer); // Allocated in the enclosing region
	}
	```
- **Cross-Function Regions:**
  - Functions define regions. Allocations default to the function’s region.
	```rust
	fn create_buffer() -> *mut u8 {
    	alloc(100, free_in: outer) // Allocated in the caller's region
	}
	```
- **Global/Static Regions:**
	```rust
	let global_data = alloc(100, free_in: 'static);
	```
- **Safety:**
  - **No use-after-free**: The compiler ensures references don’t outlive their region.
  - **No double-free**: Memory is tied to exactly one region.
  - **No manual management**: Regions are inferred and managed automatically.


## Exception

- **Manual Managenet:**
  - **Manual memory management** could still be use inside a block annoted with the `Mem-Manual` attribute
  - Useful for Performance-critical, interfacing with FFI/ABI, Debugging...
  ```rust
	use memory::allocator;
	
	#[Mem-Manual]
	fn manual() {
		let ptr = alloc(100);
		//Do Something
		free(ptr);
		//ptr is now free
	}
  ```
## Limitation
  - Use implicit region for <64KB
  - Use Arena for <1MB
  - Custom allocator and free >= 1MB