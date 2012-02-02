use std;
import std::io::println;
import float::{fmin,fmax,sqrt,acos};


type num   = float;

type vec2 = { x : num, y : num };

fn new(xnew : num, ynew : num) -> vec2 {
  ret { x : xnew, y : ynew };
}

fn abs(n : num) -> num { if n < 0.0 { -n } else { n } }




iface vec<T> {  
  fn add(v: T) -> T;
  fn sub(v: T) -> T;
}

const NEAR_DELTA  : float = 0.001;
fn zero() -> vec2 { { x: 0.0, y : 0.0 } }

impl of vec<vec2> for vec2 {
  fn add(v : vec2)      -> vec2 { new(self.x + v.x, self.y + v.y)    }
  fn sub(v : vec2)      -> vec2 { new(self.x - v.x, self.y - v.y)    }
  fn equal(v : vec2)    -> bool { (self.x == v.x) && (self.y == v.y) }
  fn neg()              -> vec2 { new(-self.x, -self.y)              }
  fn mult(n : num)      -> vec2 { new(self.x * n, self.y * n)        }
  fn div(n : num)       -> vec2 { new(self.x / n, self.y / n)        }
  fn dot(v: vec2)       -> num  { self.x*v.x + self.y*v.y            }
  fn cross(v: vec2)     -> num  { self.x*v.y - self.y*v.x            }
  fn perp()             -> vec2 { new(-self.y,  self.x)              }
  fn rperp()            -> vec2 { new( self.y, -self.x)             }
  fn project(v : vec2)  -> vec2 { 
    v.mult((self.dot(v))/(v.dot(v)))       
  }
  fn rotate(v : vec2) -> vec2  { 
    new(self.x*v.x - self.y*v.y, self.x*v.y + self.y*v.x)
  }
  fn unrotate(v : vec2) -> vec2 { 
    new(self.x*v.x + self.y*v.y, self.x*v.y - self.y*v.x)
  }

  fn lengthsq()            -> num   { self.dot(self)                         }
  fn length()              -> num   { sqrt(self.dot(self))                   }
  fn lerp(v: vec2, t : num)-> vec2  { self.mult(1.0-t).add(v.mult(t))        }
  fn iszero()              -> bool  { (self.x == 0.0) && (self.y == 0.0)     }
  fn normalize()          -> vec2 { self.div(self.length())                  }
  fn normalize_safe()     -> vec2 { 
    if self.iszero() { zero() } else { self.normalize()                      }
  }
  
  fn clamp(len : num)     -> vec2 { 
    if self.lengthsq() > (len*len) { self.normalize().mult(len) } else { self }
  }

  fn xnear(x : num)       -> bool  { abs(self.x - x) < NEAR_DELTA             }
  fn ynear(y : num)       -> bool  { abs(self.y - y) < NEAR_DELTA             }
  fn xynear(x: num, y : num)  -> bool  {
    self.xnear(x) && self.ynear(y)
  }
}


/*
 *
 * 

/// Linearly interpolate between self towards v by distance d.
static inline cpVect cpvlerpconst(cpVect self, cpVect v, cpFloat d)
{
  return cpvadd(self, cpvclamp(cpvsub(v, self), d));
}

/// Returns the distance between self and v.
static inline cpFloat cpvdist(const cpVect self, const cpVect v)
{
  return cpvlength(cpvsub(self, v));
}

/// Returns the squared distance between self and v. Faster than cpvdist() when you only need to compare distances.
static inline cpFloat cpvdistsq(const cpVect self, const cpVect v)
{
  return cpvlengthsq(cpvsub(self, v));
}

/// Returns true if the distance between self and v is less than dist.
static inline cpBool cpvnear(const cpVect self, const cpVect v, const cpFloat dist)
{
  return cpvdistsq(self, v) < dist*dist;
}
 
inline cpVect
cpvslerp(const cpVect self, const cpVect v, const cpFloat t)
{
  cpFloat omega = cpfacos(cpvdot(self, v));

  if(omega){
    cpFloat denom = 1.0f/cpfsin(omega);
    return cpvadd(cpvmult(self, cpfsin((1.0f - t)*omega)*denom), cpvmult(v, cpfsin(t*omega)*denom));
  } else {
    return self;
  }
}

cpVect
cpvslerpconst(const cpVect self, const cpVect v, const cpFloat a)
{
  cpFloat angle = cpfacos(cpvdot(self, v));
  return cpvslerp(self, v, cpfmin(a, angle)/angle);
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
fn test_vec2() {
  let self = new(1.0, 2.0);
  let v = new(4.0, 7.0);  
  let v3 = self.add(v);
  let v4 = v.sub(self);
  let v5 = new(0.0, 3.0);
  assert(self.x == 1.0);
  assert(self.y == 2.0);
  assert(v3.x == 5.0);  
  assert(v3.y == 9.0);
  assert(v3.xnear(5.0));
  assert(v3.ynear(9.0));
  assert(v4.x == 3.0);
  assert(v4.y == 5.0);
  assert(v5.length() == 3.0);
}





