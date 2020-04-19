// 空白文字
const EMPTY_STR: &str = ".";
// 先手文字
const FIRST_STR: &str = "#";
// 後手文字
const SECOND_STR: &str = "*";
// サイズ
const SIZE: i32 = 8;
// ターン
#[derive(Debug)]
#[derive(PartialEq)]
enum Player {
	FIRST,
	SECOND
}
// orientは向きを示す。具体的には以下の通り。
// 1 2 3
// 4 * 6
// 7 8 9
fn check_straight_line_same_piece_exist(map: &Vec<Vec<&str>>,
										player: &Player,
										x: usize,
										y: usize,
										ptr: (i32, i32)) -> bool {
	let (x_, y_) = ptr;
	let player_str = match player {
		Player::FIRST => FIRST_STR,
		Player::SECOND => SECOND_STR,
	};
	if (x as i32+x_) as usize >= (SIZE as usize) ||
		(y as i32+y_) as usize >= (SIZE as usize) {
			return false
	}

	if map[(x as i32+x_) as usize][(y as i32+y_) as usize] == player_str {
		return true
	}else if map[(x as i32+x_) as usize][(y as i32+y_) as usize] == EMPTY_STR {
		return false
	}else {
		return check_straight_line_same_piece_exist(map,
													player,
													(x as i32+x_) as usize,
													(y as i32+y_) as usize,
													ptr)
	}
}
fn pickup_points(map: &Vec<Vec<&str>>, player: &Player) -> Vec<(usize, usize)> {
	let another_player_str = match player {
		Player::FIRST => SECOND_STR,
		Player::SECOND => FIRST_STR,
	};
	let mut result = Vec::new();
	for x in 0..map.len() {
		for y in 0..map[0].len() {
			if map[x][y] == EMPTY_STR {
				for i in -1i32..2 {
					for j in -1i32..2 {
						if (x as i32 + i) >= 0 && (x as i32 + i) < SIZE &&
							(y as i32 + j) >= 0 &&
							(y as i32 + j) < SIZE &&
							map[(x as i32+i) as usize][(y as i32+j) as usize] == another_player_str {
								// 該当方向の先にplayerと同じピースが存在するか。
								let i_ = ((x as i32)+i) as usize;
								let j_ = ((y as i32)+j) as usize;
								if i_ < (SIZE as usize) && j_ < (SIZE as usize) &&
									check_straight_line_same_piece_exist(&map, player, i_, j_, (i, j)) {
										result.push((x, y));
									}
							}
					}
				}
			}
		}
	}
	return result
}
fn is_skip(map: &Vec<Vec<&str>>, player: &Player) -> bool {
	return pickup_points(map, player).len() <= 0
}
fn check_put_piece(map: &Vec<Vec<&str>>, x: usize, y: usize, player: &Player) -> bool {
	// 打てる手を全て算出。
	let ptrs = pickup_points(&map, player);
	// 算出した手以外にピースを置いた場合、false。
	for ptr in ptrs {
		if ptr.0 == x && ptr.1 == y {
			return true
		}
	}
	return false
}

