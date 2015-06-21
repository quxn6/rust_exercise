extern crate rand;

fn quick_sort(arr : &mut [i32], left : usize, right : usize) {
	let index : usize = partition(arr, left, right);
	println!("{:?} {:?} {:?}", left, right, index);
	println!("{:?}", arr);
	if left < index -1 {
		quick_sort(arr, left, index - 1);
	}

	if right > index {
		quick_sort(arr, index , right );	
	}
}

fn partition(arr : &mut [i32], mut left : usize, mut right : usize) -> usize {
	let pivot_index : usize = rand::random::<usize>() % (right-left) + left;
	// println!("pivot defined {:?} {:?} {:?}", pivot_index, left, right);
	let pivot : i32 = arr[pivot_index];
	// println!("pivot check" );
	// let pivot : i32 = arr[rand::random::<usize>() % (right-left) + left];
	// let pivot : i32 = arr[(left + right) /2];
	while left <= right {
		while arr[left] < pivot{
			left += 1;
		}
		
		while pivot < arr[right] {
			right -= 1;
		}
		
		if left <= right {
			swap(arr, left, right);
			
			
			if left < std::u32::MAX as usize 
			{				
				left += 1;
			}

			if right > 0 {
				right -= 1;
			}
		}
	}

	left
}

fn swap(arr : &mut [i32], index1 : usize, index2 : usize)
{
	let tmp : i32 = arr[index1];
	arr[index1] = arr[index2];
	arr[index2] = tmp;
}

fn main() {
    let mut array1 : [i32; 10] = [3,2,1,-10,30,64,21,102,201,32];
 //   size : usize = rand::random::<usize>();
 //   let mut rand_array : [i32; size];
  	// let tmp : usize = 0;
  	// println!("{:?}", array1[tmp]); 
    println!("{:?}", array1);
    quick_sort(&mut array1, 0, 9);
    println!("{:?}", array1);
}
