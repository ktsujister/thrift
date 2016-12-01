/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements. See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership. The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License. You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied. See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

#[macro_use]
extern crate terminal_thrift as thrift;
extern crate bufstream;

mod tutorial;
mod shared;

use std::net::TcpListener;
use std::cell::RefCell;
use std::collections::HashMap;

use thrift::protocol::binary_protocol::BinaryProtocol;
use thrift::server::SimpleServer;

use tutorial::*;
use shared::*;

use bufferserver::BufferServer;

mod bufferserver;

struct CalculatorHandler {
    log: RefCell<HashMap<i32, SharedStruct>>
}

impl<'a> Calculator for &'a CalculatorHandler {
    fn ping(&self) -> thrift::exception::Result<()> {
        println!("ping()");
        Ok(())
    }

    fn add(&self, n1: i32, n2: i32) -> thrift::exception::Result<i32> {
        println!("add({}, {})", n1, n2);
        Ok(n1 + n2)
    }

    fn calculate(&self, log_id: i32, work: Work) -> thrift::exception::Result<Result<i32, CalculatorCalculateError>> {
        println!("calculate({}, {:?})", log_id, work);

        let num1 = work.num1;
        let num2 = work.num2;

        let val = match work.op {
            Operation::ADD => num1 + num2,
            Operation::SUBTRACT => num1 - num2,
            Operation::MULTIPLY => num1 * num2,
            Operation::DIVIDE => {
                if num2 == 0 {
                    return Ok(Err(CalculatorCalculateError::Ouch(InvalidOperation {
                        what_op: work.op as i32,
                        why: "Cannot divide by 0".into()
                    })))
                }

                num1 / num2
            }
        };

        let ss = SharedStruct { key: log_id, value: val.to_string() };
        self.log.borrow_mut().insert(log_id, ss);

        Ok(Ok(val))
    }

    fn zip(&self) -> thrift::exception::Result<()> {
        println!("zip");
        Ok(())
    }
}

impl<'a> SharedService for &'a CalculatorHandler {
    fn getStruct(&self, log_id: i32) -> thrift::exception::Result<SharedStruct> {
        println!("getStruct({})", log_id);
        Ok(self.log.borrow()[&log_id].clone())
    }
}

pub fn main() {
    let handler = CalculatorHandler { log: RefCell::new(HashMap::new()) };
    let processor = CalculatorProcessor::new(&handler, &handler);

    let server_transport = BufferServer(TcpListener::bind("127.0.0.1:9090").unwrap());
    let mut server = SimpleServer::new(processor, server_transport, || BinaryProtocol);

    println!("Starting the server...");
    server.serve();
    println!("Done.");
}
