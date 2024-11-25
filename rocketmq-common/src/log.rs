/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::str::FromStr;

use tracing::level_filters::LevelFilter;

/// Initializes the logger with the specified configuration.
///
/// This function sets up the logger using the `tracing_subscriber` crate.
/// It reads the log level from the `RUST_LOG` environment variable, defaulting to "INFO" if not
/// set. The logger is configured to include thread names, log levels, line numbers, and thread IDs
/// in the log output.
pub fn init_logger() {
    let info_level = std::env::var("RUST_LOG").unwrap_or(String::from("INFO"));
    tracing_subscriber::fmt()
        .with_thread_names(true)
        .with_level(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_max_level(LevelFilter::from_str(info_level.as_str()).unwrap())
        .init();
}
