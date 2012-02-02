use std;
import std::io::println;
import float::{fmin,fmax,sqrt,acos};


type num   = float;

type vec2d = { x : num, y : num };

fn new(xnew : num, ynew : num) -> vec2d {
  ret { x : xnew, y : ynew };
}

fn abs(n : num) -> num { if n < 0.0 { -n } else { n } }




iface vec<T> {  
  fn add(that: T) -> T;
  fn sub(that: T) -> T;
}

const NEAR_DELTA : float = 0.001;

impl of vec<vec2d> for vec2d {
  fn add(that : vec2d)    -> vec2d { new(self.x + that.x, self.y + that.y)    }
  fn sub(that : vec2d)    -> vec2d { new(self.x - that.x, self.y - that.y)    }
  fn dot(that: vec2d)     -> num   { self.x*that.x + self.y*that.y            }
  fn length()             -> num   { sqrt(self.dot(self))                     }
  fn equal(that : vec2d)  -> bool  { (self.x == that.x) && (self.y == that.y) }
  fn neg()                -> vec2d { new(-self.x, -self.y)                    }
  fn xnear(x : num)       -> bool  { abs(self.x - x) < NEAR_DELTA             }
  fn ynear(y : num)       -> bool  { abs(self.y - y) < NEAR_DELTA             }
  fn xynear(x: num, y : num)  -> bool  {
    self.xnear(x) && self.ynear(y)
  }
}


/*
 *
 * 

/// Negate a vector.
static inline cpVect cpvneg(const cpVect v)
{
  return cpv(-v.x, -v.y);
}

/// Scalar multiplication.
static inline cpVect cpvmult(const cpVect v, const cpFloat s)
{
  return cpv(v.x*s, v.y*s);
}

/// Vector dot product.
static inline cpFloat cpvdot(const cpVect v1, const cpVect v2)
{
  return v1.x*v2.x + v1.y*v2.y;
}

/// 2D vector cross product analog.
/// The cross product of 2D vectors results in a 3D vector with only a z component.
/// This function returns the magnitude of the z value.
static inline cpFloat cpvcross(const cpVect v1, const cpVect v2)
{
  return v1.x*v2.y - v1.y*v2.x;
}

/// Returns a perpendicular vector. (90 degree rotation)
static inline cpVect cpvperp(const cpVect v)
{
  return cpv(-v.y, v.x);
}

/// Returns a perpendicular vector. (-90 degree rotation)
static inline cpVect cpvrperp(const cpVect v)
{
  return cpv(v.y, -v.x);
}

/// Returns the vector projection of v1 onto v2.
static inline cpVect cpvproject(const cpVect v1, const cpVect v2)
{
  return cpvmult(v2, cpvdot(v1, v2)/cpvdot(v2, v2));
}

/// Uses complex number multiplication to rotate v1 by v2. Scaling will occur if v1 is not a unit vector.
static inline cpVect cpvrotate(const cpVect v1, const cpVect v2)
{
  return cpv(v1.x*v2.x - v1.y*v2.y, v1.x*v2.y + v1.y*v2.x);
}

/// Inverse of cpvrotate().
static inline cpVect cpvunrotate(const cpVect v1, const cpVect v2)
{
  return cpv(v1.x*v2.x + v1.y*v2.y, v1.y*v2.x - v1.x*v2.y);
}

/// Returns the squared length of v. Faster than cpvlength() when you only need to compare lengths.
static inline cpFloat cpvlengthsq(const cpVect v)
{
  return cpvdot(v, v);
}

/// Linearly interpolate between v1 and v2.
static inline cpVect cpvlerp(const cpVect v1, const cpVect v2, const cpFloat t)
{
  return cpvadd(cpvmult(v1, 1.0f - t), cpvmult(v2, t));
}

/// Returns a normalized copy of v.
static inline cpVect cpvnormalize(const cpVect v)
{
  return cpvmult(v, 1.0f/cpvlength(v));
}

/// Returns a normalized copy of v or cpvzero if v was already cpvzero. Protects against divide by zero errors.
static inline cpVect cpvnormalize_safe(const cpVect v)
{
  return (v.x == 0.0f && v.y == 0.0f ? cpvzero : cpvnormalize(v));
}

/// Clamp v to length len.
static inline cpVect cpvclamp(const cpVect v, const cpFloat len)
{
  return (cpvdot(v,v) > len*len) ? cpvmult(cpvnormalize(v), len) : v;
}

/// Linearly interpolate between v1 towards v2 by distance d.
static inline cpVect cpvlerpconst(cpVect v1, cpVect v2, cpFloat d)
{
  return cpvadd(v1, cpvclamp(cpvsub(v2, v1), d));
}

/// Returns the distance between v1 and v2.
static inline cpFloat cpvdist(const cpVect v1, const cpVect v2)
{
  return cpvlength(cpvsub(v1, v2));
}

/// Returns the squared distance between v1 and v2. Faster than cpvdist() when you only need to compare distances.
static inline cpFloat cpvdistsq(const cpVect v1, const cpVect v2)
{
  return cpvlengthsq(cpvsub(v1, v2));
}

/// Returns true if the distance between v1 and v2 is less than dist.
static inline cpBool cpvnear(const cpVect v1, const cpVect v2, const cpFloat dist)
{
  return cpvdistsq(v1, v2) < dist*dist;
}
 
inline cpVect
cpvslerp(const cpVect v1, const cpVect v2, const cpFloat t)
{
  cpFloat omega = cpfacos(cpvdot(v1, v2));

  if(omega){
    cpFloat denom = 1.0f/cpfsin(omega);
    return cpvadd(cpvmult(v1, cpfsin((1.0f - t)*omega)*denom), cpvmult(v2, cpfsin(t*omega)*denom));
  } else {
    return v1;
  }
}

cpVect
cpvslerpconst(const cpVect v1, const cpVect v2, const cpFloat a)
{
  cpFloat angle = cpfacos(cpvdot(v1, v2));
  return cpvslerp(v1, v2, cpfmin(a, angle)/angle);
}

cpVect
cpvforangle(const cpFloat a)
{
  return cpv(cpfcos(a), cpfsin(a));
}

cpFloat
cpvtoangle(const cpVect v)
{
  return cpfatan2(v.y, v.x);
}

char*
cpvstr(const cpVect v)
{
  static char str[256];
  sprintf(str, "(% .3f, % .3f)", v.x, v.y);
  return str;
}


  
}
*/


#[test]
fn test_vec2d() {
  let v1 = new(1.0, 2.0);
  let v2 = new(4.0, 7.0);  
  let v3 = v1.add(v2);
  let v4 = v2.sub(v1);
  let v5 = new(0.0, 3.0);
  assert(v1.x == 1.0);
  assert(v1.y == 2.0);
  assert(v3.x == 5.0);  
  assert(v3.y == 9.0);
  assert(v3.xnear(5.0));
  assert(v3.ynear(9.0));
  assert(v4.x == 3.0);
  assert(v4.y == 5.0);
  assert(v5.length() == 3.0);
}





