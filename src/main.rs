// 空白文字
const EMPTY_STR: &str = ".";
// 先手文字
const FIRST_STR: &str = "#";
// 後手文字
const SECOND_STR: &str = "*";
// サイズ
const SIZE: i32 = 8;
// ターン
#[derive(PartialEq)]
enum Player {
	FIRST,
	SECOND
}
fn pickup_points(map: &Vec<Vec<&str>>, player: &Player) -> Vec<(i32, i32)> {

	return [].to_vec()
}
fn check_put_piece(map: &Vec<Vec<&str>>, x: usize, y: usize, player: &Player) -> bool {
	// 打てる手を全て算出。
	let ptrs = pickup_points(&map, player);
	// 算出した手以外にピースを置いた場合、false。
	return false
}
// ピースの配置
fn put_piece(map: &mut Vec<Vec<&str>>, x: usize, y: usize, player: &Player) {
	// check
	if !check_put_piece(&map.to_vec(), x, y, player) {
		println!("Foul move.");
		return
	}
	let piece = match player {
		Player::FIRST => FIRST_STR,
		Player::SECOND => SECOND_STR
	};
	map[x][y] = piece;
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
fn is_skip(map: &Vec<Vec<&str>>, player: &Player) -> bool {
	return false
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
			}else if f_cnt == s_cnt {
				println!("draw.");
			}else{
				println!("winner is second.");
			}
		}
		// Skip判定
		if is_skip(&map, &player) {
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
		print!("input: ");
		let mut s = String::new();
		std::io::stdin().read_line(&mut s).ok();
		let s_rep = s.replace("\n", "");
		let ptrs:Vec<&str> = s_rep.split(",").collect();
		if ptrs.len() != 2 {
			println!("Failed input.");
			break;
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
    fn test_check_put_piece() {
		let map = create_default_map();
		// ピースが裏返らない手は打てない。
		assert_eq!(false, check_put_piece(&map, 0, 0, &Player::FIRST));
		// すでに置いてある場所には置けない。
		assert_eq!(true, check_put_piece(&map, 3, 3, &Player::FIRST));
    }
	#[test]
	fn test_is_skip(){
		let mut map = create_default_map();
		// 初期パターン。当然false。
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
		let expect = [(3, 2), (2, 3), (4, 5), (5, 4)].to_vec();
		assert_eq!(expect, pickup_points(&map, &Player::FIRST));
	}
}
