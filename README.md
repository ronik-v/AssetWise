# Trade Robot for Algorithmic Trading

[![Build Status](https://img.shields.io/travis/ronik-v/trade_robot/master.svg?style=flat-square)](https://travis-ci.org/ronik-v/trade_robot)
[![License](https://img.shields.io/github/license/ronik-v/trade_robot.svg?style=flat-square)](https://github.com/ronik-v/trade_robot/blob/master/LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.57.0-orange.svg?style=flat-square)](https://www.rust-lang.org/)
[![IDE](https://img.shields.io/badge/IDE-RustRover%202024.1-blue.svg?style=flat-square)](https://www.rust-lang.org/tools/install)

Welcome to Trade Robot, a console-based desktop application written in Rust! This robot utilizes ARIMA and SMA strategies (SMA5 / SMA12) for algorithmic trading.

## Features

- **ARIMA Strategy:** Implements ARIMA (AutoRegressive Integrated Moving Average) model for time-series prediction.
- **SMA5 / SMA12 Strategy:** Utilizes Simple Moving Average (SMA) with two different periods (5 and 12) for trend analysis.
- **Console-based Interface:** Designed for ease of use in a desktop environment.

## About

Trade Robot is developed by a master's degree student for research purposes in algorithmic trading. It aims to explore the effectiveness of ARIMA and SMA strategies in automated trading systems.

## Installation

To build and run Trade Robot, ensure you have Rust installed on your system. Then, clone this repository and use Cargo to build the project.

```bash
git clone https://github.com/ronik-v/trade_robot.git
cd trade_robot
cargo build --release
```

## Usage
After building the project, you can run the Trade Robot from the command line:
```bash
cargo run
```

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.