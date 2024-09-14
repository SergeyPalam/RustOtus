use rand::prelude::{thread_rng, Rng};
use rand_distr::StandardNormal;
use log::*;

const AVG_POWER: f64 = 4_000.0; // 4 kW
const POWER_SPREAD: f64 = 100.0; // 100W

#[derive(Default)]
pub struct SmartSocket {
    is_turn_on: bool,
}

impl SmartSocket {
    pub fn turn_on(&mut self) {
        info!("Socket is turned on");
        self.is_turn_on = true;
    }

    pub fn turn_off(&mut self) {
        info!("Socket is turned off");
        self.is_turn_on = false;
    }

    pub fn get_power(&mut self) -> f64 {
        if !self.is_turn_on {
            return 0.0;
        }

        let noize = thread_rng().sample::<f64, StandardNormal>(StandardNormal) - 0.5;
        let scalied_noize = noize * POWER_SPREAD as f64;
        let res = AVG_POWER + scalied_noize;
        res
    }
}