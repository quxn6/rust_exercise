struct PriorityQueue {
	pub heap : Vec<i32>
}

impl PriorityQueue {
	fn push(&mut self , new_value : i32)
	{
		let length = self.heap.len();
		self.heap.push(new_value);
		self._bubble_up( length );
	}

	pub fn _bubble_up(&mut self, index : usize)
	{
		if index == 0 {
			return ;
		}

		let parent_index : usize = ( index -1 ) / 2;

		if self.heap[parent_index] > self.heap[index] {
			self.heap.swap(parent_index, index);// ._swap(parent_index, index);
			self._bubble_up(parent_index);
		}
	}

	pub fn _swap(&mut self, index1 : usize, index2 : usize)
	{
		let tmp : i32 = self.heap[index1];
		self.heap[index1] = self.heap[index2];
		self.heap[index2] = tmp;
	}

	fn peek(& self) -> i32
	{
		if self.heap.len() <= 0 {
			panic!();
		}

		self.heap[0]		
	}

	fn pop(&mut self) -> Result<i32, &str> {
		let return_val = self.peek();

		self.heap[0] = match self.heap.pop() {
			Some(number) => number,
			None => {panic!();}
		};
		self._bubble_down(0);

		Ok(return_val)
	}

	fn _bubble_down(&mut self, index : usize)
	{
		let length : usize = self.heap.len();
		let left_child_index : usize = 2 * index + 1;
		let right_child_index : usize = 2 * index + 2;

		if left_child_index >= length {
			return ;
		}

		let mut min_index = index;

		if self.heap[index] > self.heap[left_child_index] {
			min_index = left_child_index;
		}

		if right_child_index < length && self.heap[right_child_index] < self.heap[min_index] {
			min_index = right_child_index;
		}

		if min_index != index {
			self._swap(min_index, index);
			self._bubble_down(min_index);
		}
	} 

}

fn main() {
	println!("priority heap");

	let mut pq = PriorityQueue {
		heap : Vec::new()
	};

	pq.push(32);
	pq.push(23);
	pq.push(11);
	pq.push(-4);
	pq.push(1100);

	pq.heap.push(1);

	println!("{:?}", pq.peek());
	println!("{:?}", pq.pop());
	println!("{:?}", pq.pop());
}
