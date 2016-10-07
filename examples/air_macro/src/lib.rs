#![feature(plugin, custom_derive)]
#![plugin(mockers_macros)]

#[cfg(test)] extern crate mockers;
#[cfg(test)] mod tests;

#[derive(Mock)]
pub trait AirConditioner {
    fn make_hotter(&mut self, by: i16);
    fn make_cooler(&mut self, by: i16);
    fn get_temperature(&self) -> i16;
}

pub fn set_temperature_20(cond: &mut AirConditioner) {
    let t = cond.get_temperature();
    if t < 20 {
        cond.make_hotter(20 - t);
    } else {
        cond.make_cooler(t - 20);
    }
}