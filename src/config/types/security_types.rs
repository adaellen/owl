use sha2::Sha512;
use generic_array;
use digest::BlockInput;

pub type ApplicationHasher = Sha512;
pub type HashSize = <ApplicationHasher as BlockInput>::BlockSize;
pub type HashResult = generic_array::GenericArray<u8, HashSize>;