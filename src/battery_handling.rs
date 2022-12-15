use std::{fs, thread, time::Duration};

use crate::config::Config;
use color_eyre::eyre::Result;
use thiserror::Error;

const CAPACITY_FILE_PATH: &str = "/sys/class/power_supply/macsmc-battery/capacity";
const CHARGE_BEHAVIOUR_FILE_PATH: &str = "/sys/class/power_supply/macsmc-battery/charge_behaviour";
const STATUS_FILE_PATH: &str = "/sys/class/power_supply/macsmc-battery/status";

#[derive(Error, Debug)]
pub enum BatteryError {
    #[error("unable to access battery information")]
    NotAccessible,
    #[error("unexpected value found in sys file, this is a bug")]
    UnexpectedValue,
}

struct BatteryState {
    capacity: i32,
    charge_behaviour: ChargeBehaviour,
    charging_status: ChargingStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ChargeBehaviour {
    Auto,
    InhibitCharge,
    ForceDischarge,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ChargingStatus {
    Charging,
    Discharging,
    NotCharging,
}

pub fn start_handler(config: Config) -> Result<(), BatteryError> {
    let mut battery = BatteryState::new()?;
    loop {
        battery.refresh()?;
        if battery.get_charging_status() == ChargingStatus::Charging
            && battery.get_capacity() >= config.get_stop_charging_threshold()
            && battery.get_charge_behaviour() != ChargeBehaviour::InhibitCharge
        {
            info!("Stopped charging");
            battery.set_charge_behaviour(ChargeBehaviour::InhibitCharge)?;
        } else if battery.get_capacity() <= config.get_start_charging_threshold()
            && battery.get_charge_behaviour() != ChargeBehaviour::Auto
        {
            info!("Started charging");
            battery.set_charge_behaviour(ChargeBehaviour::Auto)?;
        } else if battery.get_charging_status() == ChargingStatus::Discharging
            && battery.get_charge_behaviour() != ChargeBehaviour::Auto
        {
            info!(
                "Charger disconnected while preventing charging, setting charge behaviour to auto"
            );
            battery.set_charge_behaviour(ChargeBehaviour::Auto)?;
        }

        thread::sleep(Duration::from_secs(30));
    }
}

impl BatteryState {
    fn new() -> Result<Self, BatteryError> {
        let mut state = BatteryState {
            capacity: 0,
            charge_behaviour: ChargeBehaviour::Auto,
            charging_status: ChargingStatus::Discharging,
        };
        state.refresh()?;

        Ok(state)
    }

    fn get_capacity(&self) -> i32 {
        self.capacity
    }

    fn get_charge_behaviour(&self) -> ChargeBehaviour {
        self.charge_behaviour
    }

    fn get_charging_status(&self) -> ChargingStatus {
        self.charging_status
    }

    fn set_charge_behaviour(&mut self, behaviour: ChargeBehaviour) -> Result<(), BatteryError> {
        let behaviour_str = match behaviour {
            ChargeBehaviour::Auto => "auto",
            ChargeBehaviour::InhibitCharge => "inhibit-charge",
            ChargeBehaviour::ForceDischarge => "force-discharge",
        };
        fs::write(CHARGE_BEHAVIOUR_FILE_PATH, behaviour_str)
            .or(Err(BatteryError::NotAccessible))?;
        self.charge_behaviour = behaviour;

        Ok(())
    }

    fn refresh(&mut self) -> Result<(), BatteryError> {
        self.refresh_capacity()?;
        self.refresh_charge_behaviour()?;
        self.refresh_charging_status()?;

        Ok(())
    }

    fn refresh_capacity(&mut self) -> Result<(), BatteryError> {
        self.capacity = fs::read_to_string(CAPACITY_FILE_PATH)
            .or(Err(BatteryError::NotAccessible))?
            .trim_end()
            .parse::<i32>()
            .or(Err(BatteryError::NotAccessible))?;

        Ok(())
    }

    fn refresh_charge_behaviour(&mut self) -> Result<(), BatteryError> {
        let behaviour_str =
            fs::read_to_string(CHARGE_BEHAVIOUR_FILE_PATH).or(Err(BatteryError::NotAccessible))?;

        self.charge_behaviour = match behaviour_str.trim() {
            "auto" => ChargeBehaviour::Auto,
            "inhibit-charge" => ChargeBehaviour::InhibitCharge,
            "force-discharge" => ChargeBehaviour::ForceDischarge,
            _ => return Err(BatteryError::UnexpectedValue),
        };

        Ok(())
    }

    fn refresh_charging_status(&mut self) -> Result<(), BatteryError> {
        let status = fs::read_to_string(STATUS_FILE_PATH).or(Err(BatteryError::NotAccessible))?;
        self.charging_status = match status.trim() {
            "Charging" => ChargingStatus::Charging,
            "Discharging" => ChargingStatus::Discharging,
            "Not charging" => ChargingStatus::NotCharging,
            _ => return Err(BatteryError::UnexpectedValue),
        };

        Ok(())
    }
}
