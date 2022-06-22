
mod front_of_house {
  pub mod hosting {
    pub fn add(){
      println!("联系");
      super::super::eat_at();
    } 
  }
}

use front_of_house::hosting::add;
pub fn eat_at() {
  crate::front_of_house::hosting::add(); // 绝对路径
  front_of_house::hosting::add(); // 相对路径
  add();
}
