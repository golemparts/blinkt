// Copyright (c) 2016 Rene van der Meer
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! Identifies the Raspberry Pi model based on `/proc/cpuinfo`.

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::result;

const BCM2708_PERIPHERAL_BASE: u32 = 0x20000000;
const BCM2709_PERIPHERAL_BASE: u32 = 0x3f000000;
const GPIO_OFFSET: u32 = 0x200000;

quick_error! {
    #[derive(Debug)]
/// Errors that can occur when trying to identify the Raspberry Pi model.
    pub enum Error {
        UnknownSoC { description("unknown BCM SoC") }
        UnknownModel { description("unknown Raspberry Pi model") }
        CantAccessProcCPUInfo { description("can't access /proc/cpuinfo") }
    }
}

/// Result type returned from methods that can have `system::Error`s.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Model {
    RaspberryPiA,
    RaspberryPiAPlus,
    RaspberryPiB,
    RaspberryPiBPlus,
    RaspberryPi2B,
    RaspberryPi3B,
    RaspberryPiCompute,
    RaspberryPiZero,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum SoC {
    BCM2835,
    BCM2836,
    BCM2837,
}

pub struct System {
    pub model: Model,
    pub soc: SoC,
    pub peripheral_base: u32,
    pub gpio_offset: u32,
}

impl System {
    pub fn new() -> Result<System> {
        // Parse hardware/revision from /proc/cpuinfo to figure out model/SoC
        let proc_cpuinfo = BufReader::new(match File::open("/proc/cpuinfo") {
            Err(_) => return Err(Error::CantAccessProcCPUInfo),
            Ok(file) => file,
        });

        let mut hardware: String = String::new();
        let mut revision: String = String::new();
        for line_result in proc_cpuinfo.lines() {
            if let Some(line) = line_result.ok() {
                if line.starts_with("Hardware\t: ") {
                    hardware = String::from(&line[11..]);
                } else if line.starts_with("Revision\t: ") {
                    revision = String::from(&line[11..]);
                }
            }
        }

        // Make sure we're actually running on a compatible BCM SoC
        match &hardware[..] {
            "BCM2708" | "BCM2709" | "BCM2835" | "BCM2836" | "BCM2837" => {}
            _ => return Err(Error::UnknownSoC),
        }

        let mut model = Model::Unknown;

        // Newer revisions consist of at least 6 characters
        if revision.len() >= 6 {
            model = match &revision[revision.len() - 3..revision.len() - 1] {
                "00" => Model::RaspberryPiA,
                "01" => Model::RaspberryPiB,
                "02" => Model::RaspberryPiAPlus,
                "03" => Model::RaspberryPiBPlus,
                "04" => Model::RaspberryPi2B,
                "06" => Model::RaspberryPiCompute,
                "08" => Model::RaspberryPi3B,
                "09" => Model::RaspberryPiZero,
                _ => Model::Unknown,
            };
        } else if revision.len() == 4 {
            // Older revisions are 4 characters long
            model = match &revision[..] {
                "0007" | "0008" | "0009" => Model::RaspberryPiA,
                "0002" | "0003" | "0004" | "0005" | "0006" | "000d" | "000e" | "000f" => {
                    Model::RaspberryPiB
                }
                "0012" => Model::RaspberryPiAPlus,
                "0010" | "0013" => Model::RaspberryPiBPlus,
                "0011" => Model::RaspberryPiCompute,
                _ => Model::Unknown,
            };
        }

        match model {
            Model::RaspberryPiA |
            Model::RaspberryPiAPlus |
            Model::RaspberryPiB |
            Model::RaspberryPiBPlus |
            Model::RaspberryPiCompute |
            Model::RaspberryPiZero => {
                Ok(System {
                    model: model,
                    soc: SoC::BCM2835,
                    peripheral_base: BCM2708_PERIPHERAL_BASE,
                    gpio_offset: GPIO_OFFSET,
                })
            }
            Model::RaspberryPi2B => {
                Ok(System {
                    model: model,
                    soc: SoC::BCM2836,
                    peripheral_base: BCM2709_PERIPHERAL_BASE,
                    gpio_offset: GPIO_OFFSET,
                })
            }
            Model::RaspberryPi3B => {
                Ok(System {
                    model: model,
                    soc: SoC::BCM2837,
                    peripheral_base: BCM2709_PERIPHERAL_BASE,
                    gpio_offset: GPIO_OFFSET,
                })
            }
            _ => Err(Error::UnknownModel),
        }
    }
}
