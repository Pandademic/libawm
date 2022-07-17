// libawm::user_config manages the user facing script that configures awm as a whole.
// as of 7/17/22, it is just a wireframe of the A.P.I 
// the eventual game plan is to let it be configured in a xyz embedable scripting langauge, such as:
// - wren ( via ruwren<https://github.com/Jengamon/ruwren>)
// - lua ( via the rlua crate)
// - or gravity ( via the gravity-rs<https://github.com/bruflot/gravity-rs crate)
// they all have their strengths and weaknesses 
// this is still all under debate, so plans can change at any moment.

                                                                           
pub fn gaps() -> i32 {
  return 5; 
}

pub fn layout -> &'static str {
  return "side stack";  
}
