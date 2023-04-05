#![feature(daisogen_api)]

fn main() {
    loop {
        std::daisogen::pd_call0("kbd_get_char");
    }

    //std::daisogen::yld();
}
