use std::cmp;
use std::io;
use std::collections::HashMap;

static mut triangle: [[i32;10];10] = [[0;10];10];
static mut height: usize = 0;

//높이가 n인 lower triangular matrix가 입력되었을 때,
//좌상에서 시작하여 아래쪽이나 그 오른쪽으로만 이동할 수 있다면
//바닥에 닿았을 때의 최대합 구하기
fn main() {
  unsafe{
    //입력받기
    let mut input: String = String::new();
    println!("삼각형의 높이:");
    io::stdin().read_line(&mut input).expect("input error");
    height = input.trim().parse().unwrap();
    for i in 0..height {
      println!("{}층의 숫자 {}개 입력:",i,i+1);
      input = String::from("");
      io::stdin().read_line(&mut input).expect("input error");
      let mut split = input.split_whitespace();
      for j in 0..i+1 {
        triangle[i][j] = split.next().unwrap().parse().unwrap();
      }
    }
    //삼각형 출력
    println!("삼각형:");
    for i in 0..height {
      for j in 0..i+1 {
        print!("{} ",triangle[i][j]);
      }
      println!();
    }
    //최장경로 출력
    //cache.get((y,x)) = y,x에서부터 끝까지의 최대합 
    let mut cache: HashMap<(usize,usize),i32> = HashMap::new();
    println!("최장경로: {}",pathSum3(0,0,&mut cache));
  }
}

//현재 위치가 (y,x)이고 현재까지의 합이 sum일 때, 최대 합 구하기
unsafe fn pathSum(y:usize, x:usize, sum:i32) -> i32 {
  if y==height-1 {return sum+triangle[y][x];}
  cmp::max(pathSum(y+1,x,sum+triangle[y][x]),pathSum(y+1,x+1,sum+triangle[y][x]))
}
//현재 위치가 (y,x)일 때 남은 길 중 최대 합 구하기 (최적 부분 구조)
unsafe fn pathSum2(y:usize, x:usize) -> i32 {
  if y==height-1 {return triangle[y][x];}
  triangle[y][x]+cmp::max(pathSum2(y+1,x),pathSum2(y+1,x+1))
}
//pathSum2를 memoization한 것
unsafe fn pathSum3(y:usize, x:usize, cache: &mut HashMap<(usize,usize),i32>) -> i32 {
  if y==height-1 {return triangle[y][x];}
  //memoization
  let max: i32 = match cache.get(&(y,x)){
    Some(_) => 0,
    None => cmp::max(pathSum3(y+1,x,cache),pathSum3(y+1,x+1,cache)),
  };
  *cache.entry((y,x)).or_insert(triangle[y][x]+max)
}