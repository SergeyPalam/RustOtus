use rand::prelude::{thread_rng, Rng};
use rand_distr::StandardNormal;
use log::*;

const AVG_TEMP: f64 = 25.0; // C
const TEMP_SPREAD: f64 = 5.0; // 100W

#[derive(Default)]
pub struct SmartTherm {
    is_turn_on: bool,
}

impl SmartTherm {
    pub fn turn_on(&mut self) {
        info!("Therm is turned on");
        self.is_turn_on = true;
    }

    pub fn turn_off(&mut self) {
        info!("Therm is turned off");
        self.is_turn_on = false;
    }

    pub fn get_temperature(&mut self) -> f64 {
        if !self.is_turn_on {
            return 0.0;
        }

        let noize = thread_rng().sample::<f64, StandardNormal>(StandardNormal) - 0.5;
        let scalied_noize = noize * TEMP_SPREAD as f64;
        let res = AVG_TEMP + scalied_noize;
        res
    }
}