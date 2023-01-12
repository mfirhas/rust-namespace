// ketika ada main.rs dan lib.rs di dalam src. Maka cargo akan mendeteksi main.rs sebagai binary crate untuk executable/entry program 
// dan lib.rs sebagai crate library untuk supporting library.
// main.rs menjadi crate utama sehingga module2 lain selain lib hanya bisa di-declare di dalam main.rs.
// lib.rs hanya bisa import hal-hal di dalam ranah dia.

use asd::lib1;
use asd::lib2;

// lib merupakan crate sekaligus module yang secara konvensi di-deteksi sebagai crate non-executable yang bisa dibaca dari level package `sample_simple`
pub mod lib;

// asd merupakan module tapi bukan crate. Module ini ketika suatu projek memiliki main.rs(binary) dan berada pada level yang sama, 
// maka akan berada dibawah crate main tersebut sehingga tidak bisa diimport oleh lib.rs.
pub mod asd;

pub fn from_main() {
    println!("im from main");
}

fn main() {
    lib::b(); // can import b function from lib declaration, or

    sample_simple::b(); // can import b function from package's name

    println!("{}", crate::lib::CONSTANT);
    crate::lib::anu::a();
    println!("{}", lib::CONSTANT);
    println!("{}", sample_simple::CONSTANT);
    // sample_simple::C:  ////! cannot use constant C from asd.rs this as asd.rs is not convention for library crate's file name
    // sample_simple::get() ////! cannot use function get() from asd.rs as asd.rs is not convention for library crate's file name
    println!("{}", asd::C);
    println!("{}", crate::asd::C);
    println!("Hello, world!");
}
