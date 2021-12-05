pub mod lib;
pub mod lib2;

use lib2::MyTrait;

pub trait Visible: MyTrait {
    fn f2(&self, s: &Self);
}
