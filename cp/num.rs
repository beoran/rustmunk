/* Use a num type in case someone wants to use different sized floating
* point numerals
*/

type num   = float;
type nonum = ();

const INFINITY : num = 1.0 / 0.0;
const PI       : num = 3.14159265358979323846;

enum optionalnum {
  num,
  nonum
}  

// Used for resizing hash tables. Values approximately double.
// http://planetmath.org/encyclopedia/GoodHashTablePrimes.html
fn next_prime(n : int) -> int {
  let primes_ = [
  5,
  13,
  23,
  47,
  97,
  193,
  389,
  769,
  1543,
  3079,
  6151,
  12289,
  24593,
  49157,
  98317,
  196613,
  393241,
  786433,
  1572869,
  3145739,
  6291469,
  12582917,
  25165843,
  50331653,
  100663319,
  201326611,
  402653189,
  805306457,
  1610612741,
  0
  ];
  let i = 0;
  while n > primes_[i] {
    i += 1;    
  }
  ret primes_[i];
}


