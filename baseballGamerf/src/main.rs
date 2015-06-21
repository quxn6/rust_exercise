use std::io;
use std::io::Write;

extern crate rand;

struct BaseballGame
{
	pub round_count : i32,
	pub strike_count : i32,
	pub ball_count : i32,
	pub is_correct_answer : bool,
	pub numbers : [u32; 3]

}

impl BaseballGame 
{
	fn init_game(&mut self)
	{
		println!("initialize BaseballGame");
		self.round_count = 0;
		self.strike_count = 0;
		self.ball_count = 0;
		self.is_correct_answer = false;
		self.generate_random_numbers();
	}	

	fn generate_random_numbers(&mut self)
	{
		for i in 0..self.numbers.len()
		{
			'get_random : loop 
			{
				self.numbers[i] = rand::random::<u32>() % 9 + 1;	
				for j in 0..i
				{
					if ( i == j ) { break 'get_random; }
					if ( self.numbers[j] == self.numbers[i] ) { break;}
				}
				break;
			}
			
			println!("{:?}", self.numbers[i]);
		}
		

	}

	fn play_game(&mut self)
	{
		println!("Play Baseball Game");
		while self.is_correct_answer == false
		{
			let mut answer : u32 = 0;
			self.get_input(&mut answer);
			self.check_answer(answer);
			self.check_win();
		}
	}

	fn get_input(&mut self, answer : &mut u32)
	{
		let mut input : String = String::new();
		io::stdin().read_line(&mut input);

		let answer_converted = u32::from_str_radix(input.trim(), 10);
		
		match answer_converted 
		{
			Ok(num) => { *answer = num },
			Err(_) => { println!("invalid number format"); },
		};

		println!("{:?}", answer);
	}

	fn check_answer(&mut self, answer : u32)
	{
		let mut answers : [u32;3] = [0;3];

		answers[0] = answer / 100;
		answers[1] = ( answer % 100 ) / 10;
		answers[2] = ( answer % 10 );

		println!("{:?}", answers);

		for i in 0..self.numbers.len()
		{
			if self.numbers[i] == answers[i]
			{
				self.strike_count += 1;
			}
			else
			{
				for j in 0..self.numbers.len()
				{
					if (self.numbers[j] == answers[i])
					{
						self.ball_count += 1;
					}
				}
			}
		}

		println!("{:?} strikes, {:?} balls", self.strike_count, self.ball_count);
	}

	fn check_win(&mut self)
	{
		self.round_count += 1;
		if ( self.strike_count == 3)
		{
			self.is_correct_answer = true;
		}
		self.strike_count = 0;
		self.ball_count = 0;
	}
}

fn main()
{
	let mut c = BaseballGame
	{ 
		round_count : 0,
		strike_count : 0,
		ball_count : 0,
		is_correct_answer : false,
		numbers : [0;3]
	};

	c.init_game();
	c.play_game();
	println!("you win ! you have tried {:?}", c.round_count);
}
