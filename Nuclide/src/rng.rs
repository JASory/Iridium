/*
   PRNG for decay chain 
*/


// Possible improvements
// The higher bits of x are going to be identical in most cases so some copying/XOR-ing of 
// lower bits into higher positions could improve entropy
#[inline(always)]
fn default_xor() -> u64{
   let mut x = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64;
   
    x ^= x.wrapping_shr(12);
    x ^= x.wrapping_shl(25);
    x ^= x.wrapping_shr(27);
    x.wrapping_mul(0x2545F4914F6CDD1D)
}

#[allow(unreachable_code)]
pub(crate) fn rand() -> u64 {

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("rdrand"){// USE RDRAND if possible
            let mut x: u64 = 0;
            unsafe { core::arch::x86_64::_rdrand64_step(&mut x) };
            return x;
        }// If processor is x86 and does not support RDRAND use xor shift
       return default_xor()
    }

    {// All other architectures use xor shift
        default_xor()
    }
}
