
use rand::Rng;
use std::thread;
use std::time::Duration;
use std::collections::BTreeMap as Map;

#[derive(Debug)]
#[derive(Clone)]
enum Expr {

	// State expressions
	State(String),
	Neighbor(i32, i32),

	// Boolean expressions
	Equals(Box<Expr>, Box<Expr>),      // x == y   x is y
	LessThan(Box<Expr>, Box<Expr>),    // x < y
	GreaterThan(Box<Expr>, Box<Expr>), // x > y
	And(Box<Expr>, Box<Expr>),         // x and y
	Or(Box<Expr>, Box<Expr>),          // x or y
	Not(Box<Expr>),                    // not x

	// Integer expressions
	Int(i32),
	Add(Box<Expr>, Box<Expr>),
	Sub(Box<Expr>, Box<Expr>),

	// Other
	IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
	Identifier(String),
	Let(String, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Value {
	State(String),
	Bool(bool),
	Int(i32),
}

const N: usize = 150;

type Environment = Map<String, Value>;

fn eval(env: &mut Environment, e: &Expr, cells: &[[String; N]; N], i: i32, j: i32) -> Value {
	match e {
		Expr::State(s) => Value::State(s.clone()),
		Expr::Int(n) => Value::Int(*n),
		Expr::Neighbor(di, dj) => {
			let ni = ((i + di).rem_euclid(N as i32)) as usize;
			let nj = ((j + dj).rem_euclid(N as i32)) as usize;
			Value::State(cells[ni][nj].clone().into())
		}
		Expr::Equals(e1, e2) => Value::Bool(eval(env, e1, cells, i, j) == eval(env, e2, cells, i, j)),
		Expr::LessThan(e1, e2) => {
			if let Value::Int(n1) = eval(env, e1, cells, i, j) {
				if let Value::Int(n2) = eval(env, e2, cells, i, j) {
					Value::Bool(n1 < n2)
				} else {
					Value::Bool(false)
				}
			} else {
				Value::Bool(false)
			}
		}
		Expr::GreaterThan(e1, e2) => {
			if let Value::Int(n1) = eval(env, e1, cells, i, j) {
				if let Value::Int(n2) = eval(env, e2, cells, i, j) {
					Value::Bool(n1 > n2)
				} else {
					Value::Bool(false)
				}
			} else {
				Value::Bool(false)
			}
		}
		Expr::And(e1, e2) => {
			if let Value::Bool(b1) = eval(env, e1, cells, i, j) {
				if let Value::Bool(b2) = eval(env, e2, cells, i, j) {
					Value::Bool(b1 && b2)
				} else {
					Value::Bool(false)
				}
			} else {
				Value::Bool(false)
			}
		}
		Expr::Or(e1, e2) => {
			if let Value::Bool(b1) = eval(env, e1, cells, i, j) {
				if let Value::Bool(b2) = eval(env, e2, cells, i, j) {
					Value::Bool(b1 || b2)
				} else {
					Value::Bool(false)
				}
			} else {
				Value::Bool(false)
			}
		}
		Expr::Not(e) => {
			if let Value::Bool(b) = eval(env, e, cells, i, j) {
				Value::Bool(!b)
			} else {
				Value::Bool(false)
			}
		}
		Expr::IfThenElse(c, t, f) => {
			if eval(env, c, cells, i, j) == Value::Bool(true) {
				eval(env, t, cells, i, j)
			} else {
				eval(env, f, cells, i, j)
			}
		}
		Expr::Add(e1, e2) => {
			let v1 = eval(env, e1, cells, i, j);
			let v2 = eval(env, e2, cells, i, j);
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
		Expr::Sub(e1, e2) => {
			let v1 = eval(env, e1, cells, i, j);
			let v2 = eval(env, e2, cells, i, j);
			if let Value::Int(n1) = v1 {
				if let Value::Int(n2) = v2 {
					Value::Int(n1 - n2)
				} else {
					Value::Int(0)
				}
			} else {
				Value::Int(0)
			}
		}
		Expr::Identifier(s) => {
			if let Some(v) = env.get(s) {
				(*v).clone()
			} else {
				Value::Int(0)
			}
		}
		Expr::Let(s, edef, ebody) => {
			// Rust won't let you inline this variable!
			// Because of "overlapping mutable borrows"
			// If they are in separate statements, it works!
			let v = eval(env, edef, cells, i, j);
			env.insert(s.to_string(), v);
			eval(env, ebody, cells, i, j)
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
			let mut environment: Environment = Map::new();
			if let Value::State(s) = eval(&mut environment, e, cells, i as i32, j as i32) {
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
	// let n = (if [1, -1] == Alive then 1 else 0) +
	// 	(if [1,  0] == Alive then 1 else 0) +
	// 	(if [1,  1] == Alive then 1 else 0) +
	// 	(if [0, -1] == Alive then 1 else 0) +
	// 	(if [0,  1] == Alive then 1 else 0) +
	// 	(if [1, -1] == Alive then 1 else 0) +
	// 	(if [1,  0] == Alive then 1 else 0) +
	// 	(if [1,  1] == Alive then 1 else 0)
	// in
	// 	if [0, 0] == Alive and (n == 2 or n == 3) then Alive
	// 	else if [0, 0] == Dead and n == 3 then Alive
	// 	else Dead

	let expr: Expr = Expr::Let(
		"n".to_string(),
		Box::new(Expr::Add(
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
			Box::new(Expr::IfThenElse(Box::new(Expr::Equals(Box::new(Expr::State("Alive".to_string())), Box::new(Expr::Neighbor( 1,  1)))), Box::new(Expr::Int(1)), Box::new(Expr::Int(0)))))),
		Box::new(Expr::IfThenElse(
			Box::new(Expr::And(
				Box::new(Expr::Equals(Box::new(Expr::Neighbor(0, 0)), Box::new(Expr::State("Alive".to_string())))),
				Box::new(Expr::Or(
					Box::new(Expr::Equals(Box::new(Expr::Identifier("n".to_string())), Box::new(Expr::Int(2)))),
					Box::new(Expr::Equals(Box::new(Expr::Identifier("n".to_string())), Box::new(Expr::Int(3))))
				))
			)),
			Box::new(Expr::State("Alive".to_string())),
			Box::new(Expr::IfThenElse(
				Box::new(Expr::And(
					Box::new(Expr::Equals(Box::new(Expr::Neighbor(0, 0)), Box::new(Expr::State("Dead".to_string())))),
					Box::new(Expr::Equals(Box::new(Expr::Identifier("n".to_string())), Box::new(Expr::Int(3))))
				)),
				Box::new(Expr::State("Alive".to_string())),
				Box::new(Expr::State("Dead".to_string()))
			))
		))
	);

	// print!("{:?}\n", expr);

	print_grid(&cells);

	loop {
		cells = apply(&expr, &cells);

		print!("\x1B[2J\x1B[1;1H");
		print_grid(&cells);

		thread::sleep(Duration::from_millis(1));
	}
}
