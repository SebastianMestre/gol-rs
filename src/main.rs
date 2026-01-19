
use rand::Rng;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
#[derive(Clone)]
enum Expr {

	// State expressions
	State(String),
	Neighbor(i32, i32),

	// Boolean expressions
	Equals(Box<Expr>, Box<Expr>),        // x == y   x is y
	// LessThan(Expr, Expr),      // x < y
	// GreaterThan(Expr, Expr),   // x > y
	// And(Expr, Expr),           // x and y
	// Or(Expr, Expr),            // x or y
	// Not(Expr),                 // not x

	// Integer expressions
	Int(i32),
	Add(Box<Expr>, Box<Expr>),
	// Sub(Expr, Expr),

	// Other
	IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
	// Let(String, Expr, Expr),
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Value {
	State(String),
	Bool(bool),
	Int(i32),
}

const N: usize = 150;

fn eval(e: &Expr, cells: &[[String; N]; N], i: i32, j: i32) -> Value {
	match e {
		Expr::State(s) => Value::State(s.clone()),
		Expr::Int(n) => Value::Int(*n),
		Expr::Neighbor(di, dj) => {
			let ni = ((i + di).rem_euclid(N as i32)) as usize;
			let nj = ((j + dj).rem_euclid(N as i32)) as usize;
			Value::State(cells[ni][nj].clone().into())
		}
		Expr::Equals(e1, e2) => Value::Bool(eval(e1, cells, i, j) == eval(e2, cells, i, j)),
		Expr::IfThenElse(c, t, f) => {
			if eval(c, cells, i, j) == Value::Bool(true) {
				eval(t, cells, i, j)
			} else {
				eval(f, cells, i, j)
			}
		}
		Expr::Add(e1, e2) => {
			let v1 = eval(e1, cells, i, j);
			let v2 = eval(e2, cells, i, j);
			if let Value::Int(n1) = v1 {
				if let Value::Int(n2) = v2 {
					Value::Int(n1 + n2)
				} else {
					Value::Int(0)
				}
			} else {
				Value::Int(0)
			}
		}
	}
}

fn print_grid(cells: &[[String; N]; N]) {
	for i in 0..N {
		for j in 0..N {
			if cells[i][j] == "Alive" {
				print!("[]");
			} else {
				print!("  ");
			}
		}
		print!("\n");
	}
}

fn apply(e: &Expr, cells: &[[String; N]; N]) -> [[String; N]; N] {
	let mut result: [[String; N]; N] =
		core::array::from_fn(|_i|
			core::array::from_fn(|_j|
				"".to_string()));

	for i in 0..N {
		for j in 0..N {
			if let Value::State(s) = eval(e, cells, i as i32, j as i32) {
				result[i as usize][j as usize] = s;
			}
		}
	}
	result
}

fn main() {

	let mut rng = rand::rng();

	let mut cells: [[String; N]; N] =
		core::array::from_fn(|_i|
			core::array::from_fn(|_j|
				"Dead".to_string()));
	for i in 0..N {
		for j in 0..N {
			if rng.random_bool(0.5) {
				cells[i][j] = "Alive".to_string();
			} else {
				cells[i][j] = "Dead".to_string();
			}
		}
	}


	// Hard-code typical Game of Life rules:
	let neighbor_count =
		Expr::Add(
		Box::new(Expr::Add(
		Box::new(Expr::Add(
		Box::new(Expr::Add(
		Box::new(Expr::Add(
		Box::new(Expr::Add(
		Box::new(Expr::Add(
			Box::new(Expr::IfThenElse(Box::new(Expr::Equals(Box::new(Expr::State("Alive".to_string())), Box::new(Expr::Neighbor(-1, -1)))), Box::new(Expr::Int(1)), Box::new(Expr::Int(0)))),
			Box::new(Expr::IfThenElse(Box::new(Expr::Equals(Box::new(Expr::State("Alive".to_string())), Box::new(Expr::Neighbor(-1,  0)))), Box::new(Expr::Int(1)), Box::new(Expr::Int(0)))))),
			Box::new(Expr::IfThenElse(Box::new(Expr::Equals(Box::new(Expr::State("Alive".to_string())), Box::new(Expr::Neighbor(-1,  1)))), Box::new(Expr::Int(1)), Box::new(Expr::Int(0)))))),
			Box::new(Expr::IfThenElse(Box::new(Expr::Equals(Box::new(Expr::State("Alive".to_string())), Box::new(Expr::Neighbor( 0, -1)))), Box::new(Expr::Int(1)), Box::new(Expr::Int(0)))))),
			Box::new(Expr::IfThenElse(Box::new(Expr::Equals(Box::new(Expr::State("Alive".to_string())), Box::new(Expr::Neighbor( 0,  1)))), Box::new(Expr::Int(1)), Box::new(Expr::Int(0)))))),
			Box::new(Expr::IfThenElse(Box::new(Expr::Equals(Box::new(Expr::State("Alive".to_string())), Box::new(Expr::Neighbor( 1, -1)))), Box::new(Expr::Int(1)), Box::new(Expr::Int(0)))))),
			Box::new(Expr::IfThenElse(Box::new(Expr::Equals(Box::new(Expr::State("Alive".to_string())), Box::new(Expr::Neighbor( 1,  0)))), Box::new(Expr::Int(1)), Box::new(Expr::Int(0)))))),
			Box::new(Expr::IfThenElse(Box::new(Expr::Equals(Box::new(Expr::State("Alive".to_string())), Box::new(Expr::Neighbor( 1,  1)))), Box::new(Expr::Int(1)), Box::new(Expr::Int(0)))));

	let me = Expr::Neighbor(0, 0);

	let expr: Expr = Expr::IfThenElse(
		Box::new(Expr::Equals(Box::new(me.clone()), Box::new(Expr::State("Alive".to_string())))),
			Box::new(Expr::IfThenElse(Box::new(Expr::Equals(Box::new(neighbor_count.clone()), Box::new(Expr::Int(2)))), Box::new(Expr::State("Alive".to_string())),
			Box::new(Expr::IfThenElse(Box::new(Expr::Equals(Box::new(neighbor_count.clone()), Box::new(Expr::Int(3)))), Box::new(Expr::State("Alive".to_string())),
			Box::new(Expr::State("Dead".to_string())))))),
			Box::new(Expr::IfThenElse(Box::new(Expr::Equals(Box::new(neighbor_count.clone()), Box::new(Expr::Int(3)))), Box::new(Expr::State("Alive".to_string())),
			Box::new(Expr::State("Dead".to_string())))));

	// print!("{:?}\n", expr);

	print_grid(&cells);

	loop {
		cells = apply(&expr, &cells);

		print!("\x1B[2J\x1B[1;1H");
		print_grid(&cells);

		thread::sleep(Duration::from_millis(1));
	}
}
