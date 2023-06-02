/*
 * CXL FM Infrastructure -- CXl Fabric Manager (FM) Infrastructure.
 *
 * CXL FM configuration tool implementation.
 *
 * Copyright (c) 2023 Viacheslav Dubeyko <slava@dubeyko.com>,
 * All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub mod cxl_switch_command {
	use clap::{ArgMatches};
	use std::net::{TcpStream};
	use fm_library::cxl_fm_lib::CxlFmOptions;
	use fm_library::cxl_fm_lib::send_command;
	use fm_library::cxl_fm_lib::CXL_FM_GET_SWITCH_INFO_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_GET_SWITCH_CONFIG_COMMAND;
	use fm_library::cxl_fm_lib::CXL_FM_SET_SWITCH_CONFIG_COMMAND;

	/*
	 * Get CXL switch status/info
	 */
	pub fn get_info(options: &ArgMatches, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", crate::CXL_FM_SWITCH_GET_INFO_COMMAND_DESCRIPTOR);
		}

		if options.args_present() {
			/*
			 * Ignore currently.
			 * Add code later.
			 */
		}

		match TcpStream::connect(&env.ip_port) {
			Ok(mut stream) => {
				if env.is_debug {
					println!("Successfully connected to server: {}",
						 env.ip_port);
				}

				send_command(&mut stream,
					     CXL_FM_GET_SWITCH_INFO_COMMAND,
					     &env);
			},
			Err(e) => {
				println!("Failed to connect: {}", e);
			}
		}
	}

	/*
	 * Get CXL switch configuration
	 */
	pub fn get_config(options: &ArgMatches, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", crate::CXL_FM_SWITCH_GET_CONFIG_COMMAND_DESCRIPTOR);
		}

		if options.args_present() {
			/*
			 * Ignore currently.
			 * Add code later.
			 */
		}

		match TcpStream::connect(&env.ip_port) {
			Ok(mut stream) => {
				if env.is_debug {
					println!("Successfully connected to server: {}",
						 env.ip_port);
				}

				send_command(&mut stream,
					     CXL_FM_GET_SWITCH_CONFIG_COMMAND,
					     &env);
			},
			Err(e) => {
				println!("Failed to connect: {}", e);
			}
		}
	}

	/*
	 * Set CXL switch configuration
	 */
	pub fn set_config(options: &ArgMatches, env: &CxlFmOptions) {
		if env.is_debug {
			println!("{}", crate::CXL_FM_SWITCH_SET_CONFIG_COMMAND_DESCRIPTOR);
		}

		if options.args_present() {
			/*
			 * Ignore currently.
			 * Add code later.
			 */
		}

		match TcpStream::connect(&env.ip_port) {
			Ok(mut stream) => {
				if env.is_debug {
					println!("Successfully connected to server: {}",
						 env.ip_port);
				}

				send_command(&mut stream,
					     CXL_FM_SET_SWITCH_CONFIG_COMMAND,
					     &env);
			},
			Err(e) => {
				println!("Failed to connect: {}", e);
			}
		}
	}
}
