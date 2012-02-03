/* Use a num type in case someone wants to use different sized floating
* point numerals
*/

type num   = float;
type nonum = ();

const INFINITY : num = 1.0 / 0.0;

enum optionalnum {
  num,
  nonum
}  