fn put_piece_effect_map(map: &mut Vec<Vec<&str>>, player: &Player, x: usize, y: usize, ptr: (i32, i32)){
	let (x_, y_) = ptr;
	let player_str = match player {
		Player::FIRST => FIRST_STR,
		Player::SECOND => SECOND_STR,
	};
	let another_player_str = match player {
		Player::FIRST => SECOND_STR,
		Player::SECOND => FIRST_STR,
	};
	let mut cur_x: i32 = x as i32 + x_;
	let mut cur_y: i32 = y as i32 + y_;
	let mut rep_ptrs: Vec<(usize, usize)> = Vec::new();
	let mut put_flg = false;
	while cur_x < SIZE && cur_x >= 0 && cur_y < SIZE && cur_y >= 0 {
		if map[cur_x as usize][cur_y as usize] == player_str {
			put_flg = true;
			break
		}else if map[cur_x as usize][cur_y as usize] == EMPTY_STR {
			break
		}else if map[cur_x as usize][cur_y as usize] == another_player_str {
			rep_ptrs.push((cur_x as usize, cur_y as usize));
		}
		cur_x += x_;
		cur_y += y_;
	}
	if put_flg {
		for ptr in rep_ptrs {
			map[ptr.0][ptr.1] = player_str;
		}
	}
}
// ピースの配置
fn put_piece(map: &mut Vec<Vec<&str>>, x: usize, y: usize, player: &Player) {
	let piece = match player {
		Player::FIRST => FIRST_STR,
		Player::SECOND => SECOND_STR
	};
	map[x][y] = piece;
	for i in -1i32..2 {
		for j in -1i32..2 {
			if !(i == 0 && j == 0) {
				put_piece_effect_map(map, player, x, y, (i, j))
			}
		}
	}
}
fn create_default_map() -> Vec<Vec<&'static str>> {
	let mut map: Vec<Vec<&str>> = Vec::new();
	for x in 0..SIZE {
		let mut tmp: Vec<&str> = Vec::new();
		for y in 0..SIZE {
			if x == (SIZE / 2 - 1) && y == (SIZE / 2 - 1) {
				tmp.push(FIRST_STR);
			}else if x == (SIZE / 2 - 1) && y == (SIZE / 2) {
				tmp.push(SECOND_STR);
			}else if x == (SIZE / 2) && y == (SIZE / 2 - 1){
				tmp.push(SECOND_STR);
			}else if x == (SIZE / 2) && y == (SIZE / 2){
				tmp.push(FIRST_STR);
			}else{
				tmp.push(EMPTY_STR);
			}
		}
		map.push(tmp);
	}
	return map
}
fn is_finish(map: &Vec<Vec<&str>>) -> bool {
	let e_cnt: usize = map.clone().into_iter()
		.map(|x: Vec<&str>| x.iter().filter(|y| **y == EMPTY_STR).count())
		.collect::<Vec<usize>>().into_iter().sum::<usize>();
	if e_cnt == 0 {
		return true
	}else{
		return false
	}
}
fn main() {
	// マップ
	let mut map: Vec<Vec<&str>> = create_default_map();
	// ターン変数
	let mut player: Player = Player::FIRST;
    loop{
		// 終了判定
		if is_finish(&map) {
			let f_cnt: usize = map.clone().into_iter()
				.map(|x: Vec<&str>| x.iter().filter(|y| **y == FIRST_STR).count())
				.collect::<Vec<usize>>().into_iter().sum::<usize>();
			let s_cnt: usize = map.clone().into_iter()
				.map(|x: Vec<&str>| x.iter().filter(|y| **y == SECOND_STR).count())
				.collect::<Vec<usize>>().into_iter().sum::<usize>();
			if f_cnt > s_cnt {
				println!("winner is first.");
				break
			}else if f_cnt == s_cnt {
				println!("draw.");
				break
			}else{
				println!("winner is second.");
				break
			}
		}
		// Skip判定
		if is_skip(&map, &player) {
			println!("skip.");
			continue
		}
		// map出力
		for lines in &map {
			for panel in lines {
				print!("{}", panel);
			}
			println!("");
		}
		// 座標読み込み
		println!("You can next input points is {:?}", pickup_points(&map, &player));
		print!("input: ");
		let mut s = String::new();
		std::io::stdin().read_line(&mut s).ok();
		let s_rep = s.replace("\n", "");
		let ptrs:Vec<&str> = s_rep.split(",").collect();
		if ptrs.len() != 2 {
			println!("Failed input.");
			continue
		}
		println!();
		// check
		if !check_put_piece(&map.to_vec(), ptrs[0].parse::<usize>().unwrap(),
				  ptrs[1].parse::<usize>().unwrap(), &player) {
			println!("Foul move.");
			continue
		}
		// マップ反映
		put_piece(&mut map, ptrs[0].parse::<usize>().unwrap(),
				  ptrs[1].parse::<usize>().unwrap(), &player);
		// ターン変更
		if player == Player::FIRST {
			player = Player::SECOND;
		}else {
			player = Player::FIRST;
		}
	}
}
#[cfg(test)]
mod tests {
	use crate::*;
	#[test]
	fn test_main(){
		let mut map = create_default_map();
		let mut player = Player::FIRST;
		loop {
			let ptrs = pickup_points(&map, &player);
			put_piece(&mut map, ptrs[0].0, ptrs[0].1, &player);
			player = match player {
				Player::FIRST => Player::SECOND,
				Player::SECOND => Player::FIRST,
			};
			if is_finish(&map) {
				break
			}
		}
	}
	#[test]
	fn test_put_piece(){
		let mut map = create_default_map();
		put_piece(&mut map, 2, 4, &Player::FIRST);
		assert_eq!(true, map[3 as usize][4 as usize] == FIRST_STR);
	}
    #[test]
    fn test_check_put_piece() {
		let map = create_default_map();
		// ピースが裏返らない手は打てない。
		assert_eq!(false, check_put_piece(&map, 0, 0, &Player::FIRST));
		// すでに置いてある場所には置けない。
		assert_eq!(false, check_put_piece(&map, 3, 3, &Player::FIRST));
    }
	#[test]
	fn test_is_skip(){
		let mut map = create_default_map();
		// 初期パターン。当然false。
		assert_eq!(false, is_skip(&map, &Player::SECOND));
		assert_eq!(false, is_skip(&map, &Player::FIRST));
		// 全てのパターンが黒になった時。
		put_piece(&mut map, 3, 4, &Player::FIRST);
		put_piece(&mut map, 4, 3, &Player::FIRST);
		assert_eq!(true, is_skip(&map, &Player::FIRST));
	}
	#[test]
	fn test_is_finish(){
		let mut map = Vec::new();
		for _ in 0..SIZE {
			let mut tmp: Vec<&str> = Vec::new();
			for _ in 0..SIZE {
				tmp.push(FIRST_STR);
			}
			map.push(tmp);
		}
		// 全てにコマが配置してあるのでtrue。
		assert_eq!(true, is_finish(&map));
		// 開始時点では当然false。
		assert_eq!(false, is_finish(&create_default_map()));
	}
	#[test]
	fn test_pickup_points(){
		let map = create_default_map();
		let expect = [(2, 3), (3, 2), (4, 5), (5, 4)].to_vec();
		assert_eq!(expect, pickup_points(&map, &Player::SECOND));
		let expect2 = [(2, 4), (3, 5), (4, 2), (5, 3)].to_vec();
		assert_eq!(expect2, pickup_points(&map, &Player::FIRST));
	}
	#[test]
	fn test_check_straight_line_same_piece_exist(){
		let map = create_default_map();
		let mut map2 = map.clone();
		put_piece(&mut map2, 3, 2, &Player::SECOND);
		put_piece(&mut map2, 4, 5, &Player::SECOND);

		let result1 = check_straight_line_same_piece_exist(&map, &Player::SECOND, 3, 2, (0, 1));
		let result2 = check_straight_line_same_piece_exist(&map, &Player::SECOND, 4, 5, (0, -1));
		let result3 = check_straight_line_same_piece_exist(&map, &Player::SECOND, 0, 0, (0, -1));

		assert_eq!(true, result1);
		assert_eq!(true, result2);
		assert_eq!(false, result3);
	}
}
