#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(improper_ctypes)]

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate libc;

#[macro_use]
extern crate rustful;

#[macro_use]
extern crate nickel;


use nickel::{
	Nickel, JsonBody, 
	HttpRouter, Request, 
	Response, MiddlewareResult, 
	MediaType
};

use libc::{c_char, uint32_t};
use std::str;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
// use rustc_serialize::json;

use mongodb::{ThreadedClient, Client};
use mongodb::db::options::CreateCollectionOptions;
use mongodb::db::{ThreadedDatabase, Database};
use mongodb::coll::Collection;
use rustful::{Server, TreeRouter};
use bson::Bson;
use bson::oid::ObjectId;
use serde_json::{Value, Error};


fn main() {
	init_server();
}

fn mongodb_conn(collection: &str) -> mongodb::coll::Collection {
	let client = Client::connect("localhost", 27017)
		      		.ok().expect("Error establishing connection.");
	client.db("test").collection(collection)
}

fn build_json_response(cursor: mongodb::cursor::Cursor) -> String {
	// Opening for the JSON string to be returned
	let mut data_result = "{\"data\":[".to_owned();

	for (_i, result) in cursor.enumerate() {

		let rec = Bson::Document(result.unwrap());
		let serialized = serde_json::to_string(&rec).unwrap();

		data_result.push_str(&serialized);
		data_result.push_str(",");

	};
	data_result.pop();
	data_result.push_str("]}");
	data_result = str::replace(&data_result, "\\", "");

	// Return JSON
	format!("{}", data_result)
}

fn init_server() {
	let mut server = Nickel::new();
	
	server.utilize(
		router!{
			/* GET http://localhost:3002/bs/MATH */
			get "/bs/:SUBJECT" => |request, mut response| {
				response.set(MediaType::Json);

		      	build_json_response(
		      		mongodb_conn("bs").find(
		      			Some(doc!{ 
		      				"SUBJECT" => request.param("SUBJECT").unwrap() 
		      			}), 
		      			None
		      		).unwrap()
		      	)
			}

			/* GET http://localhost:3002/bs */
			get "/bs" => |_request, mut response| {
		      	response.set(MediaType::Json);

		      	build_json_response(
		      		mongodb_conn("bs").find(None, None).unwrap()
		      	)
			}

			/* GET http://localhost:3002/courses */
			get "/courses" => |_request, mut response| {
		    	response.set(MediaType::Json);

		    	build_json_response(
		    		mongodb_conn("courses").find(None, None).unwrap()
		    	)
			}
			/* GET http://localhost:3002/courses/BIOE */
			get "/courses/:SUBJECT" => |request, mut response| {
				response.set(MediaType::Json);

		      	build_json_response(
		      		mongodb_conn("courses").find(
		      			Some(doc!{ 
		      				"SUBJECT" => request.param("SUBJECT").unwrap() 
		      			}), 
		      			None
		      		).unwrap()
		      	)
			}

			get "/" => |_request, response| {
				"hello_world"
			}
		}
	);

	/* Listen on port 3002 */
	match server.listen("127.0.0.1:3002") {
		Ok(_a) => {
			println!("Server connected ok!")
		},
		Err(e) => println!("Error with server: {:?}", e)
	}
}









