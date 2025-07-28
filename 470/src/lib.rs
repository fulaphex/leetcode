/** 
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        loop {
            let (x, y) = (rand7()-1, rand7()-1);
            let out = x*7 + y;
            if out < 40 {
                return (out % 10)+1;
            }
        }
    }
}
