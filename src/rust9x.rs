use crate::Error;
use core::mem::MaybeUninit;

pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    for i in dest {
        i.write(fastrand::u8(..));
    }

    Ok(())
}
